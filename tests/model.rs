#![expect(missing_docs, clippy::panic_in_result_fn, reason = "OK in tests.")]

pub mod fixture;
use fixture::{NAMESPACE_LOOKUP_READ_ONLY, pod_job_segment, pod_result_segment, pod_segment};
use indoc::indoc;
use orcapod::{core::model::to_yaml, uniffi::error::Result};
use pretty_assertions::assert_eq as pretty_assert_eq;

#[test]
fn hash_pod() -> Result<()> {
    assert_eq!(
        pod_segment()?.hash,
        "454c117d131912e96b36d0f9c33a32c8f71df1546dfa816abe3a8a1a78af284b",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_to_yaml() -> Result<()> {
    pretty_assert_eq!(
        to_yaml(&pod_segment()?)?,
        indoc! {r"
            class: pod
            image: example.server.com/user/segment:1.0.0
            command:
            - python3
            - /run.py
            input_spec:
              base-input:
                path: /input
                match_pattern: input/.*
              extra-config:
                path: /extra_configs/config2.csv
                match_pattern: .*\.csv
            output_dir: /output
            output_spec:
              overlay_image1:
                path: overlay_image1.png
                match_pattern: .*\.png
              overlay_image2:
                path: overlay_image2.png
                match_pattern: .*\.png
            source_commit_url: https://github.com/user/segment/tree/1.0.0
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
        pod_job_segment(&NAMESPACE_LOOKUP_READ_ONLY)?.hash,
        "31caf15566c088a39b7734b15c6005a449a967d4fe0aca403b6a78f4ced63de7",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_job_to_yaml() -> Result<()> {
    pretty_assert_eq!(
        to_yaml(&pod_job_segment(&NAMESPACE_LOOKUP_READ_ONLY)?)?,
        indoc! {"
            class: pod_job
            pod: 454c117d131912e96b36d0f9c33a32c8f71df1546dfa816abe3a8a1a78af284b
            input_packet:
              base-input:
              - kind: File
                location:
                  namespace: default
                  path: overlay_rgb_configs/config1.csv
                checksum: 9b7c08286f87e667dbd2dfee7f61a62185d40752bec4e46cb75add5957583ac1
              - kind: File
                location:
                  namespace: default
                  path: images/subject.jpeg
                checksum: 263f0561cc87e60e0fab37dda4121450dd1d61d4c515dcb66cac19de46c9dd6c
              extra-config:
                kind: File
                location:
                  namespace: default
                  path: overlay_rgb_configs/colorblind_friendly.csv
                checksum: f78dfdfbdfd3ffec1e0731f155cb4293406d627928ea93df363a790eb3eaed38
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
        pod_result_segment(&NAMESPACE_LOOKUP_READ_ONLY)?.hash,
        "00f0d08f438c2ad61984c81bd8208037f973de9dbb488569b6518501016664b3",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_result_to_yaml() -> Result<()> {
    pretty_assert_eq!(
        to_yaml(&pod_result_segment(&NAMESPACE_LOOKUP_READ_ONLY)?)?,
        indoc! {"
            class: pod_result
            pod_job: 31caf15566c088a39b7734b15c6005a449a967d4fe0aca403b6a78f4ced63de7
            output_packet:
              overlay_image1:
                kind: File
                location:
                  namespace: default
                  path: output/overlay_image1.png
                checksum: f7ee153db93f7a287b59bfae4aafc4864ac90c5fafd3663df8d40f147f7d4ab4
              overlay_image2:
                kind: File
                location:
                  namespace: default
                  path: output/overlay_image2.png
                checksum: 2f8baec0e8f595cc14907e5caab8c8eb4865e5806149e9646679127669258f0c
            assigned_name: simple-endeavour
            status: Completed
            created: 1737922307
            terminated: 1737925907
        "},
        "YAML serialization didn't match."
    );
    Ok(())
}
