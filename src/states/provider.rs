use kubelet::pod::Pod;
use kubelet::state::common::GenericProviderState;
use kubelet::store::Store;
use std::path::PathBuf;
use std::sync::Arc;

// Track shared provider-level state across pods.
#[derive(Clone)]
pub struct ProviderState {
    pub client: kube::Client,
    pub volume_path: PathBuf,
    pub store: Arc<dyn Store + Sync + Send>,
}

#[async_trait::async_trait]
impl GenericProviderState for ProviderState {
    fn client(&self) -> kube::Client {
        self.client.clone()
    }

    fn store(&self) -> std::sync::Arc<dyn kubelet::store::Store + Sync + Send> {
        self.store.clone()
    }

    async fn stop(&self, _pod: &Pod) -> anyhow::Result<()> {
        Ok(())
    }
}
