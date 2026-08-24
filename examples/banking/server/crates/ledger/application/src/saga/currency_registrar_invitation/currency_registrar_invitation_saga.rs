use crate::command::CurrencyRegistrarMembershipCreateCommand;
use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::{
    CurrencyRegistrarInvitation, CurrencyRegistrarInvitationEventPayload,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipCreateRejectionReason,
    CurrencyRegistrarMembershipEventPayload,
};

use super::{
    CurrencyRegistrarInvitationSagaError, CurrencyRegistrarInvitationSagaSpec,
    CurrencyRegistrarInvitationSagaState, CurrencyRegistrarInvitationSagaStatus,
};

/// Coordinates the currency registrar invitation workflow into currency registrar membership creation.
pub struct CurrencyRegistrarInvitationSaga;

impl Saga for CurrencyRegistrarInvitationSaga {
    type Spec = CurrencyRegistrarInvitationSagaSpec;
    type Error = CurrencyRegistrarInvitationSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<CurrencyRegistrarInvitation>() {
            let invitation_event = event.try_into_domain_event::<CurrencyRegistrarInvitation>()?;
            if let CurrencyRegistrarInvitationEventPayload::Accepted {
                currency_registrar_id,
                invitee_id,
            } = invitation_event.payload()
            {
                *instance.state_mut() = Some(CurrencyRegistrarInvitationSagaState::new(
                    invitation_event.aggregate_id(),
                ));

                instance.append_command(
                    event,
                    &CurrencyRegistrarMembershipCreateCommand {
                        currency_registrar_id: *currency_registrar_id,
                        user_id: *invitee_id,
                    },
                )?;
            }

            return Ok(());
        } else if event.is_for_aggregate::<CurrencyRegistrarMembership>() {
            let membership_event = event.try_into_domain_event::<CurrencyRegistrarMembership>()?;
            match membership_event.payload() {
                CurrencyRegistrarMembershipEventPayload::Created { .. } => {
                    instance.state_required_mut()?.status =
                        CurrencyRegistrarInvitationSagaStatus::MembershipCreated;
                    instance.succeed();
                }
                CurrencyRegistrarMembershipEventPayload::CreateRejected { reason, .. } => {
                    // A duplicate invitation delivery finds the pair already
                    // taken; that is a successful outcome for this workflow,
                    // not a failure to compensate.
                    if *reason == CurrencyRegistrarMembershipCreateRejectionReason::AlreadyMember {
                        instance.state_required_mut()?.status =
                            CurrencyRegistrarInvitationSagaStatus::AlreadyMember;
                        instance.succeed();
                    } else {
                        instance.state_required_mut()?.status =
                            CurrencyRegistrarInvitationSagaStatus::Failed;
                        instance.fail();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
