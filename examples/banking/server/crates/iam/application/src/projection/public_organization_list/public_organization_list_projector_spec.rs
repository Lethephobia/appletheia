use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload};

/// Projector specification for public organization list read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PublicOrganizationListProjectorSpec;

impl ProjectorSpec for PublicOrganizationListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("public_organization_list"),
        Subscription::AnyOf(&[
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
        ]),
    );
}
