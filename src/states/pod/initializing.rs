use kubelet::state::TransitionTo;
use kubelet::pod::state::prelude::*;

use crate::states::pod::{PodState,ProviderState};
use super::starting::Starting;

#[derive(Default, Debug, TransitionTo)]
#[transition_to(Starting)]
pub struct Initializing;

#[async_trait::async_trait]
impl State<PodState> for Initializing {
    async fn next(
        self: Box<Self>,
        _provider_state: SharedState<ProviderState>,
        _pod_state: &mut PodState,
        pod: Manifest<Pod>,
    ) -> Transition<PodState> {
        let pod = pod.latest();

        // Execute any initContainers in the pod spec
        for _init_container in pod.init_containers() { }

        Transition::next(self, Starting)
    }

    async fn status(&self, _pod_state: &mut PodState, _pod: &Pod) -> anyhow::Result<PodStatus> {
        Ok(make_status(Phase::Succeeded, "Initializing"))
    } 
}
