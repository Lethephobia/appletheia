use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{
    OrganizationMembership, OrganizationMembershipEventPayload, User, UserEventPayload,
};

/// Projector specification for organization membership fragments.
///
/// Membership lifecycle now belongs to the `OrganizationMembership` aggregate.
/// `User::Removed` is still observed so the read model drops the memberships of
/// a removed user without waiting for aggregate-level cleanup.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationMembershipFragmentProjectorSpec;

impl ProjectorSpec for OrganizationMembershipFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_membership_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<OrganizationMembership>(
                OrganizationMembershipEventPayload::CREATED,
            ),
            EventSelector::new::<OrganizationMembership>(
                OrganizationMembershipEventPayload::ROLES_CHANGED,
            ),
            EventSelector::new::<OrganizationMembership>(
                OrganizationMembershipEventPayload::REMOVED,
            ),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
        ]),
    );
}
