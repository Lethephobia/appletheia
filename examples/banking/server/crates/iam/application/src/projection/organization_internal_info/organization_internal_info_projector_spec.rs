use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload};

/// Projector specification for organization-internal information read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationInternalInfoProjectorSpec;

impl ProjectorSpec for OrganizationInternalInfoProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_internal_info"),
        Subscription::AnyOf(&[
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DESCRIPTION_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::WEBSITE_URL_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
        ]),
    );
}
