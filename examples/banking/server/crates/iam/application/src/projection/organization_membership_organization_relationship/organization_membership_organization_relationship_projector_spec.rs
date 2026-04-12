use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipEventPayload};

/// Declares the subscription for the organization membership organization relationship projector.
pub struct OrganizationMembershipOrganizationRelationshipProjectorSpec;

impl ProjectorSpec for OrganizationMembershipOrganizationRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_membership_organization_relationship"),
        Subscription::AnyOf(&[
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::CREATED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::REMOVED,
            ),
        ]),
    );
}
