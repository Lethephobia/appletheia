mod account_balance;
mod account_balance_error;
mod account_error;
mod account_event_payload;
mod account_event_payload_error;
mod account_id;
mod account_state;
mod account_state_error;

pub use account_balance::AccountBalance;
pub use account_balance_error::AccountBalanceError;
pub use account_error::AccountError;
pub use account_event_payload::AccountEventPayload;
pub use account_event_payload_error::AccountEventPayloadError;
pub use account_id::AccountId;
pub use account_state::AccountState;
pub use account_state_error::AccountStateError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore, AggregateState};
use banking_iam_domain::UserId;

use crate::currency_definition::CurrencyDefinitionId;

/// Represents the `Account` aggregate root.
#[aggregate(type = "account", error = AccountError)]
pub struct Account {
    core: AggregateCore<AccountState, AccountEventPayload>,
}

impl Account {
    /// Returns the account owner.
    pub fn user_id(&self) -> Result<&UserId, AccountError> {
        Ok(&self.state_required()?.user_id)
    }

    /// Returns the currency definition referenced by the account.
    pub fn currency_definition_id(&self) -> Result<&CurrencyDefinitionId, AccountError> {
        Ok(&self.state_required()?.currency_definition_id)
    }

    /// Returns the current balance.
    pub fn balance(&self) -> Result<&AccountBalance, AccountError> {
        Ok(&self.state_required()?.balance)
    }

    /// Returns the current reserved balance.
    pub fn reserved_balance(&self) -> Result<&AccountBalance, AccountError> {
        Ok(&self.state_required()?.reserved_balance)
    }

    /// Returns the current available balance.
    pub fn available_balance(&self) -> Result<AccountBalance, AccountError> {
        let state = self.state_required()?;

        state
            .balance
            .try_sub(state.reserved_balance)
            .map_err(|error| match error {
                AccountBalanceError::InsufficientBalance => AccountError::InvalidReservedBalance,
                AccountBalanceError::BalanceOverflow => AccountError::BalanceOverflow,
            })
    }

    /// Returns whether the account is frozen.
    pub fn is_frozen(&self) -> Result<bool, AccountError> {
        Ok(self.state_required()?.frozen)
    }

    /// Opens a new account.
    pub fn open(
        &mut self,
        user_id: UserId,
        currency_definition_id: CurrencyDefinitionId,
    ) -> Result<(), AccountError> {
        if self.state().is_some() {
            return Err(AccountError::AlreadyOpened);
        }

        self.append_event(AccountEventPayload::Opened {
            id: AccountId::new(),
            user_id,
            currency_definition_id,
        })
    }

    /// Freezes the account.
    pub fn freeze(&mut self) -> Result<(), AccountError> {
        if self.state_required()?.frozen {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Frozen)
    }

    /// Thaws the account.
    pub fn thaw(&mut self) -> Result<(), AccountError> {
        if !self.state_required()?.frozen {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Thawed)
    }

    /// Deposits balance into the account.
    pub fn deposit(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        let state = self.state_required()?;

        if state.frozen {
            return Err(AccountError::Frozen);
        }

        let _ = state.balance.try_add(amount)?;

        self.append_event(AccountEventPayload::Deposited { amount })
    }

    /// Withdraws balance from the account.
    pub fn withdraw(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        let state = self.state_required()?;

        if state.frozen {
            return Err(AccountError::Frozen);
        }

        if self.available_balance()?.value() < amount.value() {
            return Err(AccountError::InsufficientBalance);
        }

        self.append_event(AccountEventPayload::Withdrawn { amount })
    }

    /// Reserves funds in the account.
    pub fn reserve_funds(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        let state = self.state_required()?;

        if state.frozen {
            return Err(AccountError::Frozen);
        }

        if self.available_balance()?.value() < amount.value() {
            return Err(AccountError::InsufficientAvailableBalance);
        }

        let _ = state.reserved_balance.try_add(amount)?;

        self.append_event(AccountEventPayload::FundsReserved { amount })
    }

