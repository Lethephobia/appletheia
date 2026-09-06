use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_iam_domain::{
    OrganizationInvitation, OrganizationInvitationEventPayload, OrganizationMembership,
    OrganizationMembershipEventPayload,
};


/// Declares the descriptor for the organization invitation saga.
pub struct OrganizationInvitationSagaSpec;

impl SagaSpec for OrganizationInvitationSagaSpec {

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("organization_invitation"),
        SagaStartEvents::new(&[EventSelector::new::<OrganizationInvitation>(
            OrganizationInvitationEventPayload::ACCEPTED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<OrganizationInvitation>(
                OrganizationInvitationEventPayload::ACCEPTED,
            ),
            EventSelector::new::<OrganizationMembership>(
                OrganizationMembershipEventPayload::CREATED,
            ),
        ]),
    );
}
