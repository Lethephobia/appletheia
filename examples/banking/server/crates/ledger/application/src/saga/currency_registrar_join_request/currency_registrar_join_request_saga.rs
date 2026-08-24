use crate::command::CurrencyRegistrarMembershipCreateCommand;
use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestEventPayload,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipCreateRejectionReason,
    CurrencyRegistrarMembershipEventPayload,
};

use super::{
    CurrencyRegistrarJoinRequestSagaError, CurrencyRegistrarJoinRequestSagaSpec,
    CurrencyRegistrarJoinRequestSagaState, CurrencyRegistrarJoinRequestSagaStatus,
};

/// Coordinates the currency registrar join request workflow into currency registrar membership creation.
pub struct CurrencyRegistrarJoinRequestSaga;

impl Saga for CurrencyRegistrarJoinRequestSaga {
    type Spec = CurrencyRegistrarJoinRequestSagaSpec;
    type Error = CurrencyRegistrarJoinRequestSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
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
                    event,
                    &CurrencyRegistrarMembershipCreateCommand {
                        currency_registrar_id: *currency_registrar_id,
                        user_id: *requester_id,
                    },
                )?;
            }

            return Ok(());
        } else if event.is_for_aggregate::<CurrencyRegistrarMembership>() {
            let membership_event = event.try_into_domain_event::<CurrencyRegistrarMembership>()?;
            match membership_event.payload() {
                CurrencyRegistrarMembershipEventPayload::Created { .. } => {
                    instance.state_required_mut()?.status =
                        CurrencyRegistrarJoinRequestSagaStatus::MembershipCreated;
                    instance.succeed();
                }
                CurrencyRegistrarMembershipEventPayload::CreateRejected { reason, .. } => {
                    // A duplicate approval delivery finds the pair already
                    // taken; that is a successful outcome for this workflow,
                    // not a failure to compensate.
                    if *reason == CurrencyRegistrarMembershipCreateRejectionReason::AlreadyMember {
                        instance.state_required_mut()?.status =
                            CurrencyRegistrarJoinRequestSagaStatus::AlreadyMember;
                        instance.succeed();
                    } else {
                        instance.state_required_mut()?.status =
                            CurrencyRegistrarJoinRequestSagaStatus::Failed;
                        instance.fail();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
