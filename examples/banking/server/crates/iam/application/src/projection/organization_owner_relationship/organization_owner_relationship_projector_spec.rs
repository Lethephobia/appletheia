use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::Organization;
use banking_iam_domain::OrganizationEventPayload;

/// Declares the subscription for the organization owner relationship projector.
pub struct OrganizationOwnerRelationshipProjectorSpec;

impl ProjectorSpec for OrganizationOwnerRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_owner_relationship"),
        Subscription::Only(&[
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::OWNER_ASSIGNED),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::OWNER_UNASSIGNED,
            ),
        ]),
    );
}
