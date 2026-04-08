use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload,
};

use super::OrganizationInvitationSagaState;

/// Declares the descriptor and state for the organization invitation saga.
pub struct OrganizationInvitationSagaSpec;

impl SagaSpec for OrganizationInvitationSagaSpec {
    type State = OrganizationInvitationSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_invitation"),
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
