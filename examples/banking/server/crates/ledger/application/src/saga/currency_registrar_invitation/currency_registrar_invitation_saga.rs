use crate::command::CurrencyRegistrarMembershipCreateCommand;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::{
    CurrencyRegistrarInvitation, CurrencyRegistrarInvitationEventPayload,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipEventPayload,
};

use super::{
    CurrencyRegistrarInvitationSagaError, CurrencyRegistrarInvitationSagaSpec,
    CurrencyRegistrarInvitationSagaState, CurrencyRegistrarInvitationSagaStep,
};

/// Coordinates the currency registrar invitation workflow into currency registrar membership creation.
pub struct CurrencyRegistrarInvitationSaga;

impl Saga for CurrencyRegistrarInvitationSaga {
    type Spec = CurrencyRegistrarInvitationSagaSpec;
    type Step = CurrencyRegistrarInvitationSagaStep;
    type Error = CurrencyRegistrarInvitationSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        _causative_step: Option<Self::Step>,
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
                    CausationId::from(event.event_id),
                    CurrencyRegistrarInvitationSagaStep::CreateMembership,
                    &CurrencyRegistrarMembershipCreateCommand {
                        currency_registrar_id: *currency_registrar_id,
                        user_id: *invitee_id,
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
