use kubelet::state::TransitionTo;
use kubelet::pod::state::prelude::*;

use crate::states::pod::{PodState,ProviderState};
use super::completed::Completed;

#[derive(Default, Debug, TransitionTo)]
#[transition_to(Completed)]
pub struct Running;

#[async_trait::async_trait]
impl State<PodState> for Running {
    async fn next(
        self: Box<Self>,
        _provider_state: SharedState<ProviderState>,
        _pod_state: &mut PodState,
        pod: Manifest<Pod>,
    ) -> Transition<PodState> {
        let _pod = pod.latest();

        Transition::next(self, Completed)
    }

    async fn status(&self, _pod_state: &mut PodState, _pod: &Pod) -> anyhow::Result<PodStatus> {
        Ok(make_status(Phase::Succeeded, "Running"))
    } 
}
