use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::{
    CurrencyRegistrarInvitation, CurrencyRegistrarInvitationEventPayload,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipEventPayload,
};

/// Declares the descriptor for the currency registrar invitation saga.
pub struct CurrencyRegistrarInvitationSagaSpec;

impl SagaSpec for CurrencyRegistrarInvitationSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_registrar_invitation"),
        SagaStartEvents::new(&[EventSelector::new::<CurrencyRegistrarInvitation>(
            CurrencyRegistrarInvitationEventPayload::ACCEPTED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<CurrencyRegistrarInvitation>(
                CurrencyRegistrarInvitationEventPayload::ACCEPTED,
            ),
            EventSelector::new::<CurrencyRegistrarMembership>(
                CurrencyRegistrarMembershipEventPayload::CREATED,
            ),
        ]),
    );
}
