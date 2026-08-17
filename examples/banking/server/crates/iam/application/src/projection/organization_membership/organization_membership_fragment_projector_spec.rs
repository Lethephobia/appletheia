use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{User, UserEventPayload};

/// Projector specification for organization membership fragments.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationMembershipFragmentProjectorSpec;

impl ProjectorSpec for OrganizationMembershipFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_membership_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANTED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_ROLES_CHANGED),
            EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_REMOVED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
        ]),
    );
}
