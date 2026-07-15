use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use banking_iam_domain::{OrganizationId, UserId};
use uuid::Uuid;

use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{AccountName, AccountOwner, AccountStateError, AccountStatus};

/// Stores the materialized state of an `Account` aggregate.
#[aggregate_state(error = AccountStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_ref_value),
    entry(key = "owner_organization", value = owner_organization_ref_value),
    entry(key = "currency", value = currency_ref_value)
)]
pub struct AccountState {
    pub(super) owner: AccountOwner,
    pub(super) name: AccountName,
    pub(super) currency_id: CurrencyId,
    pub(super) balance: CurrencyAmount,
    pub(super) reserved_balance: CurrencyAmount,
    pub(super) status: AccountStatus,
}

fn owner_user_ref_value(
    state: &AccountState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, AccountStateError> {
    Ok(state.owner.user_id().copied())
}

fn owner_organization_ref_value(
    state: &AccountState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, AccountStateError> {
    Ok(state.owner.organization_id().copied())
}

fn currency_ref_value(
    state: &AccountState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyId>, AccountStateError> {
    Ok(Some(state.currency_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{ReferenceIndexes, ReferenceValues};
    use uuid::Uuid;

    use banking_iam_domain::{OrganizationId, UserId};

    use crate::currency::CurrencyId;

    use super::{AccountName, AccountOwner, AccountState, AccountStatus, CurrencyAmount};

    fn account_name() -> AccountName {
        AccountName::try_from("main").expect("account name should be valid")
    }

    #[test]
    fn state_stores_domain_attributes() {
        let owner = AccountOwner::User(UserId::new());
        let state = AccountState {
            owner,
            name: account_name(),
            currency_id: CurrencyId::new(),
            balance: CurrencyAmount::zero(),
            reserved_balance: CurrencyAmount::zero(),
            status: AccountStatus::Active,
        };
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn new_initializes_zero_balances_and_active_status() {
        let owner = AccountOwner::User(UserId::new());
        let state = AccountState {
            owner,
            name: account_name(),
            currency_id: CurrencyId::new(),
            balance: CurrencyAmount::zero(),
            reserved_balance: CurrencyAmount::zero(),
            status: AccountStatus::Active,
        };

        assert_eq!(state.balance, CurrencyAmount::zero());
        assert_eq!(state.reserved_balance, CurrencyAmount::zero());
        assert_eq!(state.status, AccountStatus::Active);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn new_accepts_organization_owner() {
        let owner = AccountOwner::Organization(OrganizationId::new());
        let state = AccountState {
            owner,
            name: account_name(),
            currency_id: CurrencyId::new(),
            balance: CurrencyAmount::zero(),
            reserved_balance: CurrencyAmount::zero(),
            status: AccountStatus::Active,
        };

        assert_eq!(state.owner, owner);
    }

    #[test]
    fn user_owned_account_returns_user_and_currency_reference_entries() {
        let state = AccountState {
            owner: AccountOwner::User(UserId::new()),
            name: account_name(),
            currency_id: CurrencyId::new(),
            balance: CurrencyAmount::zero(),
            reserved_balance: CurrencyAmount::zero(),
            status: AccountStatus::Active,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(AccountState::OWNER_USER_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(AccountState::OWNER_ORGANIZATION_REF)
                .map(ReferenceValues::len),
            None
        );
        assert_eq!(
            entries
                .get(AccountState::CURRENCY_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }

    #[test]
    fn organization_owned_account_returns_organization_and_currency_reference_entries() {
        let state = AccountState {
            owner: AccountOwner::Organization(OrganizationId::new()),
            name: account_name(),
            currency_id: CurrencyId::new(),
            balance: CurrencyAmount::zero(),
            reserved_balance: CurrencyAmount::zero(),
            status: AccountStatus::Active,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(AccountState::OWNER_USER_REF)
                .map(ReferenceValues::len),
            None
        );
        assert_eq!(
            entries
                .get(AccountState::OWNER_ORGANIZATION_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(AccountState::CURRENCY_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
