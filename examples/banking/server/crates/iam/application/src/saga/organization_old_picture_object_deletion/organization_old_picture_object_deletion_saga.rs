use appletheia::application::event::EventEnvelope;
use appletheia::application::request_context::CausationId;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload};

use crate::command::OrganizationPictureObjectDeleteCommand;

use super::{
    OrganizationOldPictureObjectDeletionSagaError, OrganizationOldPictureObjectDeletionSagaSpec,
    OrganizationOldPictureObjectDeletionSagaState, OrganizationOldPictureObjectDeletionSagaStep,
};

/// Coordinates old organization picture object deletion after picture changes.
pub struct OrganizationOldPictureObjectDeletionSaga;

impl Saga for OrganizationOldPictureObjectDeletionSaga {
    type Spec = OrganizationOldPictureObjectDeletionSagaSpec;
    type Step = OrganizationOldPictureObjectDeletionSagaStep;
    type Error = OrganizationOldPictureObjectDeletionSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        _step: Option<Self::Step>,
    ) -> Result<(), Self::Error> {
        let domain_event = event
            .try_into_domain_event::<Organization>()
            .map_err(|_| OrganizationOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        let OrganizationEventPayload::PictureChanged { old_picture, .. } = domain_event.payload()
        else {
            return Err(OrganizationOldPictureObjectDeletionSagaError::UnexpectedEvent);
        };

        let state = OrganizationOldPictureObjectDeletionSagaState::new(domain_event.aggregate_id());
        *instance.state_mut() = Some(state);
        let Some(object_name) = old_picture
            .as_ref()
            .and_then(|picture| picture.as_object_name())
            .cloned()
        else {
            instance.succeed();
            return Ok(());
        };

        instance
            .append_command(
                CausationId::from(event.event_id),
                OrganizationOldPictureObjectDeletionSagaStep::DeletePictureObject,
                &OrganizationPictureObjectDeleteCommand { object_name },
            )
            .map_err(|_| OrganizationOldPictureObjectDeletionSagaError::UnexpectedEvent)?;
        instance.succeed();

        Ok(())
    }
}
