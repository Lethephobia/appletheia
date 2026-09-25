use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{OrganizationInvitation, OrganizationInvitationEventPayload};

/// Projector specification for organization invitation fragments.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationInvitationFragmentProjectorSpec;

impl ProjectorSpec for OrganizationInvitationFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_invitation_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::ISSUED,
            ),
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::ACCEPTED,
            ),
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::DECLINED,
            ),
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::CANCELED,
            ),
        ]),
    );
}
