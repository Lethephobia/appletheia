use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    Organization, OrganizationEventPayload, OrganizationInvitation,
    OrganizationInvitationEventPayload,
};

/// Declares the subscription for the organization invitation organization relationship projector.
pub struct OrganizationInvitationOrganizationRelationshipProjectorSpec;

impl ProjectorSpec for OrganizationInvitationOrganizationRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("organization_invitation_organization_relationship"),
        Subscription::Only(&[
            EventSelector::new(
                OrganizationInvitation::TYPE,
                OrganizationInvitationEventPayload::ISSUED,
            ),
            EventSelector::new(
                OrganizationInvitation::TYPE,
                OrganizationInvitationEventPayload::ACCEPTED,
            ),
            EventSelector::new(
                OrganizationInvitation::TYPE,
                OrganizationInvitationEventPayload::DECLINED,
            ),
            EventSelector::new(
                OrganizationInvitation::TYPE,
                OrganizationInvitationEventPayload::CANCELED,
            ),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::REMOVED),
        ]),
    );
}
