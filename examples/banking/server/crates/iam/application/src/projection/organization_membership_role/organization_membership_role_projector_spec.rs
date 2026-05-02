use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipEventPayload};

/// Declares the subscription for the membership role view projector.
pub struct OrganizationMembershipRoleProjectorSpec;

impl ProjectorSpec for OrganizationMembershipRoleProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("membership_role"),
        Subscription::AnyOf(&[
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::ROLE_GRANTED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::ROLE_REVOKED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::REMOVED,
            ),
        ]),
    );
}
