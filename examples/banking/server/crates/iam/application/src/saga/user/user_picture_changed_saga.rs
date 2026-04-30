use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{User, UserEventPayload};

use crate::command::UserPictureObjectDeleteCommand;

use super::{UserPictureChangedSagaError, UserPictureChangedSagaSpec, UserPictureSagaState};

/// Coordinates user picture changes into old picture object deletion.
pub struct UserPictureChangedSaga;

impl Saga for UserPictureChangedSaga {
    type Spec = UserPictureChangedSagaSpec;
    type Error = UserPictureChangedSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope
            .try_into_domain_event::<User>()
            .map_err(|_| UserPictureChangedSagaError::UnexpectedEvent)?;
        let UserEventPayload::PictureChanged { old_picture, .. } = event.payload() else {
            return Err(UserPictureChangedSagaError::UnexpectedEvent);
        };

        let state = UserPictureSagaState::new(event.aggregate_id());
        *instance.state_mut() = Some(state);
        let Some(object_name) = old_picture
            .as_ref()
            .and_then(|picture| picture.as_object_name())
            .cloned()
        else {
            return Ok(());
        };

        instance
            .append_command(
                event_envelope,
                &UserPictureObjectDeleteCommand { object_name },
            )
            .map_err(|_| UserPictureChangedSagaError::UnexpectedEvent)?;

        Ok(())
    }
}
