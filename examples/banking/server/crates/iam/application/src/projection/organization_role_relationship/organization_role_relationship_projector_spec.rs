use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipEventPayload};

/// Declares the subscription for the organization role relationship projector.
pub struct OrganizationRoleRelationshipProjectorSpec;

impl ProjectorSpec for OrganizationRoleRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_role_relationship"),
        Subscription::AnyOf(&[
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
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::ROLE_GRANTED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::ROLE_REVOKED,
            ),
        ]),
    );
}
