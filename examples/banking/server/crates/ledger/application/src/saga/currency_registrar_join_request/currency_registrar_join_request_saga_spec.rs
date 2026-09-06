use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestEventPayload,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipEventPayload,
};

/// Declares the descriptor for the currency registrar join request saga.
pub struct CurrencyRegistrarJoinRequestSagaSpec;

impl SagaSpec for CurrencyRegistrarJoinRequestSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_registrar_join_request"),
        SagaStartEvents::new(&[EventSelector::new::<CurrencyRegistrarJoinRequest>(
            CurrencyRegistrarJoinRequestEventPayload::APPROVED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<CurrencyRegistrarJoinRequest>(
                CurrencyRegistrarJoinRequestEventPayload::APPROVED,
            ),
            EventSelector::new::<CurrencyRegistrarMembership>(
                CurrencyRegistrarMembershipEventPayload::CREATED,
            ),
        ]),
    );
}
