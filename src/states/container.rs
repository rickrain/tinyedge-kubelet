use crate::states::provider::ProviderState;
use krator::ObjectState;
use kubelet::container::{Container, Status};

pub(crate) mod running;
pub(crate) mod terminated;
pub(crate) mod waiting;

pub struct ContainerState {
    pub pod: kubelet::pod::Pod, // The pod this container belongs to.
}

#[async_trait::async_trait]
impl ObjectState for ContainerState {
    type Manifest = Container;
    type Status = Status;
    type SharedState = ProviderState;
    async fn async_drop(self, _shared: &mut Self::SharedState) {}
}
