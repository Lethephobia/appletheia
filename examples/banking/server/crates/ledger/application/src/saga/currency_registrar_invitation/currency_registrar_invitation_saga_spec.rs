use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::{
    CurrencyRegistrarInvitation, CurrencyRegistrarInvitationEventPayload,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipEventPayload,
};

use super::CurrencyRegistrarInvitationSagaState;

/// Declares the descriptor and state for the currency registrar invitation saga.
pub struct CurrencyRegistrarInvitationSagaSpec;

impl SagaSpec for CurrencyRegistrarInvitationSagaSpec {
    type State = CurrencyRegistrarInvitationSagaState;

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
            EventSelector::new::<CurrencyRegistrarMembership>(
                CurrencyRegistrarMembershipEventPayload::CREATE_REJECTED,
            ),
        ]),
    );
}
