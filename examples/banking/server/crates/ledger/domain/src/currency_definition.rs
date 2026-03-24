mod currency_definition_error;
mod currency_definition_event_payload;
mod currency_definition_event_payload_error;
mod currency_definition_id;
mod currency_definition_name;
mod currency_definition_name_error;
mod currency_definition_state;
mod currency_definition_state_error;

pub use currency_definition_error::CurrencyDefinitionError;
pub use currency_definition_event_payload::CurrencyDefinitionEventPayload;
pub use currency_definition_event_payload_error::CurrencyDefinitionEventPayloadError;
pub use currency_definition_id::CurrencyDefinitionId;
pub use currency_definition_name::CurrencyDefinitionName;
pub use currency_definition_name_error::CurrencyDefinitionNameError;
pub use currency_definition_state::CurrencyDefinitionState;
pub use currency_definition_state_error::CurrencyDefinitionStateError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{CurrencyDecimals, CurrencySymbol};

/// Represents the `CurrencyDefinition` aggregate root.
#[aggregate(type = "currency_definition", error = CurrencyDefinitionError)]
pub struct CurrencyDefinition {
    core: AggregateCore<CurrencyDefinitionState, CurrencyDefinitionEventPayload>,
}

impl CurrencyDefinition {
    /// Returns the current symbol.
    pub fn symbol(&self) -> Result<&CurrencySymbol, CurrencyDefinitionError> {
        Ok(&self.state_required()?.symbol)
    }

    /// Returns the current name.
    pub fn name(&self) -> Result<&CurrencyDefinitionName, CurrencyDefinitionError> {
        Ok(&self.state_required()?.name)
    }

    /// Returns the current decimals.
    pub fn decimals(&self) -> Result<&CurrencyDecimals, CurrencyDefinitionError> {
        Ok(&self.state_required()?.decimals)
    }

    /// Returns whether the currency is active.
    pub fn is_active(&self) -> Result<bool, CurrencyDefinitionError> {
        Ok(self.state_required()?.active)
    }

    /// Defines a new currency.
    pub fn define(
        &mut self,
        symbol: CurrencySymbol,
        name: CurrencyDefinitionName,
        decimals: CurrencyDecimals,
    ) -> Result<(), CurrencyDefinitionError> {
        if self.state().is_some() {
            return Err(CurrencyDefinitionError::AlreadyDefined);
        }

        self.append_event(CurrencyDefinitionEventPayload::Defined {
            id: CurrencyDefinitionId::new(),
            symbol,
            name,
            decimals,
        })
    }

    /// Changes the current currency symbol.
    pub fn change_symbol(&mut self, symbol: CurrencySymbol) -> Result<(), CurrencyDefinitionError> {
        if self.state_required()?.symbol.eq(&symbol) {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::SymbolChanged { symbol })
    }

    /// Changes the current currency name.
    pub fn change_name(
        &mut self,
        name: CurrencyDefinitionName,
    ) -> Result<(), CurrencyDefinitionError> {
        if self.state_required()?.name.eq(&name) {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::NameChanged { name })
    }

    /// Activates the currency.
    pub fn activate(&mut self) -> Result<(), CurrencyDefinitionError> {
        if self.state_required()?.active {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::Activated)
    }

    /// Deactivates the currency.
    pub fn deactivate(&mut self) -> Result<(), CurrencyDefinitionError> {
        if !self.state_required()?.active {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::Deactivated)
    }
}