    /// Releases reserved funds in the account.
    pub fn release_reserved_funds(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        let state = self.state_required()?;

        if state.frozen {
            return Err(AccountError::Frozen);
        }

        if state.reserved_balance.value() < amount.value() {
            return Err(AccountError::InsufficientReservedBalance);
        }

        self.append_event(AccountEventPayload::ReservedFundsReleased { amount })
    }

    /// Commits reserved funds and deducts them from the account.
    pub fn commit_reserved_funds(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        let state = self.state_required()?;

        if state.frozen {
            return Err(AccountError::Frozen);
        }

        if state.reserved_balance.value() < amount.value() {
            return Err(AccountError::InsufficientReservedBalance);
        }

        let _ = state.balance.try_sub(amount)?;

        self.append_event(AccountEventPayload::ReservedFundsCommitted { amount })
    }

    /// Requests a transfer from this account to another account.
    pub fn request_transfer(
        &mut self,
        to_account_id: AccountId,
        amount: AccountBalance,
    ) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Err(AccountError::ZeroTransferAmount);
        }

        let state = self.state_required()?;

        if state.frozen {
            return Err(AccountError::Frozen);
        }

        if state.id() == to_account_id {
            return Err(AccountError::SameTransferAccount);
        }

        if self.available_balance()?.value() < amount.value() {
            return Err(AccountError::InsufficientAvailableBalance);
        }

        self.append_event(AccountEventPayload::TransferRequested {
            to_account_id,
            amount,
        })
    }
}

impl AggregateApply<AccountEventPayload, AccountError> for Account {
    fn apply(&mut self, payload: &AccountEventPayload) -> Result<(), AccountError> {
        match payload {
            AccountEventPayload::Opened {
                id,
                user_id,
                currency_definition_id,
            } => {
                self.set_state(Some(AccountState::new(
                    *id,
                    *user_id,
                    *currency_definition_id,
                )));
            }
            AccountEventPayload::Frozen => {
                self.state_required_mut()?.frozen = true;
            }
            AccountEventPayload::Thawed => {
                self.state_required_mut()?.frozen = false;
            }
            AccountEventPayload::Deposited { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.try_add(*amount)?;
            }
            AccountEventPayload::Withdrawn { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.try_sub(*amount)?;
            }
            AccountEventPayload::FundsReserved { amount } => {
                let state = self.state_required_mut()?;
                state.reserved_balance = state.reserved_balance.try_add(*amount)?;
            }
            AccountEventPayload::ReservedFundsReleased { amount } => {
                let state = self.state_required_mut()?;
                state.reserved_balance =
                    state
                        .reserved_balance
                        .try_sub(*amount)
                        .map_err(|error| match error {
                            AccountBalanceError::InsufficientBalance => {
                                AccountError::InsufficientReservedBalance
                            }
                            AccountBalanceError::BalanceOverflow => AccountError::BalanceOverflow,
                        })?;
            }
            AccountEventPayload::ReservedFundsCommitted { amount } => {
                let state = self.state_required_mut()?;
                let next_reserved =
                    state
                        .reserved_balance
                        .try_sub(*amount)
                        .map_err(|error| match error {
                            AccountBalanceError::InsufficientBalance => {
                                AccountError::InsufficientReservedBalance
                            }
                            AccountBalanceError::BalanceOverflow => AccountError::BalanceOverflow,
                        })?;
                let next_balance = state.balance.try_sub(*amount)?;
                state.reserved_balance = next_reserved;
                state.balance = next_balance;
            }
            AccountEventPayload::TransferRequested { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use banking_iam_domain::UserId;

    use crate::currency_definition::CurrencyDefinitionId;

    use super::{Account, AccountBalance, AccountEventPayload, AccountId};

    #[test]
    fn open_initializes_state_and_records_event() {
        let user_id = UserId::new();
        let currency_definition_id = CurrencyDefinitionId::new();
        let mut account = Account::default();

        account
            .open(user_id, currency_definition_id)
            .expect("open should succeed");

        assert_eq!(
            account.aggregate_id().expect("aggregate id should exist"),
            account.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(account.user_id().expect("user id should exist"), &user_id);
        assert_eq!(
            account
                .currency_definition_id()
                .expect("currency definition id should exist"),
            &currency_definition_id
        );
        assert_eq!(
            account.balance().expect("balance should exist"),
            &AccountBalance::zero()
        );
        assert_eq!(
            account
                .reserved_balance()
                .expect("reserved balance should exist"),
            &AccountBalance::zero()
        );
        assert!(!account.is_frozen().expect("frozen state should exist"));
        assert_eq!(account.uncommitted_events().len(), 1);
        assert_eq!(
            account.uncommitted_events()[0].payload().name(),
            AccountEventPayload::OPENED
        );
    }

    #[test]
    fn changing_to_same_status_is_a_no_op() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        account.thaw().expect("no-op thaw should succeed");

        assert_eq!(account.uncommitted_events().len(), 1);
    }

    #[test]
    fn freeze_and_thaw_update_state() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        account.freeze().expect("freeze should succeed");
        account.thaw().expect("thaw should succeed");

        assert!(!account.is_frozen().expect("frozen state should exist"));
        assert_eq!(account.uncommitted_events().len(), 3);
        assert_eq!(
            account.uncommitted_events()[1].payload().name(),
            AccountEventPayload::FROZEN
        );
        assert_eq!(
            account.uncommitted_events()[2].payload().name(),
            AccountEventPayload::THAWED
        );
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = AccountId::new();
        let user_id = UserId::new();
        let currency_definition_id = CurrencyDefinitionId::new();
        let opened = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            AccountEventPayload::Opened {
                id,
                user_id,
                currency_definition_id,
            },
        );
        let frozen = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            AccountEventPayload::Frozen,
        );
        let deposited = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            AccountEventPayload::Deposited {
                amount: AccountBalance::new(100),
            },
        );
        let mut account = Account::default();

