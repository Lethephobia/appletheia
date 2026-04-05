use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload,
};

/// Declares the subscription for the organization member relationship projector.
pub struct OrganizationMemberRelationshipProjectorSpec;

impl ProjectorSpec for OrganizationMemberRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_member_relationship"),
        Subscription::Only(&[
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::CREATED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::ACTIVATED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::INACTIVATED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::REMOVED,
            ),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::REMOVED),
        ]),
    );
}
