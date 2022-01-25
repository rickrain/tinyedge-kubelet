use super::ContainerState;
use crate::states::provider::ProviderState;
use kubelet::container::state::prelude::*;

#[derive(Default, Debug)]
pub struct Terminated;

impl Terminated {
    pub fn new() -> Self {
        Terminated {}
    }
}

#[async_trait::async_trait]
impl State<ContainerState> for Terminated {
    async fn next(
        self: Box<Self>,
        _provider_state: SharedState<ProviderState>,
        _container_state: &mut ContainerState,
        container: Manifest<Container>,
    ) -> Transition<ContainerState> {
        let _container = container.latest();

        Transition::Complete(Ok(()))
    }

    async fn status(
        &self,
        _container_state: &mut ContainerState,
        _container: &Container,
    ) -> anyhow::Result<Status> {
        Ok(Status::waiting("Terminated"))
    }
}
