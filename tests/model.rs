#![expect(missing_docs, clippy::panic_in_result_fn, reason = "OK in tests.")]

pub mod fixture;
use fixture::{NAMESPACE_LOOKUP_READ_ONLY, pod_job_style, pod_result_style, pod_style};
use indoc::indoc;
use orcapod::{core::model::to_yaml, uniffi::error::Result};
use pretty_assertions::assert_eq as pretty_assert_eq;

#[test]
fn hash_pod() -> Result<()> {
    assert_eq!(
        pod_style()?.hash,
        "bdcd489a09ef462e864e9ea48e0e4cf4f756dbfd70f000c6813a8a109a5bcae9",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_to_yaml() -> Result<()> {
    pretty_assert_eq!(
        to_yaml(&pod_style()?)?,
        indoc! {r"
            class: pod
            image: example.server.com/user/style-transfer:1.0.0
            command:
            - python3
            - /run.py
            input_spec:
              base-input:
                path: /input
                match_pattern: input/.*
              extra-style:
                path: /extra_styles/style2.t7
                match_pattern: .*\.t7
            output_dir: /output
            output_spec:
              result1:
                path: result1.jpeg
                match_pattern: .*\.jpeg
              result2:
                path: result2.jpeg
                match_pattern: .*\.jpeg
            source_commit_url: https://github.com/user/style-transfer/tree/1.0.0
            recommended_cpus: 0.25
            recommended_memory: 1073741824
            required_gpu: null
        "},
        "YAML serialization didn't match."
    );
    Ok(())
}

#[test]
fn hash_pod_job() -> Result<()> {
    assert_eq!(
        pod_job_style(&NAMESPACE_LOOKUP_READ_ONLY)?.hash,
        "4dc6fe947cd406d40f3d971676c5b33da31ba04606b5f4a179069fa2f5e8721b",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_job_to_yaml() -> Result<()> {
    pretty_assert_eq!(
        to_yaml(&pod_job_style(&NAMESPACE_LOOKUP_READ_ONLY)?)?,
        indoc! {"
            class: pod_job
            pod: bdcd489a09ef462e864e9ea48e0e4cf4f756dbfd70f000c6813a8a109a5bcae9
            input_packet:
              base-input:
              - kind: File
                location:
                  namespace: default
                  path: styles/style1.t7
                checksum: 69e709c1697e290994d2da75ddfb2097bf801a9436a3727a282e0230e703da2b
              - kind: File
                location:
                  namespace: default
                  path: images/subject.jpeg
                checksum: 8b44b8ea83b1f5eec3ac16cf941767e629896c465803fb69c21adbbf984516bd
              extra-style:
                kind: File
                location:
                  namespace: default
                  path: styles/mosaic.t7
                checksum: fbd7d882e9e02aafb57366e726762025ff6b2e12cd41abd44b874542b7693771
            output_dir:
              namespace: default
              path: output
            cpu_limit: 0.5
            memory_limit: 2147483648
            env_vars:
              AAA: SORT
              ZZZ: PLEASE
        "},
        "YAML serialization didn't match."
    );
    Ok(())
}

#[test]
fn hash_pod_result() -> Result<()> {
    assert_eq!(
        pod_result_style(&NAMESPACE_LOOKUP_READ_ONLY)?.hash,
        "4503ce41026f1464f02f084b3a520fba4f8d5a3dcd65f0caef9c43b77689ea07",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_result_to_yaml() -> Result<()> {
    pretty_assert_eq!(
        to_yaml(&pod_result_style(&NAMESPACE_LOOKUP_READ_ONLY)?)?,
        indoc! {"
            class: pod_result
            pod_job: 4dc6fe947cd406d40f3d971676c5b33da31ba04606b5f4a179069fa2f5e8721b
            output_packet:
              result1:
                kind: File
                location:
                  namespace: default
                  path: output/result1.jpeg
                checksum: 0a77eed486d85cd058b077711c938e5f1f6a84ea0c253cee4c1a1d8c5d9fdeef
              result2:
                kind: File
                location:
                  namespace: default
                  path: output/result2.jpeg
                checksum: 82adff54d6188fa0049af7a40c304531f4dbb29d34f68a8b2706e5096e30aa30
            assigned_name: simple-endeavour
            status: Completed
            created: 1737922307
            terminated: 1737925907
        "},
        "YAML serialization didn't match."
    );
    Ok(())
}
