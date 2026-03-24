use appletheia::aggregate_state;
use appletheia::domain::UniqueConstraints;
use banking_iam_domain::UserId;

use crate::currency_definition::CurrencyDefinitionId;

use super::{AccountBalance, AccountBalanceError, AccountError, AccountId, AccountStateError};

/// Stores the materialized state of an `Account` aggregate.
#[aggregate_state(error = AccountStateError)]
pub struct AccountState {
    pub(super) id: AccountId,
    pub(super) user_id: UserId,
    pub(super) currency_definition_id: CurrencyDefinitionId,
    pub(super) balance: AccountBalance,
    pub(super) reserved_balance: AccountBalance,
    pub(super) frozen: bool,
}

impl AccountState {
    /// Creates a new account state.
    pub(super) fn new(
        id: AccountId,
        user_id: UserId,
        currency_definition_id: CurrencyDefinitionId,
    ) -> Self {
        Self {
            id,
            user_id,
            currency_definition_id,
            balance: AccountBalance::zero(),
            reserved_balance: AccountBalance::zero(),
            frozen: false,
        }
    }

    /// Returns the account owner.
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    /// Returns the currency definition referenced by the account.
    pub fn currency_definition_id(&self) -> &CurrencyDefinitionId {
        &self.currency_definition_id
    }

    /// Returns the current balance.
    pub fn balance(&self) -> &AccountBalance {
        &self.balance
    }

    /// Returns the current reserved balance.
    pub fn reserved_balance(&self) -> &AccountBalance {
        &self.reserved_balance
    }

    /// Returns the current available balance.
    pub fn available_balance(&self) -> Result<AccountBalance, AccountError> {
        self.balance
            .try_sub(self.reserved_balance)
            .map_err(|error| match error {
                AccountBalanceError::InsufficientBalance => AccountError::InvalidReservedBalance,
                AccountBalanceError::BalanceOverflow => AccountError::BalanceOverflow,
            })
    }

    /// Returns whether the account is frozen.
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }
}

impl UniqueConstraints<AccountStateError> for AccountState {}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateState;

    use banking_iam_domain::UserId;

    use crate::currency_definition::CurrencyDefinitionId;

    use super::{AccountBalance, AccountId, AccountState};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = AccountId::new();
        let state = AccountState::new(id, UserId::new(), CurrencyDefinitionId::new());

        assert_eq!(state.id(), id);
    }

    #[test]
    fn available_balance_excludes_reserved_balance() {
        let mut state =
            AccountState::new(AccountId::new(), UserId::new(), CurrencyDefinitionId::new());
        state.balance = AccountBalance::new(100);
        state.reserved_balance = AccountBalance::new(30);

        assert_eq!(state.balance(), &AccountBalance::new(100));
        assert_eq!(state.reserved_balance(), &AccountBalance::new(30));
        assert_eq!(
            state
                .available_balance()
                .expect("available balance should be valid"),
            AccountBalance::new(70)
        );
    }
}
