use crate::command::CurrencyRegistrarMembershipCreateCommand;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestEventPayload,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipEventPayload,
};

use super::{
    CurrencyRegistrarJoinRequestSagaError, CurrencyRegistrarJoinRequestSagaSpec,
    CurrencyRegistrarJoinRequestSagaState, CurrencyRegistrarJoinRequestSagaStep,
};

/// Coordinates the currency registrar join request workflow into currency registrar membership creation.
pub struct CurrencyRegistrarJoinRequestSaga;

impl Saga for CurrencyRegistrarJoinRequestSaga {
    type Spec = CurrencyRegistrarJoinRequestSagaSpec;
    type Step = CurrencyRegistrarJoinRequestSagaStep;
    type Error = CurrencyRegistrarJoinRequestSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        _causative_step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<CurrencyRegistrarJoinRequest>() {
            let join_request_event =
                event.try_into_domain_event::<CurrencyRegistrarJoinRequest>()?;
            if let CurrencyRegistrarJoinRequestEventPayload::Approved {
                currency_registrar_id,
                requester_id,
            } = join_request_event.payload()
            {
                *instance.state_mut() = Some(CurrencyRegistrarJoinRequestSagaState::new(
                    join_request_event.aggregate_id(),
                ));

                instance.append_command(
                    CausationId::from(event.event_id),
                    CurrencyRegistrarJoinRequestSagaStep::CreateMembership,
                    &CurrencyRegistrarMembershipCreateCommand {
                        currency_registrar_id: *currency_registrar_id,
                        user_id: *requester_id,
                    },
                )?;
            }

            return Ok(());
        } else if event.is_for_aggregate::<CurrencyRegistrarMembership>() {
            let membership_event = event.try_into_domain_event::<CurrencyRegistrarMembership>()?;
            if let CurrencyRegistrarMembershipEventPayload::Created { .. } =
                membership_event.payload()
            {
                instance.succeed();
            }
        }

        Ok(())
    }
}