impl AggregateApply<CurrencyDefinitionEventPayload, CurrencyDefinitionError>
    for CurrencyDefinition
{
    fn apply(
        &mut self,
        payload: &CurrencyDefinitionEventPayload,
    ) -> Result<(), CurrencyDefinitionError> {
        match payload {
            CurrencyDefinitionEventPayload::Defined {
                id,
                symbol,
                name,
                decimals,
            } => {
                self.set_state(Some(CurrencyDefinitionState::new(
                    *id,
                    symbol.clone(),
                    name.clone(),
                    *decimals,
                )));
            }
            CurrencyDefinitionEventPayload::SymbolChanged { symbol } => {
                self.state_required_mut()?.symbol = symbol.clone();
            }
            CurrencyDefinitionEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone();
            }
            CurrencyDefinitionEventPayload::Activated => {
                self.state_required_mut()?.active = true;
            }
            CurrencyDefinitionEventPayload::Deactivated => {
                self.state_required_mut()?.active = false;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::core::{CurrencyDecimals, CurrencySymbol};

    use super::{
        CurrencyDefinition, CurrencyDefinitionEventPayload, CurrencyDefinitionId,
        CurrencyDefinitionName,
    };

    #[test]
    fn define_initializes_state_and_records_event() {
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid");
        let decimals = CurrencyDecimals::new(6);
        let mut currency_definition = CurrencyDefinition::default();

        currency_definition
            .define(symbol.clone(), name.clone(), decimals)
            .expect("definition should succeed");

        assert_eq!(
            currency_definition
                .aggregate_id()
                .expect("aggregate id should exist"),
            currency_definition
                .aggregate_id()
                .expect("aggregate id should exist")
        );
        assert_eq!(
            currency_definition.symbol().expect("symbol should exist"),
            &symbol
        );
        assert_eq!(
            currency_definition.name().expect("name should exist"),
            &name
        );
        assert_eq!(
            currency_definition
                .decimals()
                .expect("decimals should exist"),
            &decimals
        );
        assert!(
            currency_definition
                .is_active()
                .expect("active state should exist")
        );
        assert_eq!(currency_definition.uncommitted_events().len(), 1);
        assert_eq!(
            currency_definition.uncommitted_events()[0].payload().name(),
            CurrencyDefinitionEventPayload::DEFINED
        );
    }

    #[test]
    fn changing_to_same_values_and_same_status_is_a_no_op() {
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid");
        let decimals = CurrencyDecimals::new(6);
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(symbol.clone(), name.clone(), decimals)
            .expect("definition should succeed");

        currency_definition
            .change_symbol(symbol)
            .expect("no-op symbol change should succeed");
        currency_definition
            .change_name(name)
            .expect("no-op name change should succeed");
        currency_definition
            .activate()
            .expect("no-op activation should succeed");

        assert_eq!(currency_definition.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_methods_append_events_and_update_state() {
        let initial_symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let initial_name =
            CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid");
        let changed_symbol = CurrencySymbol::try_from("usdce").expect("symbol should be valid");
        let changed_name =
            CurrencyDefinitionName::try_from("USD Coin Example").expect("name should be valid");
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(initial_symbol, initial_name, CurrencyDecimals::new(6))
            .expect("definition should succeed");

        currency_definition
            .change_symbol(changed_symbol.clone())
            .expect("symbol change should succeed");
        currency_definition
            .change_name(changed_name.clone())
            .expect("name change should succeed");
        currency_definition
            .deactivate()
            .expect("deactivation should succeed");

        assert_eq!(
            currency_definition.symbol().expect("symbol should exist"),
            &changed_symbol
        );
        assert_eq!(
            currency_definition.name().expect("name should exist"),
            &changed_name
        );
        assert_eq!(
            currency_definition
                .decimals()
                .expect("decimals should exist"),
            &CurrencyDecimals::new(6)
        );
        assert!(
            !currency_definition
                .is_active()
                .expect("active state should exist")
        );
        assert_eq!(currency_definition.uncommitted_events().len(), 4);
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = CurrencyDefinitionId::new();
        let defined = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            CurrencyDefinitionEventPayload::Defined {
                id,
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
            },
        );
        let deactivated = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            CurrencyDefinitionEventPayload::Deactivated,
        );
        let mut currency_definition = CurrencyDefinition::default();

        currency_definition
            .replay_events(vec![defined, deactivated], None)
            .expect("events should replay");

        assert_eq!(
            currency_definition
                .symbol()
                .expect("symbol should exist")
                .value(),
            "USDC"
        );
        assert!(
            !currency_definition
                .is_active()
                .expect("active state should exist")
        );
        assert_eq!(currency_definition.version().value(), 2);
        assert!(currency_definition.uncommitted_events().is_empty());
    }

    #[test]
    fn define_rejects_already_defined_currency() {
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        let error = currency_definition
            .define(
                CurrencySymbol::try_from("sol").expect("symbol should be valid"),
                CurrencyDefinitionName::try_from("Solana").expect("name should be valid"),
                CurrencyDecimals::new(9),
            )
            .expect_err("duplicate definition should fail");

        assert!(matches!(
            error,
            super::CurrencyDefinitionError::AlreadyDefined
        ));
    }
}
