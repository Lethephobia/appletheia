use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{AccountId, AccountName, AccountOwner, AccountStateError, AccountStatus};

/// Stores the materialized state of an `Account` aggregate.
#[aggregate_state(error = AccountStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_value),
    entry(key = "owner_organization", value = owner_organization_value),
    entry(key = "currency", value = currency_value)
)]
pub struct AccountState {
    pub(super) id: AccountId,
    pub(super) owner: AccountOwner,
    pub(super) name: AccountName,
    pub(super) currency_id: CurrencyId,
    pub(super) balance: CurrencyAmount,
    pub(super) reserved_balance: CurrencyAmount,
    pub(super) status: AccountStatus,
}

impl AccountState {
    /// Creates a new account state.
    pub(super) fn new(
        id: AccountId,
        owner: AccountOwner,
        name: AccountName,
        currency_id: CurrencyId,
    ) -> Self {
        Self {
            id,
            owner,
            name,
            currency_id,
            balance: CurrencyAmount::zero(),
            reserved_balance: CurrencyAmount::zero(),
            status: AccountStatus::Active,
        }
    }
}

fn owner_user_value(
    state: &AccountState,
) -> Result<Option<banking_iam_domain::UserId>, AccountStateError> {
    Ok(state.owner.user_id().copied())
}

fn owner_organization_value(
    state: &AccountState,
) -> Result<Option<banking_iam_domain::OrganizationId>, AccountStateError> {
    Ok(state.owner.organization_id().copied())
}

fn currency_value(state: &AccountState) -> Result<Option<CurrencyId>, AccountStateError> {
    Ok(Some(state.currency_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, ReferenceIndexes, ReferenceValues};

    use banking_iam_domain::{OrganizationId, UserId};

    use crate::currency::CurrencyId;

    use super::{
        AccountId, AccountName, AccountOwner, AccountState, AccountStatus, CurrencyAmount,
    };

    fn account_name() -> AccountName {
        AccountName::try_from("main").expect("account name should be valid")
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = AccountId::new();
        let owner = AccountOwner::User(UserId::new());
        let state = AccountState::new(id, owner, account_name(), CurrencyId::new());

        assert_eq!(state.id(), id);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn new_initializes_zero_balances_and_active_status() {
        let owner = AccountOwner::User(UserId::new());
        let state = AccountState::new(AccountId::new(), owner, account_name(), CurrencyId::new());

        assert_eq!(state.balance, CurrencyAmount::zero());
        assert_eq!(state.reserved_balance, CurrencyAmount::zero());
        assert_eq!(state.status, AccountStatus::Active);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn new_accepts_organization_owner() {
        let owner = AccountOwner::Organization(OrganizationId::new());
        let state = AccountState::new(AccountId::new(), owner, account_name(), CurrencyId::new());

        assert_eq!(state.owner, owner);
    }

    #[test]
    fn user_owned_account_returns_user_and_currency_reference_entries() {
        let state = AccountState::new(
            AccountId::new(),
            AccountOwner::User(UserId::new()),
            account_name(),
            CurrencyId::new(),
        );

        let entries = state
            .reference_entries()
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
        let state = AccountState::new(
            AccountId::new(),
            AccountOwner::Organization(OrganizationId::new()),
            account_name(),
            CurrencyId::new(),
        );

        let entries = state
            .reference_entries()
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
