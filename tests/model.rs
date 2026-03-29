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
        "21941e5dc2bccb71474ff5daf98b284c151d79e10d4e906c59679b74d08f183e",
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
                path: result1.png
                match_pattern: .*\.png
              result2:
                path: result2.png
                match_pattern: .*\.png
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
        "740282d465c1c4e772b0c28b0a470c6df8a921f9d772085707dcaf0a0f75317b",
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
            pod: 21941e5dc2bccb71474ff5daf98b284c151d79e10d4e906c59679b74d08f183e
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
        "65df61e2934ec106b4cd029d8a0da341f013584b2071826475839711bddbd926",
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
            pod_job: 740282d465c1c4e772b0c28b0a470c6df8a921f9d772085707dcaf0a0f75317b
            output_packet:
              result1:
                kind: File
                location:
                  namespace: default
                  path: output/result1.png
                checksum: aea70197d2a5afde53ad1b481095bbc9f0c50af66b6b8dcf497323484d875e00
              result2:
                kind: File
                location:
                  namespace: default
                  path: output/result2.png
                checksum: 1804e1af0015d711564223e3a4cd94c6c153885cd6812f46507f5a2be041def8
            assigned_name: simple-endeavour
            status: Completed
            created: 1737922307
            terminated: 1737925907
        "},
        "YAML serialization didn't match."
    );
    Ok(())
}
