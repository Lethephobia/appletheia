use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_iam_domain::{Organization, OrganizationEventPayload};

use crate::command::OrganizationPictureObjectDeleteCommand;

use super::{
    OrganizationPictureChangedSagaError, OrganizationPictureChangedSagaSpec,
    OrganizationPictureSagaContext,
};

/// Coordinates organization picture changes into old picture object deletion.
pub struct OrganizationPictureChangedSaga;

impl Saga for OrganizationPictureChangedSaga {
    type Spec = OrganizationPictureChangedSagaSpec;
    type Context = OrganizationPictureSagaContext;
    type EventAggregate = Organization;
    type Command = OrganizationPictureObjectDeleteCommand;
    type Error = OrganizationPictureChangedSagaError;

    fn on_event(
        &self,
        _context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let OrganizationEventPayload::PictureChanged { old_picture, .. } = event.payload() else {
            return Err(OrganizationPictureChangedSagaError::UnexpectedEvent);
        };

        let context = OrganizationPictureSagaContext::new(event.aggregate_id());
        let Some(object_name) = old_picture
            .as_ref()
            .and_then(|picture| picture.as_object_name())
            .cloned()
        else {
            return Ok(SagaTransition::no_command(context));
        };
        let command = CommandRequest::new(OrganizationPictureObjectDeleteCommand { object_name });

        Ok(SagaTransition::new(context, command))
    }
}
