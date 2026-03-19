use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::{aggregate_state, unique_constraints};

use crate::core::{CurrencyDecimals, CurrencySymbol};

use super::{CurrencyDefinitionId, CurrencyDefinitionName, CurrencyDefinitionStateError};

/// Stores the materialized state of a `CurrencyDefinition` aggregate.
#[aggregate_state(error = CurrencyDefinitionStateError)]
#[unique_constraints(entry(key = "symbol", values = symbol_values))]
pub struct CurrencyDefinitionState {
    id: CurrencyDefinitionId,
    symbol: CurrencySymbol,
    name: CurrencyDefinitionName,
    decimals: CurrencyDecimals,
    active: bool,
}

impl CurrencyDefinitionState {
    /// Creates a new currency-definition state.
    pub fn new(
        id: CurrencyDefinitionId,
        symbol: CurrencySymbol,
        name: CurrencyDefinitionName,
        decimals: CurrencyDecimals,
    ) -> Self {
        Self {
            id,
            symbol,
            name,
            decimals,
            active: true,
        }
    }

    /// Returns the current symbol.
    pub fn symbol(&self) -> &CurrencySymbol {
        &self.symbol
    }

    /// Returns the current name.
    pub fn name(&self) -> &CurrencyDefinitionName {
        &self.name
    }

    /// Returns the current decimals.
    pub fn decimals(&self) -> &CurrencyDecimals {
        &self.decimals
    }

    /// Returns whether the currency is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Replaces the current symbol.
    pub fn set_symbol(&mut self, symbol: CurrencySymbol) {
        self.symbol = symbol;
    }

    /// Replaces the current name.
    pub fn set_name(&mut self, name: CurrencyDefinitionName) {
        self.name = name;
    }

    /// Replaces the current decimals.
    pub fn set_decimals(&mut self, decimals: CurrencyDecimals) {
        self.decimals = decimals;
    }

    /// Marks the currency as active.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Marks the currency as inactive.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

fn symbol_values(
    state: &CurrencyDefinitionState,
) -> Result<Option<UniqueValues>, CurrencyDefinitionStateError> {
    let part = UniqueValuePart::try_from(state.symbol().as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use crate::core::{CurrencyDecimals, CurrencySymbol};

    use super::{CurrencyDefinitionId, CurrencyDefinitionName, CurrencyDefinitionState};

    #[test]
    fn returns_unique_entries_for_symbol() {
        let state = CurrencyDefinitionState::new(
            CurrencyDefinitionId::new(),
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid"),
            CurrencyDecimals::new(6),
        );

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UniqueKey::new("symbol")).map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = CurrencyDefinitionId::new();
        let state = CurrencyDefinitionState::new(
            id,
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid"),
            CurrencyDecimals::new(6),
        );

        assert_eq!(state.id(), id);
    }
}
