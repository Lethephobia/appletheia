use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Declares the subscription for the currency organization relationship projector.
pub struct CurrencyOwnerRelationshipProjectorSpec;

impl ProjectorSpec for CurrencyOwnerRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency_owner_relationship"),
        Subscription::AnyOf(&[EventSelector::new(
            Currency::TYPE,
            CurrencyEventPayload::DEFINED,
        )]),
    );
}
