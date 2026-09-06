use appletheia::application::command::CommandFailureEnvelope;
use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance};
use banking_iam_domain::{User, UserEventPayload};

use crate::command::UserPictureObjectDeleteCommand;

use super::{
    UserOldPictureObjectDeletionSagaError, UserOldPictureObjectDeletionSagaSpec,
    UserOldPictureObjectDeletionSagaState, UserOldPictureObjectDeletionSagaStep,
};

/// Coordinates old user picture object deletion after picture changes.
pub struct UserOldPictureObjectDeletionSaga;

impl Saga for UserOldPictureObjectDeletionSaga {
    type Spec = UserOldPictureObjectDeletionSagaSpec;
    type State = UserOldPictureObjectDeletionSagaState;
    type Step = UserOldPictureObjectDeletionSagaStep;
    type Error = UserOldPictureObjectDeletionSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<Self::State, Self::Step>,
        event: &EventEnvelope,
        _causative_step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        let domain_event = event
            .try_into_domain_event::<User>()
            .map_err(|_| UserOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        let UserEventPayload::PictureChanged { old_picture, .. } = domain_event.payload() else {
            return Err(UserOldPictureObjectDeletionSagaError::UnexpectedEvent);
        };

        let state = UserOldPictureObjectDeletionSagaState::new(domain_event.aggregate_id());
        *instance.state_mut() = Some(state);
        let Some(object_name) = old_picture
            .as_ref()
            .and_then(|picture| picture.as_object_name())
            .cloned()
        else {
            instance.complete();
            return Ok(());
        };

        instance
            .append_command(
                CausationId::from(event.event_id),
                UserOldPictureObjectDeletionSagaStep::DeletePictureObject,
                &UserPictureObjectDeleteCommand { object_name },
            )
            .map_err(|_| UserOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        instance.complete();

        Ok(())
    }

    fn on_command_failed(
        &self,
        instance: &mut SagaInstance<Self::State, Self::Step>,
        _failure: &CommandFailureEnvelope,
        _causative_step: Self::Step,
    ) -> Result<(), Self::Error> {
        instance.complete();
        Ok(())
    }
}
