//! Data persistence is provided by using a store backend.
use crate::{
    error::{AnnotationExists, FileHasNoParent, NoAnnotationFound, NoSpecFound},
    model::{from_yaml, to_yaml, Pod},
    util::get_struct_name,
};
use glob::{GlobError, Paths};
use regex::Regex;
use std::{collections::BTreeMap, error::Error, fs, iter::Map, path::PathBuf};

/// Standard behavior of any store backend supported.
pub trait OrcaStore {
    /// How a pod is stored.
    fn save_pod(&self, pod: &Pod) -> Result<(), Box<dyn Error>>;
    /// How to query stored pods.
    fn list_pod(&self) -> Result<BTreeMap<String, Vec<String>>, Box<dyn Error>>;
    /// How to load a stored pod into a model instance.
    fn load_pod(&self, name: &str, version: &str) -> Result<Pod, Box<dyn Error>>;
    /// How to delete a stored pod.
    fn delete_pod(&self, name: &str, version: &str) -> Result<(), Box<dyn Error>>;
}

/// Support for a storage backend on a local filesystem directory.
#[derive(Debug)]
pub struct LocalFileStore {
    location: PathBuf,
}

impl LocalFileStore {
    /// Construct a local file store instance.
    pub fn new(location: impl Into<PathBuf>) -> Self {
        Self {
            location: location.into(),
        }
    }
    fn _parse_annotation_path(
        filepath: &str,
    ) -> Result<
        Map<
            Paths,
            impl FnMut(Result<PathBuf, GlobError>) -> (String, (String, String)),
        >,
        Box<dyn Error>,
    > {
        let paths = glob::glob(filepath)?.map(|p| {
            let re = Regex::new(
                r"(?x)
                ^.*
                \/(?<name>[0-9a-zA-Z\-]+)
                \/
                    (?<hash>[0-9A-F]+)
                    -
                    (?<version>[0-9]+\.[0-9]+\.[0-9]+)
                    \.yaml
                $",
            )
            .unwrap(); // todo: fix unsafe
            let path_string = &p.unwrap().display().to_string(); // todo: fix unsafe
            let cap = re.captures(path_string).unwrap(); // todo: fix unsafe
            (
                cap["name"].to_string(),
                (cap["hash"].to_string(), cap["version"].to_string()),
            )
        });

        Ok(paths)
    }
    fn _get_pod_version_map(
        &self,
        name: &str,
    ) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
        Ok(LocalFileStore::_parse_annotation_path(&format!(
            "{}/annotation/pod/{}/*.yaml",
            self.location.display().to_string(),
            name,
        ))?
        .map(|(_, (h, v))| (v, h))
        .collect::<BTreeMap<String, String>>())
    }
}

impl OrcaStore for LocalFileStore {
    // todo: from pr review: stop using box dynamic error + mod.rs for file
    // todo: annotatoin optional, load_pod with enum(hash, name:version), save_pod will skip annotation save if optional
    fn save_pod(&self, pod: &Pod) -> Result<(), Box<dyn Error>> {
        let class = get_struct_name::<Pod>()?;

        let annotation_yaml = serde_yaml::to_string(&pod.annotation)?;
        let annotation_file = PathBuf::from(format!(
            "{}/{}/{}/{}/{}-{}.yaml",
            self.location.display().to_string(),
            "annotation",
            class,
            pod.annotation.name,
            pod.hash,
            pod.annotation.version,
        ));
        fs::create_dir_all(&annotation_file.parent().ok_or(FileHasNoParent {
            filepath: annotation_file.display().to_string(),
        })?)?;
        (!fs::exists(&annotation_file)?)
            .then_some(())
            .ok_or(AnnotationExists {
                class: class.clone(),
                name: pod.annotation.name.clone(),
                version: pod.annotation.version.clone(),
            })?;
        fs::write(&annotation_file, &annotation_yaml)?;

        let spec_yaml = to_yaml::<Pod>(&pod)?;
        let spec_file = PathBuf::from(format!(
            "{}/{}/{}/{}",
            self.location.display().to_string(),
            class,
            pod.hash,
            "spec.yaml",
        ));
        fs::create_dir_all(&spec_file.parent().ok_or(FileHasNoParent {
            filepath: spec_file.display().to_string(),
        })?)?;
        if fs::exists(&spec_file)? {
            println!(
                "Skip saving `{}:{}` {} since it is already stored.",
                pod.annotation.name, pod.annotation.version, class,
            );
        } else {
            fs::write(&spec_file, &spec_yaml)?;
        }

        Ok(())
    }
    fn list_pod(&self) -> Result<BTreeMap<String, Vec<String>>, Box<dyn Error>> {
        let (names, (hashes, versions)): (Vec<String>, (Vec<String>, Vec<String>)) =
            LocalFileStore::_parse_annotation_path(&format!(
                "{}/annotation/pod/**/*.yaml",
                self.location.display().to_string(),
            ))?
            .unzip();

        Ok(BTreeMap::from([
            (String::from("name"), names),
            (String::from("hash"), hashes),
            (String::from("version"), versions),
        ]))
    }
    fn load_pod(&self, name: &str, version: &str) -> Result<Pod, Box<dyn Error>> {
        let class = "pod".to_string();

        let (_, (hash, _)) = LocalFileStore::_parse_annotation_path(&format!(
            "{}/annotation/pod/{}/*-{}.yaml",
            self.location.display().to_string(),
            name,
            version,
        ))?
        .next()
        .ok_or(NoAnnotationFound {
            class: class.clone(),
            name: name.to_string(),
            version: version.to_string(),
        })?;
        let annotation_file = format!(
            "{}/annotation/pod/{}/{}-{}.yaml",
            self.location.display().to_string(),
            name,
            hash,
            version,
        );

        let spec_file = glob::glob(&format!(
            "{}/pod/{}/spec.yaml",
            self.location.display().to_string(),
            hash,
        ))?
        .next()
        .ok_or(NoSpecFound {
            class: class.clone(),
            name: name.to_string(),
            version: version.to_string(),
        })??;

        Ok(from_yaml::<Pod>(
            &annotation_file,
            &spec_file.display().to_string(),
            &hash,
        )?)
    }
    fn delete_pod(&self, name: &str, version: &str) -> Result<(), Box<dyn Error>> {
        // assumes propagate = false
        let versions = LocalFileStore::_get_pod_version_map(&self, name)?;
        let annotation_dir = format!(
            "{}/annotation/pod/{}",
            self.location.display().to_string(),
            name,
        );

        fs::remove_file(&format!(
            "{}/{}-{}.yaml",
            annotation_dir, versions[version], version,
        ))?;
        if versions
            .iter()
            .filter(|&(v, h)| v != version && h == &versions[v])
            .collect::<BTreeMap<_, _>>()
            .is_empty()
        {
            fs::remove_dir_all(&format!(
                "{}/pod/{}",
                self.location.display().to_string(),
                versions[version],
            ))?;
        }
        if versions
            .iter()
            .filter(|&(v, _)| v != version)
            .collect::<BTreeMap<_, _>>()
            .is_empty()
        {
            fs::remove_dir_all(&annotation_dir)?;
        }

        Ok(())
    }
}
