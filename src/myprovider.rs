use kubelet::resources::DeviceManager;
use kubelet::node::Builder;
use kubelet::plugin_watcher::PluginRegistry;
use kubelet::pod::state::prelude::SharedState;
use kubelet::pod::Pod;
use kubelet::provider::{DevicePluginSupport, Provider, PluginSupport, VolumeSupport};
use kubelet::state::common::terminated::Terminated;
use kubelet::state::common::GenericProvider;
use kubelet::state::common::registered::Registered;
use kubelet::store::Store;
use kubelet::volume::VolumeRef;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::states::pod::PodState;
use crate::states::provider::ProviderState;

// Create some type that will act as your provider
#[derive(Clone)]
pub struct MyProvider {
    shared: ProviderState,
}

// Implement the `Provider` trait for that type
#[async_trait::async_trait]
impl Provider for MyProvider {
    const ARCH: &'static str = "tinyedge";

    type ProviderState = ProviderState;
    type InitialState = Registered<Self>;
    type TerminatedState = Terminated<Self>;
    type PodState = PodState;

    fn provider_state(&self) -> SharedState<ProviderState> {
        Arc::new(RwLock::new(self.shared.clone()))
    }

    // Populate node information, such as taints, so that the scheduler will take these
    // into consideration when scheduling pods to nodes.
    async fn node(&self, builder: &mut Builder) -> anyhow::Result<()> {
        builder.set_architecture("amd64");
        builder.add_taint("NoSchedule", "kubernetes.io/arch", Self::ARCH);
        builder.add_taint("NoExecute", "kubernetes.io/arch", Self::ARCH);
        Ok(())
    }

    async fn initialize_pod_state(&self, pod: &Pod) -> anyhow::Result<Self::PodState> {
        Ok(PodState::new(pod))
    }

    async fn logs(&self, _namespace: String, _pod: String, _container: String, _sender: kubelet::log::Sender) -> anyhow::Result<()> { todo!() }
}

impl PluginSupport for ProviderState {
    fn plugin_registry(&self) -> Option<Arc<PluginRegistry>> {
        None
    }
}

impl DevicePluginSupport for ProviderState {
    fn device_plugin_manager(&self) -> Option<Arc<DeviceManager>> {
        None
    }
}

impl VolumeSupport for ProviderState {
    fn volume_path(&self) -> Option<&Path> {
        Some(self.volume_path.as_ref())
    }
}

static IMAGE_REPO: &str = "docker.io/library";

impl GenericProvider for MyProvider {
    type ProviderState = ProviderState;
    type PodState = crate::states::pod::PodState;
    type RunState = crate::states::pod::initializing::Initializing;

    fn validate_pod_runnable(_pod: &kubelet::pod::Pod) -> anyhow::Result<()> {
        Ok(())
    }

    fn validate_container_runnable(container: &kubelet::container::Container) -> anyhow::Result<()> {
        if let Some(image) = container.image()? {
            if !image.whole().starts_with(IMAGE_REPO) {
                return Err(anyhow::anyhow!("Repository must be '{}'.", &IMAGE_REPO));
            }
        }
        Ok(())
    }
}

pub struct ModuleRunContext {
    pub modules: HashMap<String, Vec<u8>>,
    pub volumes: HashMap<String, VolumeRef>,
    pub env_vars: HashMap<String, HashMap<String, String>>,
}

impl MyProvider {
    pub async fn new(
        kubeconfig: kube::Config,
        store: Arc<dyn Store + Sync + Send>,
    ) -> anyhow::Result<Self> {

        let client = kube::Client::try_from(kubeconfig)?;

        Ok(Self {
            shared: ProviderState {
                client: client,
                volume_path: PathBuf::from("/tmp"),
                store: store
            },
        })
    }
}