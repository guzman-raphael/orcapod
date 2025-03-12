#![expect(missing_docs, clippy::panic_in_result_fn, reason = "OK in tests.")]

pub mod fixture;
use fixture::{pod_job_style, pod_result_style, pod_style, NAMESPACE_LOOKUP_READ_ONLY};
use indoc::indoc;
use orcapod::{error::Result, model::to_yaml};

#[test]
fn hash_pod() -> Result<()> {
    assert_eq!(
        pod_style()?.hash,
        "8e34979d6c526e5948bafbfa42e3df23c42b2a98082b97510f64f77b8a68e094",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_to_yaml() -> Result<()> {
    assert_eq!(
        to_yaml(&pod_style()?)?,
        indoc! {r"
            class: pod
            image: example.server.com/user/style-transfer:1.0.0
            command: python /run.py
            input_stream_map:
              image:
                path: /input/image.jpeg
                match_pattern: .*\.jpeg
              style:
                path: /input/style.t7
                match_pattern: .*\.t7
            output_dir: /output
            output_stream_map:
              result:
                path: ./result.jpeg
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
        "dcb5c3809b5b785494a894943dbb9f1bf575eaaf2d0a2795ca480ebe58ce3430",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_job_to_yaml() -> Result<()> {
    assert_eq!(
        to_yaml(&pod_job_style(&NAMESPACE_LOOKUP_READ_ONLY)?)?,
        indoc! {"
            class: pod_job
            pod: 8e34979d6c526e5948bafbfa42e3df23c42b2a98082b97510f64f77b8a68e094
            input_stream_path:
              image:
                kind: File
                location:
                  namespace: default
                  path: images/dog.jpeg
                checksum: 8b44b8ea83b1f5eec3ac16cf941767e629896c465803fb69c21adbbf984516bd
              style:
                kind: File
                location:
                  namespace: default
                  path: styles/mosaic.t7
                checksum: fbd7d882e9e02aafb57366e726762025ff6b2e12cd41abd44b874542b7693771
            output_stream_path:
              kind: Folder
              location:
                namespace: default
                path: output
              checksum: null
            cpu_limit: 0.5
            memory_limit: 2147483648
            env_vars: null
        "},
        "YAML serialization didn't match."
    );
    Ok(())
}

#[test]
fn hash_pod_result() -> Result<()> {
    assert_eq!(
        pod_result_style(&NAMESPACE_LOOKUP_READ_ONLY)?.hash,
        "e9e6595f6e0e632616d3353968719b38515e27e09254156115d0e03df34b6762",
        "Hash didn't match."
    );
    Ok(())
}

#[test]
fn pod_result_to_yaml() -> Result<()> {
    assert_eq!(
        to_yaml(&pod_result_style(&NAMESPACE_LOOKUP_READ_ONLY)?)?,
        indoc! {"
            class: pod_result
            pod_job: dcb5c3809b5b785494a894943dbb9f1bf575eaaf2d0a2795ca480ebe58ce3430
            assigned_name: simple-endeavour
            status: Completed
            created: 1737922307
            terminated: 1737925907
        "},
        "YAML serialization didn't match."
    );
    Ok(())
}
