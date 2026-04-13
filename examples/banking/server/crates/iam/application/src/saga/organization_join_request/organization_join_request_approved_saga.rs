use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_iam_domain::{OrganizationJoinRequest, OrganizationJoinRequestEventPayload};

use crate::command::OrganizationMembershipCreateCommand;

use super::{
    OrganizationJoinRequestApprovedSagaError, OrganizationJoinRequestApprovedSagaSpec,
    OrganizationJoinRequestSagaContext,
};

/// Coordinates organization join request workflow into organization membership creation.
pub struct OrganizationJoinRequestApprovedSaga;

impl Saga for OrganizationJoinRequestApprovedSaga {
    type Spec = OrganizationJoinRequestApprovedSagaSpec;
    type Context = OrganizationJoinRequestSagaContext;
    type EventAggregate = OrganizationJoinRequest;
    type Command = OrganizationMembershipCreateCommand;
    type Error = OrganizationJoinRequestApprovedSagaError;

    fn on_event(
        &self,
        _context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let OrganizationJoinRequestEventPayload::Approved {
            organization_id,
            requester_id,
        } = event.payload()
        else {
            return Err(OrganizationJoinRequestApprovedSagaError::UnexpectedEvent);
        };

        let context = OrganizationJoinRequestSagaContext::new(event.aggregate_id());

        let command = CommandRequest::new(OrganizationMembershipCreateCommand {
            organization_id: *organization_id,
            user_id: *requester_id,
        });

        Ok(SagaTransition::new(context, command))
    }
}
