use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{User, UserEventPayload};

use crate::command::UserPictureObjectDeleteCommand;

use super::{
    UserOldPictureObjectDeletionSagaError, UserOldPictureObjectDeletionSagaSpec,
    UserOldPictureObjectDeletionSagaState, UserOldPictureObjectDeletionSagaStatus,
};

/// Coordinates old user picture object deletion after picture changes.
pub struct UserOldPictureObjectDeletionSaga;

impl Saga for UserOldPictureObjectDeletionSaga {
    type Spec = UserOldPictureObjectDeletionSagaSpec;
    type Error = UserOldPictureObjectDeletionSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope
            .try_into_domain_event::<User>()
            .map_err(|_| UserOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        let UserEventPayload::PictureChanged { old_picture, .. } = event.payload() else {
            return Err(UserOldPictureObjectDeletionSagaError::UnexpectedEvent);
        };

        let state = UserOldPictureObjectDeletionSagaState::new(event.aggregate_id());
        *instance.state_mut() = Some(state);
        let Some(object_name) = old_picture
            .as_ref()
            .and_then(|picture| picture.as_object_name())
            .cloned()
        else {
            instance.state_required_mut()?.status = UserOldPictureObjectDeletionSagaStatus::Skipped;
            instance.succeed();
            return Ok(());
        };

        instance
            .append_command(
                event_envelope,
                &UserPictureObjectDeleteCommand { object_name },
            )
            .map_err(|_| UserOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        instance.succeed();

        Ok(())
    }
}
