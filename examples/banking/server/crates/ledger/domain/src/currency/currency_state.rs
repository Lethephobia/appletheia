use appletheia::domain::UniqueValue;
use appletheia::{aggregate_state, reference_indexes, unique_constraints};
use banking_iam_domain::{OrganizationId, UserId};
use uuid::Uuid;

use crate::core::CurrencyAmount;

use super::{
    CurrencyDecimals, CurrencyDescription, CurrencyImageRef, CurrencyName, CurrencyOwner,
    CurrencyStateError, CurrencyStatus, CurrencySymbol, MintAccount,
};

/// Stores the materialized state of a `Currency` aggregate.
#[aggregate_state(error = CurrencyStateError)]
#[unique_constraints(entry(key = "symbol", value = symbol_unique_value))]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_ref_value),
    entry(key = "owner_organization", value = owner_organization_ref_value)
)]
pub struct CurrencyState {
    pub(super) owner: CurrencyOwner,
    pub(super) symbol: CurrencySymbol,
    pub(super) name: CurrencyName,
    pub(super) decimals: CurrencyDecimals,
    pub(super) description: Option<CurrencyDescription>,
    pub(super) image: Option<CurrencyImageRef>,
    pub(super) supply: CurrencyAmount,
    pub(super) pending_supply: CurrencyAmount,
    pub(super) mint_account: Option<MintAccount>,
    pub(super) status: CurrencyStatus,
}

fn symbol_unique_value(
    state: &CurrencyState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, CurrencyStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let value = UniqueValue::from_strings([state.symbol.as_ref()])?;

    Ok(Some(value))
}

fn owner_user_ref_value(
    state: &CurrencyState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, CurrencyStateError> {
    Ok(state.owner.user_id().copied())
}

fn owner_organization_ref_value(
    state: &CurrencyState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, CurrencyStateError> {
    Ok(state.owner.organization_id().copied())
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{
        ReferenceIndexes, ReferenceValues, UniqueConstraints, UniqueKey, UniqueValues,
    };
    use banking_iam_domain::{OrganizationId, UserId};
    use uuid::Uuid;

    use crate::core::CurrencyAmount;

    use super::{
        CurrencyDecimals, CurrencyName, CurrencyOwner, CurrencyState, CurrencyStatus,
        CurrencySymbol,
    };

    #[test]
    fn returns_unique_entries_for_symbol() {
        let state = CurrencyState {
            owner: CurrencyOwner::User(UserId::new()),
            symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
            decimals: CurrencyDecimals::new(6),
            description: None,
            image: None,
            supply: CurrencyAmount::zero(),
            pending_supply: CurrencyAmount::zero(),
            mint_account: None,
            status: CurrencyStatus::Provisioning,
        };

        let entries = state
            .unique_entries(Uuid::now_v7())
            .expect("unique entries should build");

        assert_eq!(
            entries.get(UniqueKey::new("symbol")).map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn state_stores_domain_attributes() {
        let state = CurrencyState {
            owner: CurrencyOwner::User(UserId::new()),
            symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
            decimals: CurrencyDecimals::new(6),
            description: None,
            image: None,
            supply: CurrencyAmount::zero(),
            pending_supply: CurrencyAmount::zero(),
            mint_account: None,
            status: CurrencyStatus::Provisioning,
        };
        assert_eq!(state.supply, CurrencyAmount::zero());
    }

    #[test]
    fn removed_state_has_no_symbol_unique_entry() {
        let mut state = CurrencyState {
            owner: CurrencyOwner::User(UserId::new()),
            symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
            decimals: CurrencyDecimals::new(6),
            description: None,
            image: None,
            supply: CurrencyAmount::zero(),
            pending_supply: CurrencyAmount::zero(),
            mint_account: None,
            status: CurrencyStatus::Provisioning,
        };
        state.status = CurrencyStatus::Removed;

        let entries = state
            .unique_entries(Uuid::now_v7())
            .expect("unique entries should build");

        assert_eq!(
            entries.get(UniqueKey::new("symbol")).map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn user_owned_currency_returns_user_reference_entry() {
        let state = CurrencyState {
            owner: CurrencyOwner::User(UserId::new()),
            symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
            decimals: CurrencyDecimals::new(6),
            description: None,
            image: None,
            supply: CurrencyAmount::zero(),
            pending_supply: CurrencyAmount::zero(),
            mint_account: None,
            status: CurrencyStatus::Provisioning,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
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
        let state = CurrencyState {
            owner: CurrencyOwner::Organization(OrganizationId::new()),
            symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
            decimals: CurrencyDecimals::new(6),
            description: None,
            image: None,
            supply: CurrencyAmount::zero(),
            pending_supply: CurrencyAmount::zero(),
            mint_account: None,
            status: CurrencyStatus::Provisioning,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
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
