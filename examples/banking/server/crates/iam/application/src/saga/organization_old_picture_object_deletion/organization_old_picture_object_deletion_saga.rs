use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload};

use crate::command::OrganizationPictureObjectDeleteCommand;

use super::{
    OrganizationOldPictureObjectDeletionSagaError, OrganizationOldPictureObjectDeletionSagaSpec,
    OrganizationOldPictureObjectDeletionSagaState, OrganizationOldPictureObjectDeletionSagaStatus,
};

/// Coordinates old organization picture object deletion after picture changes.
pub struct OrganizationOldPictureObjectDeletionSaga;

impl Saga for OrganizationOldPictureObjectDeletionSaga {
    type Spec = OrganizationOldPictureObjectDeletionSagaSpec;
    type Error = OrganizationOldPictureObjectDeletionSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope
            .try_into_domain_event::<Organization>()
            .map_err(|_| OrganizationOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        let OrganizationEventPayload::PictureChanged { old_picture, .. } = event.payload() else {
            return Err(OrganizationOldPictureObjectDeletionSagaError::UnexpectedEvent);
        };

        let state = OrganizationOldPictureObjectDeletionSagaState::new(event.aggregate_id());
        *instance.state_mut() = Some(state);
        let Some(object_name) = old_picture
            .as_ref()
            .and_then(|picture| picture.as_object_name())
            .cloned()
        else {
            if let Some(state) = instance.state_mut().as_mut() {
                state.status = OrganizationOldPictureObjectDeletionSagaStatus::Skipped;
            }
            instance.succeed();
            return Ok(());
        };

        instance
            .append_command(
                event_envelope,
                &OrganizationPictureObjectDeleteCommand { object_name },
            )
            .map_err(|_| OrganizationOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        instance.succeed();

        Ok(())
    }
}
