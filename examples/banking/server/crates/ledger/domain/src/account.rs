mod account_balance;
mod account_balance_error;
mod account_error;
mod account_event_payload;
mod account_event_payload_error;
mod account_id;
mod account_state;
mod account_state_error;
mod account_status;

pub use account_balance::AccountBalance;
pub use account_balance_error::AccountBalanceError;
pub use account_error::AccountError;
pub use account_event_payload::AccountEventPayload;
pub use account_event_payload_error::AccountEventPayloadError;
pub use account_id::AccountId;
pub use account_state::AccountState;
pub use account_state_error::AccountStateError;
pub use account_status::AccountStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};
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

    /// Returns the current account status.
    pub fn status(&self) -> Result<AccountStatus, AccountError> {
        Ok(self.state_required()?.status)
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
        Ok(self.state_required()?.status.is_frozen())
    }

    /// Returns whether the account is closed.
    pub fn is_closed(&self) -> Result<bool, AccountError> {
        Ok(self.state_required()?.status.is_closed())
    }

    /// Opens a new account.
    pub fn open(
        &mut self,
        user_id: UserId,
        currency_definition_id: CurrencyDefinitionId,
    ) -> Result<(), AccountError> {
        self.ensure_not_opened()?;
        self.append_event(AccountEventPayload::Opened {
            id: AccountId::new(),
            user_id,
            currency_definition_id,
        })
    }

    /// Freezes the account.
    pub fn freeze(&mut self) -> Result<(), AccountError> {
        if self.state_required()?.status.is_frozen() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Frozen)
    }

    /// Thaws the account.
    pub fn thaw(&mut self) -> Result<(), AccountError> {
        if self.state_required()?.status.is_active() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Thawed)
    }

    /// Closes the account permanently.
    pub fn close(&mut self) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Closed)
    }

    /// Deposits balance into the account.
    pub fn deposit(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Deposited { amount })
    }

    /// Withdraws balance from the account.
    pub fn withdraw(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Withdrawn { amount })
    }

    /// Reserves funds in the account.
    pub fn reserve_funds(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::FundsReserved { amount })
    }

    /// Releases reserved funds in the account.
    pub fn release_reserved_funds(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::ReservedFundsReleased { amount })
    }

    /// Commits reserved funds and deducts them from the account.
    pub fn commit_reserved_funds(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::ReservedFundsCommitted { amount })
    }

    fn ensure_not_opened(&self) -> Result<(), AccountError> {
        if self.state().is_some() {
            return Err(AccountError::AlreadyOpened);
        }

        Ok(())
    }

    fn ensure_active_status(&self) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => Ok(()),
            AccountStatus::Frozen => Err(AccountError::Frozen),
            AccountStatus::Closed => Err(AccountError::Closed),
        }
    }

    fn ensure_not_closed(&self) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Err(AccountError::Closed);
        }

        Ok(())
    }

    fn ensure_zero_balances_for_close(&self) -> Result<(), AccountError> {
        let state = self.state_required()?;

        if !state.reserved_balance.is_zero() {
            return Err(AccountError::ReservedBalanceRemaining);
        }

        if !state.balance.is_zero() {
            return Err(AccountError::BalanceRemaining);
        }

        Ok(())
    }

    fn ensure_available_balance_at_least(
        &self,
        amount: AccountBalance,
        error: AccountError,
    ) -> Result<(), AccountError> {
        if self.available_balance()?.value() < amount.value() {
            return Err(error);
        }

        Ok(())
    }

    fn ensure_reserved_balance_at_least(&self, amount: AccountBalance) -> Result<(), AccountError> {
        if self.state_required()?.reserved_balance.value() < amount.value() {
            return Err(AccountError::InsufficientReservedBalance);
        }

        Ok(())
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
                self.ensure_not_opened()?;
                self.set_state(Some(AccountState::new(
                    *id,
                    *user_id,
                    *currency_definition_id,
                )));
            }
            AccountEventPayload::Frozen => match self.state_required()?.status {
                AccountStatus::Active => self.state_required_mut()?.status = AccountStatus::Frozen,
                AccountStatus::Frozen => {}
                AccountStatus::Closed => return Err(AccountError::Closed),
            },
            AccountEventPayload::Thawed => match self.state_required()?.status {
                AccountStatus::Active => {}
                AccountStatus::Frozen => self.state_required_mut()?.status = AccountStatus::Active,
                AccountStatus::Closed => self.ensure_not_closed()?,
            },
            AccountEventPayload::Closed => {
                if self.state_required()?.status.is_closed() {
                    return Ok(());
                }

                self.ensure_zero_balances_for_close()?;
                self.state_required_mut()?.status = AccountStatus::Closed;
            }
            AccountEventPayload::Deposited { amount } => {
                self.ensure_active_status()?;
                let state = self.state_required_mut()?;
                state.balance = state.balance.try_add(*amount)?;
            }
            AccountEventPayload::Withdrawn { amount } => {
                self.ensure_active_status()?;
                self.ensure_available_balance_at_least(*amount, AccountError::InsufficientBalance)?;
                let state = self.state_required_mut()?;
                state.balance = state.balance.try_sub(*amount)?;
            }
            AccountEventPayload::FundsReserved { amount } => {
                self.ensure_active_status()?;
                self.ensure_available_balance_at_least(
                    *amount,
                    AccountError::InsufficientAvailableBalance,
                )?;
                let state = self.state_required_mut()?;
                state.reserved_balance = state.reserved_balance.try_add(*amount)?;
            }
            AccountEventPayload::ReservedFundsReleased { amount } => {
                self.ensure_active_status()?;
                self.ensure_reserved_balance_at_least(*amount)?;
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
                self.ensure_active_status()?;
                self.ensure_reserved_balance_at_least(*amount)?;
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
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use banking_iam_domain::UserId;

    use crate::currency_definition::CurrencyDefinitionId;

    use super::{Account, AccountBalance, AccountEventPayload, AccountId, AccountStatus};

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
        assert_eq!(
            account.status().expect("status should exist"),
            AccountStatus::Active
        );
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
    fn close_updates_state_to_closed() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");

        account.close().expect("close should succeed");

        assert!(account.is_closed().expect("closed state should exist"));
        assert_eq!(account.uncommitted_events().len(), 2);
        assert_eq!(
            account.uncommitted_events()[1].payload().name(),
            AccountEventPayload::CLOSED
        );
    }

    #[test]
    fn close_rejects_non_zero_balance() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(1))
            .expect("deposit should succeed");

        let error = account.close().expect_err("close should fail");

        assert!(matches!(error, super::AccountError::BalanceRemaining));
    }

    #[test]
    fn close_rejects_reserved_balance_remaining() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account
            .deposit(AccountBalance::new(1))
            .expect("deposit should succeed");
        account
            .reserve_funds(AccountBalance::new(1))
            .expect("reserve should succeed");

        let error = account.close().expect_err("close should fail");

        assert!(matches!(
            error,
            super::AccountError::ReservedBalanceRemaining
        ));
    }

    #[test]
    fn replay_rejects_events_after_closed() {
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
        let closed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            AccountEventPayload::Closed,
        );
        let deposited = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            AccountEventPayload::Deposited {
                amount: AccountBalance::new(1),
            },
        );
        let mut account = Account::default();

        let error = account
            .replay_events(vec![opened, closed, deposited], None)
            .expect_err("event after close should fail");

        assert!(matches!(error, super::AccountError::Closed));
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
        let deposited = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            AccountEventPayload::Deposited {
                amount: AccountBalance::new(100),
            },
        );
        let frozen = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            AccountEventPayload::Frozen,
        );
        let mut account = Account::default();

        account
            .replay_events(vec![opened, deposited, frozen], None)
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
    fn operations_reject_closed_account() {
        let mut account = Account::default();
        account
            .open(UserId::new(), CurrencyDefinitionId::new())
            .expect("open should succeed");
        account.close().expect("close should succeed");

        let freeze_error = account.freeze().expect_err("freeze should fail");
        let thaw_error = account.thaw().expect_err("thaw should fail");
        let deposit_error = account
            .deposit(AccountBalance::new(1))
            .expect_err("deposit should fail");
        let withdraw_error = account
            .withdraw(AccountBalance::new(1))
            .expect_err("withdraw should fail");
        let reserve_error = account
            .reserve_funds(AccountBalance::new(1))
            .expect_err("reserve should fail");
        let release_error = account
            .release_reserved_funds(AccountBalance::new(1))
            .expect_err("release should fail");
        let commit_error = account
            .commit_reserved_funds(AccountBalance::new(1))
            .expect_err("commit should fail");

        assert!(matches!(freeze_error, super::AccountError::Closed));
        assert!(matches!(thaw_error, super::AccountError::Closed));
        assert!(matches!(deposit_error, super::AccountError::Closed));
        assert!(matches!(withdraw_error, super::AccountError::Closed));
        assert!(matches!(reserve_error, super::AccountError::Closed));
        assert!(matches!(release_error, super::AccountError::Closed));
        assert!(matches!(commit_error, super::AccountError::Closed));
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
}
