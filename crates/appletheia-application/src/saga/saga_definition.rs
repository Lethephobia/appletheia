use appletheia_domain::AggregateType;

use crate::command::CommandName;
use crate::event::AppEvent;

use super::{SagaName, SagaOutcome, SagaState};

pub trait SagaDefinition: Send + Sync {
    type State: SagaState;
    type SagaName: SagaName;
    type AggregateType: AggregateType;

    const NAME: Self::SagaName;

    fn on_event(
        &self,
        state: &mut Option<Self::State>,
        event: &AppEvent<Self::AggregateType>,
    ) -> SagaOutcome<Self::CommandName>;

    type CommandName: CommandName;
}
