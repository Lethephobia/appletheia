use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_iam_domain::{Organization, OrganizationEventPayload};

use super::OrganizationOldPictureObjectDeletionSagaState;

/// Declares the descriptor and state for the organization old picture object deletion saga.
pub struct OrganizationOldPictureObjectDeletionSagaSpec;

impl SagaSpec for OrganizationOldPictureObjectDeletionSagaSpec {
    type State = OrganizationOldPictureObjectDeletionSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_old_picture_object_deletion"),
        SagaStartEvents::new(&[EventSelector::new::<Organization>(
            OrganizationEventPayload::PICTURE_CHANGED,
        )]),
        Subscription::One(&EventSelector::new::<Organization>(
            OrganizationEventPayload::PICTURE_CHANGED,
        )),
    );
}
