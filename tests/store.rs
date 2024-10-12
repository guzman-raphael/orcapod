use std::error::Error;

mod fixtures;

use fixtures::{add_pod_storage, pod_style};

// todo test: add GHA (integration tests + code coverage)

// X (decided against it) todo model test: require use of Pod::new
// X todo model test: to_yaml for Pod
// todo model test: from_yaml for Pod
// X todo model test: hash correct for Pod

// todo store test: save_pod (annotation + spec written)
// todo store test: save_pod with annotation that already exists (Err(AnnotationExists))
// todo store test: save_pod with spec that already exists (skipped and logged)
// todo store test: save_pod w/o annotation (spec written)
// todo store test: save_pod w/o annotation with spec that already exists (skipped and logged)
// todo store test: load_pod (instance matches values in annotation + spec)
// todo store test: load_pod with missing annotation (Err(NoAnnotationFound))
// todo store test: load_pod with missing spec (Err(NoSpecFound))
// todo store test: load_pod w/ hash (instance matches spec)
// todo store test: load_pod w/ hash with missing spec (Err(NoSpecFound))
// todo store test: list_pod (displays correct saved pods, include w/ w/o annotation)
// todo store test: delete_pod (removes annotation leaves spec)
// todo store test: delete_pod (removes annotation, removes spec dir if last ref'ed annotation)
// todo store test: delete_pod (removes annotation, removes spec dir, removes annotation dir if last version)
// todo store test: delete_pod w/ hash (removes spec + all annotations)

#[test]
fn verify_saved() -> Result<(), Box<dyn Error>> {
    let pod = add_pod_storage(pod_style()?)?;
    println!("pod annotation: {:?}", pod.annotation);
    assert!(false);
    Ok(())
}
