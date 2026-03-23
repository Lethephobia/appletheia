mod account_balance;
mod account_error;
mod account_event_payload;
mod account_event_payload_error;
mod account_id;
mod account_state;
mod account_state_error;

pub use account_balance::AccountBalance;
pub use account_error::AccountError;
pub use account_event_payload::AccountEventPayload;
pub use account_event_payload_error::AccountEventPayloadError;
pub use account_id::AccountId;
pub use account_state::AccountState;
pub use account_state_error::AccountStateError;

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
        if self.state_required()?.is_frozen() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Frozen)
    }

    /// Thaws the account.
    pub fn thaw(&mut self) -> Result<(), AccountError> {
        if !self.state_required()?.is_frozen() {
            return Ok(());
        }

        self.append_event(AccountEventPayload::Thawed)
    }

    /// Deposits balance into the account.
    pub fn deposit(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        if self.state_required()?.is_frozen() {
            return Err(AccountError::Frozen);
        }

        self.append_event(AccountEventPayload::Deposited { amount })
    }

    /// Withdraws balance from the account.
    pub fn withdraw(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
        if amount.is_zero() {
            return Ok(());
        }

        let state = self.state_required()?;

        if state.is_frozen() {
            return Err(AccountError::Frozen);
        }

        if state.balance().value() < amount.value() {
            return Err(AccountError::InsufficientBalance);
        }

        self.append_event(AccountEventPayload::Withdrawn { amount })
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
                self.state_required_mut()?.freeze();
            }
            AccountEventPayload::Thawed => {
                self.state_required_mut()?.thaw();
            }
            AccountEventPayload::Deposited { amount } => {
                self.state_required_mut()?.deposit(*amount)?;
            }
            AccountEventPayload::Withdrawn { amount } => {
                self.state_required_mut()?.withdraw(*amount)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateState, Event, EventPayload};

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

        let state = account.state().expect("state should exist");
        assert_eq!(
            state.id(),
            account.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(state.user_id(), &user_id);
        assert_eq!(state.currency_definition_id(), &currency_definition_id);
        assert_eq!(state.balance(), &AccountBalance::zero());
        assert!(!state.is_frozen());
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

        let state = account.state().expect("state should exist");
        assert!(!state.is_frozen());
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

        let state = account.state().expect("state should exist");
        assert_eq!(state.user_id(), &user_id);
        assert_eq!(state.currency_definition_id(), &currency_definition_id);
        assert!(state.is_frozen());
        assert_eq!(state.balance(), &AccountBalance::new(100));
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

        let state = account.state().expect("state should exist");
        assert_eq!(state.balance(), &AccountBalance::new(110));
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
}
