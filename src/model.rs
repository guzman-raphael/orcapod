//! Components of the data model.
use crate::util::{get_struct_name, hash_buffer};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Converts a model instance into a consistent yaml.
pub fn to_yaml<T: Serialize>(instance: &T) -> Result<String, Box<dyn Error>> {
    let mapping: BTreeMap<String, Value> =
        serde_yaml::from_str(&serde_yaml::to_string(instance)?)?; // sort
    let mut yaml = serde_yaml::to_string(
        &mapping
            .into_iter()
            .filter(|(k, _)| k != "annotation" && k != "hash")
            .collect::<BTreeMap<_, _>>(),
    )?; // skip fields
    yaml.insert_str(0, &format!("class: {}\n", get_struct_name::<T>()?)); // replace class at top

    Ok(yaml)
}

/// Instantiates a model from from yaml content and its unique hash.
pub fn from_yaml<T: DeserializeOwned>(
    annotation_file: &str,
    spec_file: &str,
    hash: &str,
) -> Result<T, Box<dyn Error>> {
    let annotation: Mapping =
        serde_yaml::from_str(&fs::read_to_string(&annotation_file)?)?;
    let spec_yaml = BufReader::new(fs::File::open(&spec_file)?)
        .lines()
        .skip(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    let mut spec_mapping: BTreeMap<String, Value> = serde_yaml::from_str(&spec_yaml)?;
    spec_mapping.insert("annotation".to_string(), Value::from(annotation));
    spec_mapping.insert("hash".to_string(), Value::from(hash));

    let instance: T = serde_yaml::from_str(&serde_yaml::to_string(&spec_mapping)?)?;
    Ok(instance)
}

// --- core model structs ---

/// A reusable, containerized computational unit.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pod {
    command: String,
    /// Metadata that doesn't affect reproducibility.
    pub annotation: Annotation,
    file_content_checksums: BTreeMap<PathBuf, String>,
    // gpu: Option<GPUSpecRequirement>
    /// Unique id based on reproducibility.
    pub hash: String,
    image: String, // Sha256 of docker image hash
    input_stream_map: BTreeMap<String, PathBuf>, // change this back to KeyInfo later
    // might default to /output but needs
    output_dir: PathBuf,
    // here PathBuf will be relative to output_dir
    output_stream_map: BTreeMap<String, PathBuf>,
    recommended_cpus: f32,
    recommended_memory: u64,
    source: String, // Git Commmit

                    // fn save(&self, fs: FileStore) -> Result {
                    // 	fs.save_pod(*self)
                    // download image.tar.gz on save
                    // }
}

impl Pod {
    // todo: default
    /// Construct a new pod instance.
    pub fn new(
        annotation: Annotation,
        command: String,
        image: String,
        input_stream_map: BTreeMap<String, PathBuf>,
        output_dir: PathBuf,
        output_stream_map: BTreeMap<String, PathBuf>,
        recommended_cpus: f32,
        recommended_memory: u64,
        source: String,
    ) -> Result<Self, Box<dyn Error>> {
        // todo: resolved_image+checksums -> docker image:tag -> image@digest
        let resolved_image = String::from(
            "zenmldocker/zenml-server@sha256:78efb7728aac9e4e79966bc13703e7cb239ba9c0eb6322c252bea0399ff2421f"
        );
        let checksums = BTreeMap::from([(
            PathBuf::from("image.tar.gz"),
            String::from(
                "78efb7728aac9e4e79966bc13703e7cb239ba9c0eb6322c252bea0399ff2421f",
            ),
        )]);
        let pod_no_hash = Self {
            annotation,
            command,
            file_content_checksums: checksums,
            hash: "".to_string(),
            image: resolved_image,
            input_stream_map,
            output_dir,
            output_stream_map,
            recommended_cpus,
            recommended_memory,
            source,
        };
        let hash = hash_buffer(&to_yaml::<Pod>(&pod_no_hash)?);
        Ok(Self {
            hash,
            ..pod_no_hash
        })
    }
}

// --- util structs ---

/// Standard metadata structure for all model instances.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotation {
    /// A unique name.
    pub name: String,
    /// A unique semantic version.
    pub version: String,
    /// A long form description.
    pub description: String,
}