        account
            .replay_events(vec![opened, frozen, deposited], None)
            .expect("events should replay");

        assert_eq!(account.user_id().expect("user id should exist"), &user_id);
        assert_eq!(
            account
                .currency_definition_id()
                .expect("currency definition id should exist"),
            &currency_definition_id
        );
        assert!(account.is_frozen().expect("frozen state should exist"));
        assert_eq!(
            account.balance().expect("balance should exist"),
            &AccountBalance::new(100)
        );
        assert_eq!(account.version().value(), 3);
        assert!(account.uncommitted_events().is_empty());
    }

    #[test]
    fn open_rejects_already_opened_account() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        let error = account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect_err("duplicate open should fail");

        assert!(matches!(error, super::AccountError::AlreadyOpened));
    }

    #[test]
    fn deposit_and_withdraw_update_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        account
            .deposit(AccountBalance::new(150))
            .expect("deposit should succeed");
        account
            .withdraw(AccountBalance::new(40))
            .expect("withdraw should succeed");

        assert_eq!(
            account.balance().expect("balance should exist"),
            &AccountBalance::new(110)
        );
        assert_eq!(
            account
                .available_balance()
                .expect("available balance should be valid"),
            AccountBalance::new(110)
        );
        assert_eq!(account.uncommitted_events().len(), 3);
        assert_eq!(
            account.uncommitted_events()[1].payload().name(),
            AccountEventPayload::DEPOSITED
        );
        assert_eq!(
            account.uncommitted_events()[2].payload().name(),
            AccountEventPayload::WITHDRAWN
        );
    }

    #[test]
    fn withdraw_rejects_insufficient_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        let error = account
            .withdraw(AccountBalance::new(1))
            .expect_err("withdraw should fail");

        assert!(matches!(error, super::AccountError::InsufficientBalance));
    }

    #[test]
    fn movement_rejects_frozen_account() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account.freeze().expect("freeze should succeed");

        let deposit_error = account
            .deposit(AccountBalance::new(1))
            .expect_err("deposit should fail");
        let withdraw_error = account
            .withdraw(AccountBalance::new(1))
            .expect_err("withdraw should fail");

        assert!(matches!(deposit_error, super::AccountError::Frozen));
        assert!(matches!(withdraw_error, super::AccountError::Frozen));
    }

    #[test]
    fn reserve_release_and_commit_update_reserved_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(150))
            .expect("deposit should succeed");

        account
            .reserve_funds(AccountBalance::new(40))
            .expect("reserve should succeed");
        account
            .release_reserved_funds(AccountBalance::new(10))
            .expect("release should succeed");
        account
            .commit_reserved_funds(AccountBalance::new(20))
            .expect("commit should succeed");

        assert_eq!(
            account.balance().expect("balance should exist"),
            &AccountBalance::new(130)
        );
        assert_eq!(
            account
                .reserved_balance()
                .expect("reserved balance should exist"),
            &AccountBalance::new(10)
        );
        assert_eq!(
            account
                .available_balance()
                .expect("available balance should be valid"),
            AccountBalance::new(120)
        );
        assert_eq!(account.uncommitted_events().len(), 5);
        assert_eq!(
            account.uncommitted_events()[2].payload().name(),
            AccountEventPayload::FUNDS_RESERVED
        );
        assert_eq!(
            account.uncommitted_events()[3].payload().name(),
            AccountEventPayload::RESERVED_FUNDS_RELEASED
        );
        assert_eq!(
            account.uncommitted_events()[4].payload().name(),
            AccountEventPayload::RESERVED_FUNDS_COMMITTED
        );
    }

    #[test]
    fn reserve_rejects_insufficient_available_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(100))
            .expect("deposit should succeed");
        account
            .reserve_funds(AccountBalance::new(80))
            .expect("reserve should succeed");

        let error = account
            .reserve_funds(AccountBalance::new(30))
            .expect_err("reserve should fail");

        assert!(matches!(
            error,
            super::AccountError::InsufficientAvailableBalance
        ));
    }

    #[test]
    fn request_transfer_uses_available_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(100))
            .expect("deposit should succeed");
        account
            .reserve_funds(AccountBalance::new(80))
            .expect("reserve should succeed");

        let error = account
            .request_transfer(AccountId::new(), AccountBalance::new(30))
            .expect_err("transfer request should fail");

        assert!(matches!(
            error,
            super::AccountError::InsufficientAvailableBalance
        ));
    }

    #[test]
    fn request_transfer_records_event_without_changing_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(100))
            .expect("deposit should succeed");
        let to_account_id = AccountId::new();

        account
            .request_transfer(to_account_id, AccountBalance::new(40))
            .expect("transfer request should succeed");

        assert_eq!(
            account.balance().expect("balance should exist"),
            &AccountBalance::new(100)
        );
        assert_eq!(
            account
                .reserved_balance()
                .expect("reserved balance should exist"),
            &AccountBalance::zero()
        );
        assert_eq!(account.uncommitted_events().len(), 3);
        assert_eq!(
            account.uncommitted_events()[2].payload().name(),
            AccountEventPayload::TRANSFER_REQUESTED
        );
    }

    #[test]
    fn request_transfer_rejects_zero_amount() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        let error = account
            .request_transfer(AccountId::new(), AccountBalance::zero())
            .expect_err("zero transfer should fail");

        assert!(matches!(error, super::AccountError::ZeroTransferAmount));
    }

    #[test]
    fn request_transfer_rejects_same_account() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(100))
            .expect("deposit should succeed");
        let account_id = account.aggregate_id().expect("aggregate id should exist");

        let error = account
            .request_transfer(account_id, AccountBalance::new(1))
            .expect_err("same-account transfer should fail");

        assert!(matches!(error, super::AccountError::SameTransferAccount));
    }

    #[test]
    fn request_transfer_rejects_insufficient_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        let error = account
            .request_transfer(AccountId::new(), AccountBalance::new(1))
            .expect_err("transfer request should fail");

        assert!(matches!(
            error,
            super::AccountError::InsufficientAvailableBalance
        ));
    }

    #[test]
    fn request_transfer_rejects_frozen_account() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(100))
            .expect("deposit should succeed");
        account.freeze().expect("freeze should succeed");

        let error = account
            .request_transfer(AccountId::new(), AccountBalance::new(1))
            .expect_err("transfer request should fail");

        assert!(matches!(error, super::AccountError::Frozen));
    }
}
