use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};

/// Projector specification for organization-management information read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationManagementInfoProjectorSpec;

impl ProjectorSpec for OrganizationManagementInfoProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_management_info"),
        Subscription::AnyOf(&[
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DESCRIPTION_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::WEBSITE_URL_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
        ]),
    );
}
