use kubelet::resources::DeviceManager;
use kubelet::node::Builder;
use kubelet::plugin_watcher::PluginRegistry;
use kubelet::pod::Pod;
use kubelet::provider::{DevicePluginSupport, Provider, PluginSupport};
use std::sync::Arc;
use tokio::sync::RwLock;
use kubelet::pod::state::prelude::*;
use kubelet::pod::state::Stub;

// Create some type that will act as your provider
pub struct MyProvider;

// Track shared provider-level state across pods.
pub struct ProviderState;

// Track pod state amongst pod state handlers.
pub struct PodState;

#[async_trait::async_trait]
impl ObjectState for PodState {
    type Manifest = Pod;
    type Status = PodStatus;
    type SharedState = ProviderState;
    async fn async_drop(self, _provider_state: &mut ProviderState) {}
}

// Implement the `Provider` trait for that type
#[async_trait::async_trait]
impl Provider for MyProvider {
    const ARCH: &'static str = "tinyedge";

    type ProviderState = ProviderState;
    type InitialState = Stub;
    type TerminatedState = Stub;
    type PodState = PodState;

    fn provider_state(&self) -> SharedState<ProviderState> {
        Arc::new(RwLock::new(ProviderState {}))
    }

    // Populate node information, such as taints, so that the scheduler will take these
    // into consideration when scheduling pods to nodes.
    async fn node(&self, builder: &mut Builder) -> anyhow::Result<()> {
        builder.set_architecture("amd64");
        builder.add_taint("NoSchedule", "kubernetes.io/arch", Self::ARCH);
        builder.add_taint("NoExecute", "kubernetes.io/arch", Self::ARCH);
        Ok(())
    }

    async fn initialize_pod_state(&self, _pod: &Pod) -> anyhow::Result<Self::PodState> {
        Ok(PodState)
    }

    async fn logs(&self, namespace: String, pod: String, container: String, sender: kubelet::log::Sender) -> anyhow::Result<()> { todo!() }
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