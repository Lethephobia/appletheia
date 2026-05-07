use appletheia::domain::UniqueValue;
use appletheia::{aggregate_state, reference_indexes, unique_constraints};

use crate::core::CurrencyAmount;

use super::{
    CurrencyDecimals, CurrencyId, CurrencyName, CurrencyOwner, CurrencyStateError, CurrencyStatus,
    CurrencySymbol,
};

/// Stores the materialized state of a `Currency` aggregate.
#[aggregate_state(error = CurrencyStateError)]
#[unique_constraints(entry(key = "symbol", value = symbol_value))]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_value),
    entry(key = "owner_organization", value = owner_organization_value)
)]
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

fn symbol_value(state: &CurrencyState) -> Result<Option<UniqueValue>, CurrencyStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let value = UniqueValue::from_strings([state.symbol.as_ref()])?;

    Ok(Some(value))
}

fn owner_user_value(
    state: &CurrencyState,
) -> Result<Option<banking_iam_domain::UserId>, CurrencyStateError> {
    Ok(state.owner.user_id().copied())
}

fn owner_organization_value(
    state: &CurrencyState,
) -> Result<Option<banking_iam_domain::OrganizationId>, CurrencyStateError> {
    Ok(state.owner.organization_id().copied())
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{
        AggregateState, ReferenceIndexes, ReferenceValues, UniqueConstraints, UniqueKey,
        UniqueValues,
    };
    use banking_iam_domain::{OrganizationId, UserId};

    use crate::core::CurrencyAmount;

    use super::{
        CurrencyDecimals, CurrencyId, CurrencyName, CurrencyOwner, CurrencyState, CurrencyStatus,
        CurrencySymbol,
    };

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

    #[test]
    fn user_owned_currency_returns_user_reference_entry() {
        let state = CurrencyState::new(
            CurrencyId::new(),
            CurrencyOwner::User(UserId::new()),
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyName::try_from("USD Coin").expect("name should be valid"),
            CurrencyDecimals::new(6),
        );

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(CurrencyState::OWNER_USER_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(CurrencyState::OWNER_ORGANIZATION_REF)
                .map(ReferenceValues::len),
            None
        );
    }

    #[test]
    fn organization_owned_currency_returns_organization_reference_entry() {
        let state = CurrencyState::new(
            CurrencyId::new(),
            CurrencyOwner::Organization(OrganizationId::new()),
            CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            CurrencyName::try_from("USD Coin").expect("name should be valid"),
            CurrencyDecimals::new(6),
        );

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(CurrencyState::OWNER_USER_REF)
                .map(ReferenceValues::len),
            None
        );
        assert_eq!(
            entries
                .get(CurrencyState::OWNER_ORGANIZATION_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
