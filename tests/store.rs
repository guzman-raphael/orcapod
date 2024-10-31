#![expect(clippy::panic_in_result_fn, reason = "Panics OK in tests.")]

pub mod fixture;
use fixture::{add_pod_storage, pod_style, store_test};
use orcapod::{
    error::Result,
    model::{to_yaml, Pod},
    store::filestore::LocalFileStore,
    store::Store,
};
use std::{collections::BTreeMap, fs, path::Path};
use tempfile::tempdir;

fn is_dir_two_levels_up_empty(file: &Path) -> Option<bool> {
    Some(file.parent()?.parent()?.read_dir().ok()?.next().is_none())
}

#[test]
fn verify_pod_save_and_delete() -> Result<()> {
    let store_directory = String::from(tempdir()?.path().to_string_lossy());
    {
        let pod_style = pod_style()?;
        let store = store_test(Some(&store_directory))?; // new tests can just call store_test(None)?
        let annotation_file = store.make_path::<Pod>(
            &pod_style.hash,
            &LocalFileStore::make_annotation_filename(
                &pod_style.annotation.name,
                &pod_style.annotation.version,
            ),
        );
        let spec_file = store.make_path::<Pod>(&pod_style.hash, LocalFileStore::SPEC_FILENAME);
        {
            let pod = add_pod_storage(pod_style, &store)?;
            assert!(spec_file.exists());
            assert_eq!(fs::read_to_string(&spec_file)?, to_yaml::<Pod>(&pod)?);
            assert!(annotation_file.exists());
        };
        assert!(!spec_file.exists());
        assert!(!annotation_file.exists());
        assert_eq!(is_dir_two_levels_up_empty(&spec_file), Some(true));
        assert_eq!(is_dir_two_levels_up_empty(&annotation_file), Some(true));
    };
    assert!(!fs::exists(&store_directory)?);
    Ok(())
}

#[test]
fn verify_pod_load() -> Result<()> {
    let store = store_test(None)?;
    let stored_pod = add_pod_storage(pod_style()?, &store)?;
    let loaded_pod = store.load_pod(&stored_pod.annotation.name, &stored_pod.annotation.version)?;
    assert_eq!(loaded_pod, stored_pod.pod);
    Ok(())
}

#[test]
fn verify_pod_list() -> Result<()> {
    let store = store_test(None)?;
    assert_eq!(
        store.list_pod()?,
        BTreeMap::from([
            ("hash".to_owned(), vec![],),
            ("name".to_owned(), vec![],),
            ("version".to_owned(), vec![],),
        ])
    );
    Ok(())
}
