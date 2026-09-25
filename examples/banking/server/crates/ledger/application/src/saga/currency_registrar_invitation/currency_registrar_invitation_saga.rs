use crate::command::CurrencyRegistrarMembershipCreateCommand;
use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_ledger_domain::{CurrencyRegistrarInvitation, CurrencyRegistrarInvitationEventPayload};

use super::{
    CurrencyRegistrarInvitationSagaHandlerError, CurrencyRegistrarInvitationSagaState,
    CurrencyRegistrarInvitationSagaStep,
};

/// Coordinates the currency registrar invitation workflow into currency registrar membership creation.
pub struct CurrencyRegistrarInvitationSaga;

impl Saga for CurrencyRegistrarInvitationSaga {
    type State = CurrencyRegistrarInvitationSagaState;
    type Step = CurrencyRegistrarInvitationSagaStep;
    type HandlerError = CurrencyRegistrarInvitationSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "currency_registrar_invitation",
        ))
        .add_start_step(CurrencyRegistrarInvitationSagaStep::CreateMembership)
        .on::<CurrencyRegistrarInvitation>(CurrencyRegistrarInvitationEventPayload::ACCEPTED)
        .handle(|ctx, invitation_event| {
            if let CurrencyRegistrarInvitationEventPayload::Accepted {
                currency_registrar_id,
                invitee_id,
            } = invitation_event.payload()
            {
                ctx.set_state(CurrencyRegistrarInvitationSagaState::new(
                    invitation_event.aggregate_id(),
                ));

                ctx.append_command(&CurrencyRegistrarMembershipCreateCommand {
                    currency_registrar_id: *currency_registrar_id,
                    user_id: *invitee_id,
                })?;
            }
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}
