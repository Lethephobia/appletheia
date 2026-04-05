use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload,
};

use super::OrganizationInvitationAcceptanceSagaState;

/// Declares the descriptor and state for the organization invitation acceptance saga.
pub struct OrganizationInvitationAcceptanceSagaSpec;

impl SagaSpec for OrganizationInvitationAcceptanceSagaSpec {
    type State = OrganizationInvitationAcceptanceSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_invitation_acceptance"),
        Subscription::Only(&[
            EventSelector::new(
                OrganizationInvitation::TYPE,
                OrganizationInvitationEventPayload::ACCEPTED,
            ),
            EventSelector::new(
                OrganizationMembership::TYPE,
                OrganizationMembershipEventPayload::CREATED,
            ),
        ]),
    );
}
