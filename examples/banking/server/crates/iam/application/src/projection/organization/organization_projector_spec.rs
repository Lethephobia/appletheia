use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationEventPayload};

/// Declares the subscription for the organization view projector.
pub struct OrganizationProjectorSpec;

impl ProjectorSpec for OrganizationProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization"),
        Subscription::AnyOf(&[
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::CREATED),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::OWNERSHIP_TRANSFERRED,
            ),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::DISPLAY_NAME_CHANGED,
            ),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::DESCRIPTION_CHANGED,
            ),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::WEBSITE_URL_CHANGED,
            ),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::PICTURE_CHANGED,
            ),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::REMOVED),
        ]),
    );
}
