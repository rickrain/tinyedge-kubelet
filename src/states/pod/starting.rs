use kubelet::state::TransitionTo;
use kubelet::pod::state::prelude::*;

use crate::states::pod::{PodState,ProviderState};
use super::running::Running;

#[derive(Default, Debug, TransitionTo)]
#[transition_to(Running)]
pub struct Starting;

#[async_trait::async_trait]
impl State<PodState> for Starting {
    async fn next(
        self: Box<Self>,
        _provider_state: SharedState<ProviderState>,
        _pod_state: &mut PodState,
        pod: Manifest<Pod>,
    ) -> Transition<PodState> {
        let pod = pod.latest();

        // Execute containers in the pod spec
        for _container in pod.containers() { }

        Transition::next(self, Running)
    }

    async fn status(&self, _pod_state: &mut PodState, _pod: &Pod) -> anyhow::Result<PodStatus> {
        Ok(make_status(Phase::Succeeded, "Starting"))
    } 
}
