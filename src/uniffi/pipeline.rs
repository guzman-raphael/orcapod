use crate::{core::pipeline::NodeInfo, uniffi::model::pipeline::PipelineJob};
use derive_more::Display;
use getset::CloneGetters;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio_util::task::TaskTracker;
use uniffi;

// use case

// w/ agent: send remotely and check progress (agent required since it has direct access to orchestrator. agent should be installed/started in whichever deploy strategy makes sense for user)

// new: initialize
// attach: so pipeline run state is updated
// start_pipeline_run: send so pipeline run starts
// summarize: check updates
// get_pipeline_result: long running waiting until pipeline run becomes a pipeline result

/// Status of a particular compute pipeline run.
#[derive(uniffi::Enum, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PipelineStatus {
    /// Run has completed successfully.
    Completed,
    /// Run failed.
    Failed,
    /// Run has not finished.
    Running,
}

/// Current computational pipeline managed by an orchestrator agent.
#[expect(clippy::field_scoped_visibility_modifiers, reason = "debug")]
#[derive(uniffi::Object, Debug, Display, CloneGetters, Clone)]
#[getset(get_clone, impl_attrs = "#[uniffi::export]")]
#[display("{self:#?}")]
#[uniffi::export(Display)]
pub struct PipelineRun {
    /// Original compute request.
    pub pipeline_job: Arc<PipelineJob>,
    /// Time in epoch when created in seconds.
    pub created: u64,
    /// Time in epoch when terminated in seconds.
    #[getset(skip)]
    pub terminated: Arc<Mutex<Option<u64>>>,
    /// Status of pipeline run.
    #[getset(skip)]
    pub status: Arc<Mutex<PipelineStatus>>,
    #[getset(skip)]
    pub(crate) state: Arc<Mutex<HashMap<String, NodeInfo>>>,
    #[getset(skip)]
    pub(crate) services: TaskTracker,
}

#[uniffi::export]
impl PipelineRun {
    /// # Panics
    #[expect(clippy::expect_used, clippy::unwrap_in_result, reason = "debug")]
    pub fn terminated(&self) -> Option<u64> {
        *self.terminated.lock().expect("debug")
    }
    /// # Panics
    #[expect(clippy::expect_used, reason = "debug")]
    pub fn status(&self) -> PipelineStatus {
        self.status.lock().expect("debug").clone()
    }
}
