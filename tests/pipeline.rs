#![expect(
    missing_docs,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "OK in tests."
)]

pub mod fixture;
use fixture::{NAMESPACE_LOOKUP_READ_ONLY, TestDirs, pipeline_job_adder, pod_custom};
use indoc::indoc;
use names::{Generator, Name};
use orcapod::uniffi::{
    error::Result,
    model::{
        packet::{Blob, BlobKind, PathInfo, PathSet, URI},
        pipeline::{Kernel, Pipeline, PipelineJob, SpecURI},
    },
    orchestrator::{
        agent::{Agent, AgentClient},
        docker::LocalDockerOrchestrator,
    },
    pipeline::PipelineStatus,
    store::filestore::LocalFileStore,
};
use pretty_assertions::assert_eq as pretty_assert_eq;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::{task::JoinSet, time::sleep as async_sleep};

#[test]
fn input_packet_checksum() -> Result<()> {
    let pipeline = Pipeline::new(
        indoc! {"
            digraph {
                A
            }
        "},
        HashMap::from([(
            "A".into(),
            Kernel::Pod {
                r#ref: pod_custom(
                    "alpine:3.14",
                    &["echo".into()],
                    HashMap::from([(
                        "node_key_1".into(),
                        PathInfo {
                            path: "/tmp/input".into(),
                            match_pattern: r".*\.jpeg".into(),
                        },
                    )]),
                )?
                .into(),
            },
        )]),
        &HashMap::from([(
            "pipeline_key_1".into(),
            vec![SpecURI {
                node: "A".into(),
                key: "node_key_1".into(),
            }],
        )]),
        &HashMap::new(),
    )?;

    let pipeline_job = PipelineJob::new(
        pipeline.into(),
        &HashMap::from([(
            "pipeline_key_1".into(),
            vec![PathSet::Collection(vec![Blob {
                kind: BlobKind::File,
                location: URI {
                    namespace: "default".into(),
                    path: "images/subject.jpeg".into(),
                },
                checksum: String::new(),
            }])],
        )]),
        &URI {
            namespace: "default".into(),
            path: "output/pipeline".into(),
        },
        &NAMESPACE_LOOKUP_READ_ONLY,
    )?;

    let checksum = match &pipeline_job.input_packet["pipeline_key_1"].first() {
        Some(PathSet::Collection(blobs)) => blobs[0].checksum.clone(),
        Some(_) | None => panic!("Input configuration unexpectedly changed."),
    };

    assert_eq!(
        checksum,
        "8b44b8ea83b1f5eec3ac16cf941767e629896c465803fb69c21adbbf984516bd".to_owned(),
        "Incorrect checksum"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn adder_success() -> Result<()> {
    verify_pipeline(
        |data_dir_filepath, namespace_lookup| {
            pipeline_job_adder(5, &[], data_dir_filepath, namespace_lookup)
        },
        PipelineStatus::Completed,
        15,
    )
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn adder_failure() -> Result<()> {
    verify_pipeline(
        |data_dir_filepath, namespace_lookup| {
            pipeline_job_adder(5, &["add_d"], data_dir_filepath, namespace_lookup)
        },
        PipelineStatus::Failed,
        10,
    )
    .await
}

async fn verify_pipeline(
    make_pipeline_job: fn(&Path, &HashMap<String, PathBuf>) -> Result<PipelineJob>,
    expected_pipeline_status: PipelineStatus,
    expected_runtime: u64,
) -> Result<()> {
    let test_dirs = TestDirs::new(&HashMap::from([("default".to_owned(), None::<String>)]))?;
    let data_dir_filepath = test_dirs.namespace_lookup()["default"].join("data");
    let store = LocalFileStore::new(test_dirs.namespace_lookup()["default"].join("store"));
    // config
    let margin_millis = 6;
    let (group, host) = (
        Generator::with_naming(Name::Plain)
            .next()
            .expect("Generated names overflowed."),
        "host".to_owned(),
    );
    // api
    let client = AgentClient::new(group.clone(), host.clone())?;
    let agent = Agent::new(group, host, Arc::new(LocalDockerOrchestrator::new()?))?;
    // background services
    let mut services = JoinSet::new();
    services.spawn({
        let inner_client = client.clone();
        async move { inner_client.watch("**".to_owned()).await }
    });
    services.spawn({
        let inner_agent = agent.clone();
        let inner_store = store.clone();
        let namespace_lookup = test_dirs.namespace_lookup();
        async move {
            inner_agent
                .start(&namespace_lookup, Some(inner_store.into()))
                .await
        }
    });
    // setup pipeline
    let pipeline_job =
        make_pipeline_job(data_dir_filepath.as_path(), &test_dirs.namespace_lookup())?;
    // submit request
    services.spawn(async move {
        let pipeline_run = client.start_pipeline_job(pipeline_job.into()).await?;
        assert_eq!(
            pipeline_run.status(),
            PipelineStatus::Running,
            "Pipeline not running."
        );
        let pipeline_run_pointer = Arc::new(pipeline_run);
        let pipeline_result = client
            .get_pipeline_result(Arc::clone(&pipeline_run_pointer))
            .await?;
        // give pipeline run a chance to update its status
        // give watch console stream a chance to catch up
        async_sleep(Duration::from_secs(1)).await;
        let pipeline_result_again = client
            .get_pipeline_result(Arc::clone(&pipeline_run_pointer))
            .await?;

        pretty_assert_eq!(
            pipeline_result, pipeline_result_again,
            "Pipeline results inconsistent."
        );
        assert_eq!(
            pipeline_result.status, expected_pipeline_status,
            "Pipeline status unexpected."
        );
        let actual_runtime = pipeline_result.terminated - pipeline_result.created;
        let expected_runtime_with_margin = expected_runtime + margin_millis;
        assert!(
            actual_runtime <= expected_runtime_with_margin,
            "Pipeline took too long (actual={actual_runtime}, expected={expected_runtime_with_margin})."
        );
        Ok(())
    });
    services.spawn(async {
        async_sleep(Duration::from_secs(60)).await;
        panic!("Test took too long. Killing...");
    });

    services
        .join_next()
        .await
        .expect("Services unexpectedly empty")?
}
