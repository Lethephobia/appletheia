use crate::command::CurrencyRegistrarMembershipCreateCommand;
use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestEventPayload,
};

use super::{
    CurrencyRegistrarJoinRequestSagaHandlerError, CurrencyRegistrarJoinRequestSagaState,
    CurrencyRegistrarJoinRequestSagaStep,
};

/// Coordinates the currency registrar join request workflow into currency registrar membership creation.
pub struct CurrencyRegistrarJoinRequestSaga;

impl Saga for CurrencyRegistrarJoinRequestSaga {
    type State = CurrencyRegistrarJoinRequestSagaState;
    type Step = CurrencyRegistrarJoinRequestSagaStep;
    type HandlerError = CurrencyRegistrarJoinRequestSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "currency_registrar_join_request",
        ))
        .add_start_step(CurrencyRegistrarJoinRequestSagaStep::CreateMembership)
        .on::<CurrencyRegistrarJoinRequest>(CurrencyRegistrarJoinRequestEventPayload::APPROVED)
        .handle(|ctx, join_request_event| {
            if let CurrencyRegistrarJoinRequestEventPayload::Approved {
                currency_registrar_id,
                requester_id,
            } = join_request_event.payload()
            {
                ctx.set_state(CurrencyRegistrarJoinRequestSagaState::new(
                    join_request_event.aggregate_id(),
                ));

                ctx.append_command(&CurrencyRegistrarMembershipCreateCommand {
                    currency_registrar_id: *currency_registrar_id,
                    user_id: *requester_id,
                })?;
            }
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}
