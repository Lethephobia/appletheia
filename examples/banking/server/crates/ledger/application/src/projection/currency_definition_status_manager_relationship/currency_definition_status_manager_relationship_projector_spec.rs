use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_definition::{
    CurrencyDefinition, CurrencyDefinitionEventPayload,
};

/// Declares the subscription for the currency-definition organization relationship projector.
pub struct CurrencyDefinitionStatusManagerRelationshipProjectorSpec;

impl ProjectorSpec for CurrencyDefinitionStatusManagerRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency_definition_status_manager_relationship"),
        Subscription::Only(&[EventSelector::new(
            CurrencyDefinition::TYPE,
            CurrencyDefinitionEventPayload::DEFINED,
        )]),
    );
}
