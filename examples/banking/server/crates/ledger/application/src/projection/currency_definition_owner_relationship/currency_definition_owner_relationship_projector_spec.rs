use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_definition::{
    CurrencyDefinition, CurrencyDefinitionEventPayload,
};

/// Declares the subscription for the currency-definition organization relationship projector.
pub struct CurrencyDefinitionOwnerRelationshipProjectorSpec;

impl ProjectorSpec for CurrencyDefinitionOwnerRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency_definition_owner_relationship"),
        Subscription::Only(&[EventSelector::new(
            CurrencyDefinition::TYPE,
            CurrencyDefinitionEventPayload::DEFINED,
        )]),
    );
}
