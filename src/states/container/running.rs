use super::ContainerState;
use crate::states::container::terminated::Terminated;
use crate::states::provider::ProviderState;
use kubelet::container::state::prelude::*;

#[derive(Default, Debug, TransitionTo)]
#[transition_to(Terminated)]
pub struct Running;

impl Running {
    pub fn new() -> Self {
        Running {}
    }
}

#[async_trait::async_trait]
impl State<ContainerState> for Running {
    async fn next(
        self: Box<Self>,
        _provider_state: SharedState<ProviderState>,
        _container_state: &mut ContainerState,
        container: Manifest<Container>,
    ) -> Transition<ContainerState> {
        let _container = container.latest();

        Transition::next(self, Terminated::new())
    }

    async fn status(
        &self,
        _container_state: &mut ContainerState,
        _container: &Container,
    ) -> anyhow::Result<Status> {
        Ok(Status::waiting("Running"))
    }
}
