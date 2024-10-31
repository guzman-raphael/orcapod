use crate::{
    error::{Kind, OrcaError, Result},
    model::{from_yaml, to_yaml, Annotation, Pod},
    store::Store,
    util::get_type_name,
};
use colored::Colorize;
use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
/// Support for a storage backend on a local filesystem directory.
#[derive(Debug)]
pub struct LocalFileStore {
    /// A local path to a directory where store will be located.
    directory: PathBuf,
}

impl Store for LocalFileStore {
    fn save_pod(&self, pod: &Pod) -> Result<()> {
        self.save_model(pod, &pod.hash, &pod.annotation)
    }

    fn load_pod(&self, name: &str, version: &str) -> Result<Pod> {
        self.load_model(name, version)
    }

    fn list_pod(&self) -> Result<BTreeMap<String, Vec<String>>> {
        self.list_model::<Pod>()
    }

    fn delete_pod(&self, name: &str, version: &str) -> Result<()> {
        self.delete_model::<Pod>(name, version)
    }
}

impl LocalFileStore {
    /// Construct a local file store instance in a specific directory.
    pub fn new(directory: impl AsRef<Path>) -> Self {
        Self {
            directory: directory.as_ref().into(),
        }
    }
    /// Get the directory where store is located.
    pub fn get_directory(&self) -> &Path {
        &self.directory
    }
    /// Path where annotation file is located.
    pub fn make_annotation_path(
        &self,
        class: &str,
        hash: &str,
        name: &str,
        version: &str,
    ) -> PathBuf {
        PathBuf::from(format!(
            "{}/{}/{}/{}/{}-{}.yaml",
            self.directory.to_string_lossy(),
            "annotation",
            class,
            name,
            hash,
            version,
        ))
    }
    /// Path where specification file is located.
    pub fn make_spec_path(&self, class: &str, hash: &str) -> PathBuf {
        PathBuf::from(format!(
            "{}/{}/{}/{}",
            self.directory.to_string_lossy(),
            class,
            hash,
            "spec.yaml",
        ))
    }

    fn parse_annotation_path(
        path: &Path,
    ) -> Result<impl Iterator<Item = Result<(String, (String, String))>>> {
        let re = Regex::new(
            r"(?x)
            ^.*
            \/(?<name>[0-9a-zA-Z\-]+)
            \/
                (?<hash>[0-9a-f]+)
                -
                (?<version>[0-9]+\.[0-9]+\.[0-9]+)
                \.yaml
            $",
        )?;
        let paths = glob::glob(&path.to_string_lossy())?.map(move |filepath| {
            let filepath_string = String::from(filepath?.to_string_lossy());
            let group = re
                .captures(&filepath_string)
                .ok_or_else(|| OrcaError::from(Kind::NoRegexMatch))?;
            Ok((
                group["name"].to_string(),
                (group["hash"].to_string(), group["version"].to_string()),
            ))
        });
        Ok(paths)
    }

    fn get_version_map<T>(&self, name: &str) -> Result<BTreeMap<String, String>> {
        Self::parse_annotation_path(&self.make_annotation_path(
            &get_type_name::<T>(),
            "*",
            name,
            "*",
        ))?
        .map(|metadata| -> Result<(String, String)> {
            let resolved_metadata = metadata?;
            let hash = resolved_metadata.1 .0;
            let version = resolved_metadata.1 .1;
            Ok((version, hash))
        })
        .collect::<Result<BTreeMap<String, String>>>()
    }

    fn save_file(file: &Path, content: &str, fail_if_exists: bool) -> Result<()> {
        if let Some(parent) = file.parent() {
            fs::create_dir_all(parent)?;
        }
        let file_exists = file.exists();
        if file_exists && fail_if_exists {
            return Err(OrcaError::from(Kind::FileExists(file.to_path_buf())));
        } else if file_exists {
            println!(
                "Skip saving `{}` since it is already stored.",
                file.to_string_lossy().bright_cyan(),
            );
        } else {
            fs::write(file, content)?;
        }
        Ok(())
    }

    fn save_model<T: Serialize>(
        &self,
        model: &T,
        hash: &str,
        annotation: &Annotation,
    ) -> Result<()> {
        let class = get_type_name::<T>();
        // Save the annotation file and throw and error if exist
        Self::save_file(
            &self.make_annotation_path(&class, hash, &annotation.name, &annotation.version),
            &serde_yaml::to_string(&annotation)?,
            true,
        )?;
        // Save the pod and skip if it already exist, for the case of many annotation to a single pod
        Self::save_file(&self.make_spec_path(&class, hash), &to_yaml(model)?, false)?;

        Ok(())
    }

    fn load_model<T: DeserializeOwned>(&self, name: &str, version: &str) -> Result<T> {
        let class = get_type_name::<T>();

        let (_, (hash, _)) =
            Self::parse_annotation_path(&self.make_annotation_path(&class, "*", name, version))?
                .next()
                .ok_or_else(|| {
                    OrcaError::from(Kind::NoAnnotationFound(
                        class.clone(),
                        name.to_owned(),
                        version.to_owned(),
                    ))
                })??;

        from_yaml(
            &hash,
            &fs::read_to_string(self.make_spec_path(&class, &hash))?,
            &fs::read_to_string(self.make_annotation_path(&class, &hash, name, version))?,
        )
    }

    fn list_model<T>(&self) -> Result<BTreeMap<String, Vec<String>>> {
        let (names, (hashes, versions)) = Self::parse_annotation_path(&self.make_annotation_path(
            &get_type_name::<T>(),
            "*",
            "*",
            "*",
        ))?
        .collect::<Result<(Vec<_>, (Vec<_>, Vec<_>))>>()?;

        Ok(BTreeMap::from([
            (String::from("name"), names),
            (String::from("hash"), hashes),
            (String::from("version"), versions),
        ]))
    }

    fn delete_model<T>(&self, name: &str, version: &str) -> Result<()> {
        // assumes propagate = false
        let class = get_type_name::<T>();
        let versions = self.get_version_map::<T>(name)?;
        let hash = versions.get(version).ok_or_else(|| {
            OrcaError::from(Kind::NoAnnotationFound(
                class.clone(),
                name.to_owned(),
                version.to_owned(),
            ))
        })?;

        let annotation_file = self.make_annotation_path(&class, hash, name, version);
        let annotation_dir = annotation_file
            .parent()
            .ok_or_else(|| OrcaError::from(Kind::FileHasNoParent(annotation_file.clone())))?;
        let spec_file = self.make_spec_path(&class, hash);
        let spec_dir = spec_file
            .parent()
            .ok_or_else(|| OrcaError::from(Kind::FileHasNoParent(spec_file.clone())))?;

        fs::remove_file(&annotation_file)?;
        if !versions
            .iter()
            .any(|(list_version, list_hash)| list_version != version && list_hash == hash)
        {
            fs::remove_dir_all(spec_dir)?;
        }
        if !versions
            .iter()
            .any(|(list_version, _)| list_version != version)
        {
            fs::remove_dir_all(annotation_dir)?;
        }

        Ok(())
    }
}
