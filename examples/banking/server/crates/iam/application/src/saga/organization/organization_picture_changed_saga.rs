use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload};

use crate::command::OrganizationPictureObjectDeleteCommand;

use super::{
    OrganizationPictureChangedSagaError, OrganizationPictureChangedSagaSpec,
    OrganizationPictureSagaState,
};

/// Coordinates organization picture changes into old picture object deletion.
pub struct OrganizationPictureChangedSaga;

impl Saga for OrganizationPictureChangedSaga {
    type Spec = OrganizationPictureChangedSagaSpec;
    type Error = OrganizationPictureChangedSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope
            .try_into_domain_event::<Organization>()
            .map_err(|_| OrganizationPictureChangedSagaError::UnexpectedEvent)?;
        let OrganizationEventPayload::PictureChanged { old_picture, .. } = event.payload() else {
            return Err(OrganizationPictureChangedSagaError::UnexpectedEvent);
        };

        let state = OrganizationPictureSagaState::new(event.aggregate_id());
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
                &OrganizationPictureObjectDeleteCommand { object_name },
            )
            .map_err(|_| OrganizationPictureChangedSagaError::UnexpectedEvent)?;

        Ok(())
    }
}
