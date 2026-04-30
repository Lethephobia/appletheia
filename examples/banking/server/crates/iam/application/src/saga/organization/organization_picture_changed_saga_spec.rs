use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationEventPayload};

use super::OrganizationPictureSagaState;

/// Declares the descriptor and state for the organization picture saga.
pub struct OrganizationPictureChangedSagaSpec;

impl SagaSpec for OrganizationPictureChangedSagaSpec {
    type State = OrganizationPictureSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_picture_changed"),
        Subscription::One(&EventSelector::new(
            Organization::TYPE,
            OrganizationEventPayload::PICTURE_CHANGED,
        )),
    );
}
