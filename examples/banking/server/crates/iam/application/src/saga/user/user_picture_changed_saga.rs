use appletheia::application::command::CommandRequest;
use appletheia::application::saga::{Saga, SagaTransition};
use appletheia::domain::{Aggregate, Event};
use banking_iam_domain::{User, UserEventPayload};

use crate::command::UserPictureObjectDeleteCommand;

use super::{UserPictureChangedSagaError, UserPictureChangedSagaSpec, UserPictureSagaContext};

/// Coordinates user picture changes into old picture object deletion.
pub struct UserPictureChangedSaga;

impl Saga for UserPictureChangedSaga {
    type Spec = UserPictureChangedSagaSpec;
    type Context = UserPictureSagaContext;
    type EventAggregate = User;
    type Command = UserPictureObjectDeleteCommand;
    type Error = UserPictureChangedSagaError;

    fn on_event(
        &self,
        _context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error> {
        let UserEventPayload::PictureChanged { old_picture, .. } = event.payload() else {
            return Err(UserPictureChangedSagaError::UnexpectedEvent);
        };

        let context = UserPictureSagaContext::new(event.aggregate_id());
        let Some(object_name) = old_picture
            .as_ref()
            .and_then(|picture| picture.as_object_name())
            .cloned()
        else {
            return Ok(SagaTransition::no_command(context));
        };
        let command = CommandRequest::new(UserPictureObjectDeleteCommand { object_name });

        Ok(SagaTransition::new(context, command))
    }
}
