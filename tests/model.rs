// use orcapod::model::{Annotation, Pod};
// use orcapod::store::{LocalFileStore, OrcaStore};
// use std::collections::BTreeMap;
use std::error::Error;
// use std::path::PathBuf;
use orcapod::model::{to_yaml, Pod};
mod fixtures;
use fixtures::pod_style;
use indoc::indoc;

// #[test]
// fn pod() -> Result<(), Box<dyn Error>> {
//     // todo: clean up
//     let fs = LocalFileStore::new(String::from("./test_store"));
//     let pod = Pod::new(
//         Annotation {
//             name: String::from("style-transfer"),
//             description: String::from("This is an example pod."),
//             version: String::from("0.67.0"),
//         },
//         String::from("tail -f /dev/null"),
//         String::from("zenmldocker/zenml-server:0.67.0"),
//         BTreeMap::from([
//             (
//                 String::from("painting"),
//                 PathBuf::from("/input/painting.png"),
//             ),
//             (String::from("image"), PathBuf::from("/input/image.png")),
//         ]),
//         PathBuf::from("/output"),
//         BTreeMap::from([(String::from("styled"), PathBuf::from("./styled.png"))]),
//         0.25,                   // 250 millicores as frac cores
//         (2 as u64) * (1 << 30), // 2GiB in bytes
//         String::from("https://github.com/zenml-io/zenml/tree/0.67.0"),
//     )?;

//     fs.save_pod(&pod)?;

//     let pod_des = fs.load_pod("style-transfer", "0.67.0")?;
//     println!("{:?}", fs.list_pod());
//     println!("{:?}", pod_des);

//     fs.delete_pod("style-transfer", "0.67.0")?;

//     Ok(())
// }

#[test]
fn verify_hash() -> Result<(), Box<dyn Error>> {
    let pod = pod_style()?;
    assert_eq!(
        pod.hash,
        "8E42779F9C10A3388D03D060028336DA820FF451F16E1C6BD98CF887B1685090"
    );
    Ok(())
}

#[test]
fn verify_pod_to_yaml() -> Result<(), Box<dyn Error>> {
    let pod = pod_style()?;
    let spec_yaml = to_yaml::<Pod>(&pod)?;
    assert_eq!(
        spec_yaml,
        indoc! {"
            class: pod
            command: tail -f /dev/null
            file_content_checksums:
              image.tar.gz: 78efb7728aac9e4e79966bc13703e7cb239ba9c0eb6322c252bea0399ff2421f
            image: zenmldocker/zenml-server@sha256:78efb7728aac9e4e79966bc13703e7cb239ba9c0eb6322c252bea0399ff2421f
            input_stream_map:
              image: /input/image.png
              painting: /input/painting.png
            output_dir: /output
            output_stream_map:
              styled: ./styled.png
            recommended_cpus: 0.25
            recommended_memory: 2147483648
            source: https://github.com/zenml-io/zenml/tree/0.67.0
        "}
    );
    Ok(())
}
