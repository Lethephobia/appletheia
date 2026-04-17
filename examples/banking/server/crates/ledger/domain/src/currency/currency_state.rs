use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::{aggregate_state, unique_constraints};

use crate::core::CurrencyAmount;
use crate::core::{CurrencyDecimals, CurrencySymbol};

use super::{CurrencyId, CurrencyName, CurrencyOwner, CurrencyStateError, CurrencyStatus};

/// Stores the materialized state of a `Currency` aggregate.
#[aggregate_state(error = CurrencyStateError)]
#[unique_constraints(entry(key = "symbol", values = symbol_values))]
pub struct CurrencyState {
    pub(super) id: CurrencyId,
    pub(super) owner: CurrencyOwner,
    pub(super) symbol: CurrencySymbol,
    pub(super) name: CurrencyName,
    pub(super) decimals: CurrencyDecimals,
    pub(super) supply: CurrencyAmount,
    pub(super) status: CurrencyStatus,
}

impl CurrencyState {
    /// Creates a new currency state.
    pub(super) fn new(
        id: CurrencyId,
        owner: CurrencyOwner,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
    ) -> Self {
        Self {
            id,
            owner,
            symbol,
            name,
            decimals,
            supply: CurrencyAmount::zero(),
            status: CurrencyStatus::Active,
        }
    }
}

fn symbol_values(state: &CurrencyState) -> Result<Option<UniqueValues>, CurrencyStateError> {
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
    use banking_iam_domain::UserId;

    use crate::core::CurrencyAmount;
    use crate::core::{CurrencyDecimals, CurrencySymbol};

    use super::{CurrencyId, CurrencyName, CurrencyOwner, CurrencyState, CurrencyStatus};

    #[test]
    fn returns_unique_entries_for_symbol() {
        let state = CurrencyState::new(
            CurrencyId::new(),
            CurrencyOwner::User(UserId::new()),
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyName::try_from("USD Coin").expect("name should be valid"),
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
        let id = CurrencyId::new();
        let state = CurrencyState::new(
            id,
            CurrencyOwner::User(UserId::new()),
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyName::try_from("USD Coin").expect("name should be valid"),
            CurrencyDecimals::new(6),
        );

        assert_eq!(state.id(), id);
        assert_eq!(state.supply, CurrencyAmount::zero());
    }

    #[test]
    fn removed_state_has_no_symbol_unique_entry() {
        let mut state = CurrencyState::new(
            CurrencyId::new(),
            CurrencyOwner::User(UserId::new()),
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyName::try_from("USD Coin").expect("name should be valid"),
            CurrencyDecimals::new(6),
        );
        state.status = CurrencyStatus::Removed;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UniqueKey::new("symbol")).map(UniqueValues::len),
            None
        );
    }
}
