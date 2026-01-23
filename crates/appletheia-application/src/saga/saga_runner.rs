use crate::command::CommandName;
use crate::event::AppEvent;
use crate::unit_of_work::UnitOfWork;
use appletheia_domain::AggregateType;

use super::{SagaDefinition, SagaName, SagaRunReport, SagaRunnerError};

#[allow(async_fn_in_trait)]
pub trait SagaRunner: Send + Sync {
    type Uow: UnitOfWork;
    type SagaName: SagaName;
    type AggregateType: AggregateType;
    type CommandName: CommandName;

    async fn handle_event<
        D: SagaDefinition<
                SagaName = Self::SagaName,
                AggregateType = Self::AggregateType,
                CommandName = Self::CommandName,
            >,
    >(
        &self,
        uow: &mut Self::Uow,
        saga: &D,
        event: &AppEvent<Self::AggregateType>,
    ) -> Result<SagaRunReport, SagaRunnerError>;
}
