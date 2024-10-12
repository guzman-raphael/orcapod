use orcapod::{
    model::{Annotation, Pod},
    store::{LocalFileStore, OrcaStore},
};
use std::ops::Deref;
use std::{collections::BTreeMap, error::Error, path::PathBuf};

pub fn pod_style() -> Result<Pod, Box<dyn Error>> {
    Pod::new(
        Annotation {
            name: "style-transfer".to_string(),
            description: "This is an example pod.".to_string(),
            version: "0.67.0".to_string(),
        },
        "tail -f /dev/null".to_string(),
        "zenmldocker/zenml-server:0.67.0".to_string(),
        BTreeMap::from([
            ("painting".to_string(), PathBuf::from("/input/painting.png")),
            ("image".to_string(), PathBuf::from("/input/image.png")),
        ]),
        PathBuf::from("/output"),
        BTreeMap::from([("styled".to_string(), PathBuf::from("./styled.png"))]),
        0.25,                   // 250 millicores as frac cores
        (2 as u64) * (1 << 30), // 2GiB in bytes
        "https://github.com/zenml-io/zenml/tree/0.67.0".to_string(),
    )
}

#[derive(Debug)]
pub struct StoredPod {
    fs: LocalFileStore,
    pod: Pod,
}
pub fn add_pod_storage(pod: Pod) -> Result<StoredPod, Box<dyn Error>> {
    let fs = LocalFileStore::new("./test_store".to_string());

    impl Deref for StoredPod {
        type Target = Pod;

        fn deref(&self) -> &Self::Target {
            &self.pod
        }
    }
    impl Drop for StoredPod {
        fn drop(&mut self) {
            self.fs
                .delete_pod(&self.pod.annotation.name, &self.pod.annotation.version)
                .expect("Failed to teardown."); // required since can't modify drop sig
        }
    }
    let pod_with_storage = StoredPod { fs, pod };

    pod_with_storage.fs.save_pod(&pod_with_storage)?;
    Ok(pod_with_storage)
}
