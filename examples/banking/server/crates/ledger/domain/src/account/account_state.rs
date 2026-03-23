use appletheia::aggregate_state;
use appletheia::domain::UniqueConstraints;
use banking_iam_domain::UserId;

use crate::currency_definition::CurrencyDefinitionId;

use super::{AccountBalance, AccountError, AccountId, AccountStateError};

/// Stores the materialized state of an `Account` aggregate.
#[aggregate_state(error = AccountStateError)]
pub struct AccountState {
    id: AccountId,
    user_id: UserId,
    currency_definition_id: CurrencyDefinitionId,
    balance: AccountBalance,
    frozen: bool,
}

impl AccountState {
    /// Creates a new account state.
    pub fn new(
        id: AccountId,
        user_id: UserId,
        currency_definition_id: CurrencyDefinitionId,
    ) -> Self {
        Self {
            id,
            user_id,
            currency_definition_id,
            balance: AccountBalance::zero(),
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

    /// Returns whether the account is frozen.
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// Deposits balance into the account.
    pub fn deposit(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        let next_value = self
            .balance
            .value()
            .checked_add(amount.value())
            .ok_or(AccountError::BalanceOverflow)?;
        self.balance = AccountBalance::new(next_value);

        Ok(())
    }

    /// Withdraws balance from the account.
    pub fn withdraw(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        let next_value = self
            .balance
            .value()
            .checked_sub(amount.value())
            .ok_or(AccountError::InsufficientBalance)?;
        self.balance = AccountBalance::new(next_value);

        Ok(())
    }

    /// Marks the account as frozen.
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// Marks the account as thawed.
    pub fn thaw(&mut self) {
        self.frozen = false;
    }
}

impl UniqueConstraints<AccountStateError> for AccountState {}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateState;

    use banking_iam_domain::UserId;

    use crate::currency_definition::CurrencyDefinitionId;

    use super::{AccountId, AccountState};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = AccountId::new();
        let state = AccountState::new(id, UserId::new(), CurrencyDefinitionId::new());

        assert_eq!(state.id(), id);
    }
}
