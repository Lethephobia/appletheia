use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_iam_domain::{OrganizationInvitation, OrganizationInvitationEventPayload};

use crate::command::OrganizationMembershipCreateCommand;

use super::{
    OrganizationInvitationAcceptedSagaError, OrganizationInvitationAcceptedSagaSpec,
    OrganizationInvitationSagaContext,
};

/// Coordinates organization invitation workflow into organization membership creation.
pub struct OrganizationInvitationAcceptedSaga;

impl Saga for OrganizationInvitationAcceptedSaga {
    type Spec = OrganizationInvitationAcceptedSagaSpec;
    type Context = OrganizationInvitationSagaContext;
    type EventAggregate = OrganizationInvitation;
    type Command = OrganizationMembershipCreateCommand;
    type Error = OrganizationInvitationAcceptedSagaError;

    fn on_event(
        &self,
        _context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let OrganizationInvitationEventPayload::Accepted {
            organization_id,
            invitee_id,
        } = event.payload()
        else {
            return Err(OrganizationInvitationAcceptedSagaError::UnexpectedEvent);
        };

        let context = OrganizationInvitationSagaContext::new(event.aggregate_id());

        let command = CommandRequest::new(OrganizationMembershipCreateCommand {
            organization_id: *organization_id,
            user_id: *invitee_id,
        });

        Ok(SagaTransition::new(context, command))
    }
}
