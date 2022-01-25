use super::ContainerState;
use crate::states::container::running::Running;
use crate::states::container::terminated::Terminated;
use crate::states::provider::ProviderState;
use kubelet::container::state::prelude::*;
use tracing::*;

#[derive(Default, Debug, TransitionTo)]
#[transition_to(Running, Terminated)]
pub struct Waiting;

#[async_trait::async_trait]
impl State<ContainerState> for Waiting {
    async fn next(
        self: Box<Self>,
        _provider_state: SharedState<ProviderState>,
        container_state: &mut ContainerState,
        container: Manifest<Container>,
    ) -> Transition<ContainerState> {
        let _container = container.latest();

        info!("Starting container for pod {}.", container_state.pod.name());

        Transition::next(self, Running::new())
    }

    async fn status(
        &self,
        _container_state: &mut ContainerState,
        _container: &Container,
    ) -> anyhow::Result<Status> {
        Ok(Status::waiting("Waiting"))
    }
}
