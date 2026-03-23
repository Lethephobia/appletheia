use appletheia::aggregate_state;
use appletheia::domain::UniqueConstraints;
use banking_iam_domain::UserId;

use crate::currency_definition::CurrencyDefinitionId;

use super::{AccountBalance, AccountId, AccountStateError};

/// Stores the materialized state of an `Account` aggregate.
#[aggregate_state(error = AccountStateError)]
pub struct AccountState {
    id: AccountId,
    user_id: UserId,
    currency_definition_id: CurrencyDefinitionId,
    balance: AccountBalance,
    reserved_balance: AccountBalance,
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
    pub fn available_balance(&self) -> Result<AccountBalance, AccountStateError> {
        self.balance
            .try_sub(self.reserved_balance)
            .map_err(|error| match error {
                AccountStateError::InsufficientBalance => AccountStateError::InvalidReservedBalance,
                other => other,
            })
    }

    /// Returns whether the account is frozen.
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// Deposits balance into the account.
    pub fn deposit(&mut self, amount: AccountBalance) -> Result<(), AccountStateError> {
        self.balance = self.balance.try_add(amount)?;

        Ok(())
    }

    /// Withdraws balance from the account.
    pub fn withdraw(&mut self, amount: AccountBalance) -> Result<(), AccountStateError> {
        self.balance = self.balance.try_sub(amount)?;

        Ok(())
    }

    /// Reserves balance in the account.
    pub fn reserve_funds(&mut self, amount: AccountBalance) -> Result<(), AccountStateError> {
        if self.available_balance()?.value() < amount.value() {
            return Err(AccountStateError::InsufficientBalance);
        }

        self.reserved_balance = self.reserved_balance.try_add(amount)?;

        Ok(())
    }

    /// Releases reserved balance from the account.
    pub fn release_reserved_funds(
        &mut self,
        amount: AccountBalance,
    ) -> Result<(), AccountStateError> {
        self.reserved_balance =
            self.reserved_balance
                .try_sub(amount)
                .map_err(|error| match error {
                    AccountStateError::InsufficientBalance => {
                        AccountStateError::InsufficientReservedBalance
                    }
                    other => other,
                })?;

        Ok(())
    }

    /// Commits reserved balance and deducts it from the account.
    pub fn commit_reserved_funds(
        &mut self,
        amount: AccountBalance,
    ) -> Result<(), AccountStateError> {
        let next_reserved = self
            .reserved_balance
            .try_sub(amount)
            .map_err(|error| match error {
                AccountStateError::InsufficientBalance => {
                    AccountStateError::InsufficientReservedBalance
                }
                other => other,
            })?;
        let next_balance = self.balance.try_sub(amount)?;

        self.reserved_balance = next_reserved;
        self.balance = next_balance;

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
        state
            .deposit(AccountBalance::new(100))
            .expect("deposit should succeed");
        state
            .reserve_funds(AccountBalance::new(30))
            .expect("reserve should succeed");

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
