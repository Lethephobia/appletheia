use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::{aggregate_state, unique_constraints};

use crate::core::{CurrencyDecimals, CurrencySymbol};

use super::{
    CurrencyDefinitionId, CurrencyDefinitionName, CurrencyDefinitionStateError,
    CurrencyDefinitionStatus,
};

/// Stores the materialized state of a `CurrencyDefinition` aggregate.
#[aggregate_state(error = CurrencyDefinitionStateError)]
#[unique_constraints(entry(key = "symbol", values = symbol_values))]
pub struct CurrencyDefinitionState {
    pub(super) id: CurrencyDefinitionId,
    pub(super) symbol: CurrencySymbol,
    pub(super) name: CurrencyDefinitionName,
    pub(super) decimals: CurrencyDecimals,
    pub(super) status: CurrencyDefinitionStatus,
}

impl CurrencyDefinitionState {
    /// Creates a new currency-definition state.
    pub(super) fn new(
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
            status: CurrencyDefinitionStatus::Active,
        }
    }
}

fn symbol_values(
    state: &CurrencyDefinitionState,
) -> Result<Option<UniqueValues>, CurrencyDefinitionStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let part = UniqueValuePart::try_from(state.symbol.as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use crate::core::{CurrencyDecimals, CurrencySymbol};

    use super::{
        CurrencyDefinitionId, CurrencyDefinitionName, CurrencyDefinitionState,
        CurrencyDefinitionStatus,
    };

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

    #[test]
    fn removed_state_has_no_symbol_unique_entry() {
        let mut state = CurrencyDefinitionState::new(
            CurrencyDefinitionId::new(),
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid"),
            CurrencyDecimals::new(6),
        );
        state.status = CurrencyDefinitionStatus::Removed;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UniqueKey::new("symbol")).map(UniqueValues::len),
            None
        );
    }
}
