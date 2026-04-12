use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationInvitation, OrganizationInvitationEventPayload};

/// Declares the descriptor and state for the organization invitation saga.
pub struct OrganizationInvitationAcceptedSagaSpec;

impl SagaSpec for OrganizationInvitationAcceptedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_invitation_accepted"),
        EventSelector::new(
            OrganizationInvitation::TYPE,
            OrganizationInvitationEventPayload::ACCEPTED,
        ),
        SagaPredecessor::None,
    );
}
