use crate::states::provider::ProviderState;
use krator::{ObjectState, SharedState};
use kubelet::backoff::{BackoffStrategy, ExponentialBackoffStrategy};
use kubelet::pod::{Pod, PodKey, Status};
use kubelet::state::common::{BackoffSequence, GenericPodState, ThresholdTrigger};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) mod completed;
pub(crate) mod initializing;
pub(crate) mod running;
pub(crate) mod starting;

// Track pod state amongst pod state handlers.
pub struct PodState {
    _key: PodKey,
    run_context: SharedState<crate::myprovider::ModuleRunContext>,
    errors: usize,
    image_pull_backoff_strategy: ExponentialBackoffStrategy,
    pub(crate) crash_loop_backoff_strategy: ExponentialBackoffStrategy,
}

#[async_trait::async_trait]
impl ObjectState for PodState {
    type Manifest = Pod;
    type Status = Status;
    type SharedState = ProviderState;
    async fn async_drop(self, _shared: &mut Self::SharedState) {}
}

#[async_trait::async_trait]
impl GenericPodState for PodState {
    async fn set_env_vars(&mut self, env_vars: HashMap<String, HashMap<String, String>>) {
        let mut run_context = self.run_context.write().await;
        run_context.env_vars = env_vars;
    }
    async fn set_modules(&mut self, modules: HashMap<String, Vec<u8>>) {
        let mut run_context = self.run_context.write().await;
        run_context.modules = modules;
    }
    // For this provider, set_volumes extends the current volumes rather than re-assigning
    async fn set_volumes(&mut self, volumes: HashMap<String, kubelet::volume::VolumeRef>) {
        let mut run_context = self.run_context.write().await;
        run_context.volumes.extend(volumes);
    }
    async fn backoff(&mut self, sequence: BackoffSequence) {
        let backoff_strategy = match sequence {
            BackoffSequence::ImagePull => &mut self.image_pull_backoff_strategy,
            BackoffSequence::CrashLoop => &mut self.crash_loop_backoff_strategy,
        };
        backoff_strategy.wait().await;
    }
    async fn reset_backoff(&mut self, sequence: BackoffSequence) {
        let backoff_strategy = match sequence {
            BackoffSequence::ImagePull => &mut self.image_pull_backoff_strategy,
            BackoffSequence::CrashLoop => &mut self.crash_loop_backoff_strategy,
        };
        backoff_strategy.reset();
    }
    async fn record_error(&mut self) -> ThresholdTrigger {
        self.errors += 1;
        if self.errors > 3 {
            self.errors = 0;
            ThresholdTrigger::Triggered
        } else {
            ThresholdTrigger::Untriggered
        }
    }
}

impl PodState {
    pub fn new(pod: &Pod) -> Self {
        let run_context = crate::myprovider::ModuleRunContext {
            modules: Default::default(),
            volumes: Default::default(),
            env_vars: Default::default(),
        };
        let key = PodKey::from(pod);
        PodState {
            _key: key,
            run_context: Arc::new(RwLock::new(run_context)),
            errors: 0,
            image_pull_backoff_strategy: ExponentialBackoffStrategy::default(),
            crash_loop_backoff_strategy: ExponentialBackoffStrategy::default(),
        }
    }
}
