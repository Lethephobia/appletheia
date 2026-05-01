mod account_close_rejection_reason;
mod account_close_result;
mod account_deposit_rejection_reason;
mod account_deposit_result;
mod account_error;
mod account_event_payload;
mod account_event_payload_error;
mod account_freeze_rejection_reason;
mod account_freeze_result;
mod account_funds_reserve_rejection_reason;
mod account_funds_reserve_result;
mod account_id;
mod account_name;
mod account_name_change_rejection_reason;
mod account_name_change_result;
mod account_name_error;
mod account_owner;
mod account_ownership_transfer_rejection_reason;
mod account_ownership_transfer_result;
mod account_reserved_funds_commit_rejection_reason;
mod account_reserved_funds_commit_result;
mod account_reserved_funds_release_rejection_reason;
mod account_reserved_funds_release_result;
mod account_state;
mod account_state_error;
mod account_status;
mod account_thaw_rejection_reason;
mod account_thaw_result;
mod account_withdraw_rejection_reason;
mod account_withdraw_result;

pub use account_close_rejection_reason::AccountCloseRejectionReason;
pub use account_close_result::AccountCloseResult;
pub use account_deposit_rejection_reason::AccountDepositRejectionReason;
pub use account_deposit_result::AccountDepositResult;
pub use account_error::AccountError;
pub use account_event_payload::AccountEventPayload;
pub use account_event_payload_error::AccountEventPayloadError;
pub use account_freeze_rejection_reason::AccountFreezeRejectionReason;
pub use account_freeze_result::AccountFreezeResult;
pub use account_funds_reserve_rejection_reason::AccountFundsReserveRejectionReason;
pub use account_funds_reserve_result::AccountFundsReserveResult;
pub use account_id::AccountId;
pub use account_name::AccountName;
pub use account_name_change_rejection_reason::AccountNameChangeRejectionReason;
pub use account_name_change_result::AccountNameChangeResult;
pub use account_name_error::AccountNameError;
pub use account_owner::AccountOwner;
pub use account_ownership_transfer_rejection_reason::AccountOwnershipTransferRejectionReason;
pub use account_ownership_transfer_result::AccountOwnershipTransferResult;
pub use account_reserved_funds_commit_rejection_reason::AccountReservedFundsCommitRejectionReason;
pub use account_reserved_funds_commit_result::AccountReservedFundsCommitResult;
pub use account_reserved_funds_release_rejection_reason::AccountReservedFundsReleaseRejectionReason;
pub use account_reserved_funds_release_result::AccountReservedFundsReleaseResult;
pub use account_state::AccountState;
pub use account_state_error::AccountStateError;
pub use account_status::AccountStatus;
pub use account_thaw_rejection_reason::AccountThawRejectionReason;
pub use account_thaw_result::AccountThawResult;
pub use account_withdraw_rejection_reason::AccountWithdrawRejectionReason;
pub use account_withdraw_result::AccountWithdrawResult;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{CurrencyAmount, CurrencyAmountError};
use crate::currency::CurrencyId;

/// Represents the `Account` aggregate root.
#[aggregate(type = "account", error = AccountError)]
pub struct Account {
    core: AggregateCore<AccountState, AccountEventPayload>,
}

impl Account {
    /// Returns the account owner.
    pub fn owner(&self) -> Result<AccountOwner, AccountError> {
        Ok(self.state_required()?.owner)
    }

    /// Returns the account name.
    pub fn name(&self) -> Result<&AccountName, AccountError> {
        Ok(&self.state_required()?.name)
    }

    /// Returns the currency referenced by the account.
    pub fn currency_id(&self) -> Result<&CurrencyId, AccountError> {
        Ok(&self.state_required()?.currency_id)
    }

    /// Returns the current balance.
    pub fn balance(&self) -> Result<&CurrencyAmount, AccountError> {
        Ok(&self.state_required()?.balance)
    }

    /// Returns the current reserved balance.
    pub fn reserved_balance(&self) -> Result<&CurrencyAmount, AccountError> {
        Ok(&self.state_required()?.reserved_balance)
    }

    /// Returns the current available balance.
    pub fn available_balance(&self) -> Result<CurrencyAmount, AccountError> {
        let state = self.state_required()?;

        state
            .balance
            .try_sub(state.reserved_balance)
            .map_err(|error| match error {
                CurrencyAmountError::InsufficientBalance => AccountError::InvalidReservedBalance,
                CurrencyAmountError::BalanceOverflow => AccountError::BalanceOverflow,
            })
    }

    /// Returns the current account status.
    pub fn status(&self) -> Result<AccountStatus, AccountError> {
        Ok(self.state_required()?.status)
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
        owner: AccountOwner,
        name: AccountName,
        currency_id: CurrencyId,
    ) -> Result<(), AccountError> {
        if self.state().is_some() {
            return Err(AccountError::AlreadyOpened);
        }

        self.append_event(AccountEventPayload::Opened {
            id: AccountId::new(),
            owner,
            name,
            currency_id,
        })
    }

    /// Transfers ownership of the account.
    pub fn transfer_ownership(
        &mut self,
        owner: AccountOwner,
    ) -> Result<AccountOwnershipTransferResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountOwnershipTransferRejectionReason::Closed;
            self.append_event(AccountEventPayload::OwnershipTransferRejected { owner, reason })?;
            return Ok(AccountOwnershipTransferResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::OwnershipTransferred { owner })?;
        Ok(AccountOwnershipTransferResult::Transferred)
    }

    /// Changes the account name.
    pub fn change_name(
        &mut self,
        name: AccountName,
    ) -> Result<AccountNameChangeResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountNameChangeRejectionReason::Closed;
            self.append_event(AccountEventPayload::NameChangeRejected {
                name: name.clone(),
                reason,
            })?;
            return Ok(AccountNameChangeResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::NameChanged { name })?;
        Ok(AccountNameChangeResult::Changed)
    }

    /// Deposits balance into the account.
    pub fn deposit(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<AccountDepositResult, AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                let reason = AccountDepositRejectionReason::Frozen;
                self.append_event(AccountEventPayload::DepositRejected { amount, reason })?;

                return Ok(AccountDepositResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountDepositRejectionReason::Closed;
                self.append_event(AccountEventPayload::DepositRejected { amount, reason })?;

                return Ok(AccountDepositResult::Rejected { reason });
            }
        }

        self.append_event(AccountEventPayload::Deposited { amount })?;

        Ok(AccountDepositResult::Deposited)
    }

    /// Withdraws balance from the account.
    pub fn withdraw(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<AccountWithdrawResult, AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                let reason = AccountWithdrawRejectionReason::Frozen;
                self.append_event(AccountEventPayload::WithdrawRejected { amount, reason })?;
                return Ok(AccountWithdrawResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountWithdrawRejectionReason::Closed;
                self.append_event(AccountEventPayload::WithdrawRejected { amount, reason })?;
                return Ok(AccountWithdrawResult::Rejected { reason });
            }
        }

        if self.available_balance()?.value() < amount.value() {
            let reason = AccountWithdrawRejectionReason::InsufficientBalance;
            self.append_event(AccountEventPayload::WithdrawRejected { amount, reason })?;
            return Ok(AccountWithdrawResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Withdrawn { amount })?;
        Ok(AccountWithdrawResult::Withdrawn)
    }

    /// Reserves funds in the account.
    pub fn reserve_funds(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<AccountFundsReserveResult, AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                let reason = AccountFundsReserveRejectionReason::Frozen;
                self.append_event(AccountEventPayload::FundsReserveRejected { amount, reason })?;

                return Ok(AccountFundsReserveResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountFundsReserveRejectionReason::Closed;
                self.append_event(AccountEventPayload::FundsReserveRejected { amount, reason })?;

                return Ok(AccountFundsReserveResult::Rejected { reason });
            }
        }

        if self.available_balance()?.value() < amount.value() {
            let reason = AccountFundsReserveRejectionReason::InsufficientAvailableBalance;
            self.append_event(AccountEventPayload::FundsReserveRejected { amount, reason })?;

            return Ok(AccountFundsReserveResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::FundsReserved { amount })?;

        Ok(AccountFundsReserveResult::Reserved)
    }

    /// Releases reserved funds in the account.
    pub fn release_reserved_funds(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<AccountReservedFundsReleaseResult, AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                let reason = AccountReservedFundsReleaseRejectionReason::Frozen;
                self.append_event(AccountEventPayload::ReservedFundsReleaseRejected {
                    amount,
                    reason,
                })?;

                return Ok(AccountReservedFundsReleaseResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountReservedFundsReleaseRejectionReason::Closed;
                self.append_event(AccountEventPayload::ReservedFundsReleaseRejected {
                    amount,
                    reason,
                })?;

                return Ok(AccountReservedFundsReleaseResult::Rejected { reason });
            }
        }

        if self.state_required()?.reserved_balance.value() < amount.value() {
            let reason = AccountReservedFundsReleaseRejectionReason::InsufficientReservedBalance;
            self.append_event(AccountEventPayload::ReservedFundsReleaseRejected {
                amount,
                reason,
            })?;

            return Ok(AccountReservedFundsReleaseResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::ReservedFundsReleased { amount })?;

        Ok(AccountReservedFundsReleaseResult::Released)
    }

    /// Commits reserved funds and deducts them from the account.
    pub fn commit_reserved_funds(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<AccountReservedFundsCommitResult, AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                let reason = AccountReservedFundsCommitRejectionReason::Frozen;
                self.append_event(AccountEventPayload::ReservedFundsCommitRejected {
                    amount,
                    reason,
                })?;

                return Ok(AccountReservedFundsCommitResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountReservedFundsCommitRejectionReason::Closed;
                self.append_event(AccountEventPayload::ReservedFundsCommitRejected {
                    amount,
                    reason,
                })?;

                return Ok(AccountReservedFundsCommitResult::Rejected { reason });
            }
        }

        if self.state_required()?.reserved_balance.value() < amount.value() {
            let reason = AccountReservedFundsCommitRejectionReason::InsufficientReservedBalance;
            self.append_event(AccountEventPayload::ReservedFundsCommitRejected { amount, reason })?;

            return Ok(AccountReservedFundsCommitResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::ReservedFundsCommitted { amount })?;

        Ok(AccountReservedFundsCommitResult::Committed)
    }

    /// Freezes the account.
    pub fn freeze(&mut self) -> Result<AccountFreezeResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountFreezeRejectionReason::Closed;
            self.append_event(AccountEventPayload::FreezeRejected { reason })?;
            return Ok(AccountFreezeResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Frozen)?;
        Ok(AccountFreezeResult::Frozen)
    }

    /// Thaws the account.
    pub fn thaw(&mut self) -> Result<AccountThawResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountThawRejectionReason::Closed;
            self.append_event(AccountEventPayload::ThawRejected { reason })?;
            return Ok(AccountThawResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Thawed)?;
        Ok(AccountThawResult::Thawed)
    }

    /// Closes the account permanently.
    pub fn close(&mut self) -> Result<AccountCloseResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountCloseRejectionReason::AlreadyClosed;
            self.append_event(AccountEventPayload::CloseRejected { reason })?;
            return Ok(AccountCloseResult::Rejected { reason });
        }

        let state = self.state_required()?;
        if !state.reserved_balance.is_zero() {
            let reason = AccountCloseRejectionReason::ReservedBalanceRemaining;
            self.append_event(AccountEventPayload::CloseRejected { reason })?;
            return Ok(AccountCloseResult::Rejected { reason });
        }

        if !state.balance.is_zero() {
            let reason = AccountCloseRejectionReason::BalanceRemaining;
            self.append_event(AccountEventPayload::CloseRejected { reason })?;
            return Ok(AccountCloseResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Closed)?;
        Ok(AccountCloseResult::Closed)
    }
}

impl AggregateApply<AccountEventPayload, AccountError> for Account {
    fn apply(&mut self, payload: &AccountEventPayload) -> Result<(), AccountError> {
        match payload {
            AccountEventPayload::Opened {
                id,
                owner,
                name,
                currency_id,
            } => {
                let state = AccountState::new(*id, *owner, name.clone(), *currency_id);
                self.set_state(Some(state));
            }
            AccountEventPayload::OwnershipTransferred { owner } => {
                self.state_required_mut()?.owner = *owner;
            }
            AccountEventPayload::OwnershipTransferRejected { .. } => {}
            AccountEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone()
            }
            AccountEventPayload::NameChangeRejected { .. } => {}
            AccountEventPayload::Deposited { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.try_add(*amount)?;
            }
            AccountEventPayload::DepositRejected { .. } => {}
            AccountEventPayload::Withdrawn { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.try_sub(*amount)?;
            }
            AccountEventPayload::WithdrawRejected { .. } => {}
            AccountEventPayload::FundsReserved { amount } => {
                let state = self.state_required_mut()?;
                state.reserved_balance = state.reserved_balance.try_add(*amount)?;
            }
            AccountEventPayload::FundsReserveRejected { .. } => {}
            AccountEventPayload::ReservedFundsReleased { amount } => {
                let state = self.state_required_mut()?;
                state.reserved_balance =
                    state
                        .reserved_balance
                        .try_sub(*amount)
                        .map_err(|error| match error {
                            CurrencyAmountError::InsufficientBalance => {
                                AccountError::InsufficientReservedBalance
                            }
                            CurrencyAmountError::BalanceOverflow => AccountError::BalanceOverflow,
                        })?;
            }
            AccountEventPayload::ReservedFundsReleaseRejected { .. } => {}
            AccountEventPayload::ReservedFundsCommitted { amount } => {
                let state = self.state_required_mut()?;
                let next_reserved =
                    state
                        .reserved_balance
                        .try_sub(*amount)
                        .map_err(|error| match error {
                            CurrencyAmountError::InsufficientBalance => {
                                AccountError::InsufficientReservedBalance
                            }
                            CurrencyAmountError::BalanceOverflow => AccountError::BalanceOverflow,
                        })?;
                let next_balance = state.balance.try_sub(*amount)?;
                state.reserved_balance = next_reserved;
                state.balance = next_balance;
            }
            AccountEventPayload::ReservedFundsCommitRejected { .. } => {}
            AccountEventPayload::Frozen => {
                self.state_required_mut()?.status = AccountStatus::Frozen;
            }
            AccountEventPayload::FreezeRejected { .. } => {}
            AccountEventPayload::Thawed => {
                self.state_required_mut()?.status = AccountStatus::Active;
            }
            AccountEventPayload::ThawRejected { .. } => {}
            AccountEventPayload::Closed => {
                self.state_required_mut()?.status = AccountStatus::Closed;
            }
            AccountEventPayload::CloseRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::core::CurrencyAmount;
    use crate::currency::CurrencyId;

    use super::{
        Account, AccountEventPayload, AccountId, AccountName, AccountOwner, AccountStatus,
    };

    fn account_name() -> AccountName {
        AccountName::try_from("main").expect("account name should be valid")
    }

    fn account_owner() -> AccountOwner {
        AccountOwner::from(banking_iam_domain::UserId::new())
    }

    #[test]
    fn open_initializes_state_and_records_event() {
        let owner = account_owner();
        let name = account_name();
        let currency_id = CurrencyId::new();
        let mut account = Account::default();

        account
            .open(owner.clone(), name.clone(), currency_id)
            .expect("open should succeed");

        assert_eq!(
            account.aggregate_id().expect("aggregate id should exist"),
            account.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(account.owner().expect("owner should exist"), owner);
        assert_eq!(
            account.currency_id().expect("currency id should exist"),
            &currency_id
        );
        assert_eq!(
            account.balance().expect("balance should exist"),
            &CurrencyAmount::zero()
        );
        assert_eq!(
            account
                .reserved_balance()
                .expect("reserved balance should exist"),
            &CurrencyAmount::zero()
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
        assert_eq!(
            account.uncommitted_events()[0].payload(),
            &AccountEventPayload::Opened {
                id: account.aggregate_id().expect("aggregate id should exist"),
                owner,
                name,
                currency_id,
            }
        );
    }

    #[test]
    fn change_name_updates_name_and_records_event() {
        let owner = account_owner();
        let original_name = account_name();
        let name_changed = AccountName::try_from("savings").expect("account name should be valid");
        let mut account = Account::default();
        account
            .open(owner.clone(), original_name.clone(), CurrencyId::new())
            .expect("open should succeed");

        account
            .change_name(name_changed.clone())
            .expect("name change should succeed");

        assert_eq!(account.name().expect("name should exist"), &name_changed);
        assert_eq!(account.uncommitted_events().len(), 2);
        assert_eq!(
            account.uncommitted_events()[1].payload().name(),
            AccountEventPayload::NAME_CHANGED
        );
    }

    #[test]
    fn transfer_ownership_updates_owner_and_records_event() {
        let original_owner = account_owner();
        let transferred_owner =
            AccountOwner::Organization(banking_iam_domain::OrganizationId::new());
        let mut account = Account::default();
        account
            .open(original_owner, account_name(), CurrencyId::new())
            .expect("open should succeed");

        account
            .transfer_ownership(transferred_owner)
            .expect("ownership transfer should succeed");

        assert_eq!(
            account.owner().expect("owner should exist"),
            transferred_owner
        );
        assert_eq!(account.uncommitted_events().len(), 2);
        assert_eq!(
            account.uncommitted_events()[1].payload().name(),
            AccountEventPayload::OWNERSHIP_TRANSFERRED
        );
    }

    #[test]
    fn changing_to_same_name_appends_success_event() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");

        account
            .change_name(account_name())
            .expect("name change should succeed");

        assert_eq!(account.uncommitted_events().len(), 2);
    }

    #[test]
    fn transferring_to_same_owner_appends_success_event() {
        let owner = account_owner();
        let mut account = Account::default();
        account
            .open(owner, account_name(), CurrencyId::new())
            .expect("open should succeed");

        account
            .transfer_ownership(owner)
            .expect("same owner transfer should succeed");

        assert_eq!(account.uncommitted_events().len(), 2);
    }

    #[test]
    fn changing_to_same_status_appends_success_event() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");

        account.thaw().expect("thaw should succeed");

        assert_eq!(account.uncommitted_events().len(), 2);
    }

    #[test]
    fn freeze_and_thaw_update_state() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
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
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");

        account.close().expect("close should succeed");
        let duplicate_close_result = account
            .close()
            .expect("duplicate close should complete with a rejection event");

        assert!(account.is_closed().expect("closed state should exist"));
        assert_eq!(account.uncommitted_events().len(), 3);
        assert!(matches!(
            duplicate_close_result,
            super::AccountCloseResult::Rejected {
                reason: super::AccountCloseRejectionReason::AlreadyClosed
            }
        ));
        assert_eq!(
            account.uncommitted_events()[1].payload().name(),
            AccountEventPayload::CLOSED
        );
        assert_eq!(
            account.uncommitted_events()[2].payload().name(),
            AccountEventPayload::CLOSE_REJECTED
        );
    }

    #[test]
    fn close_rejects_non_zero_balance() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");
        account
            .deposit(CurrencyAmount::new(1))
            .expect("deposit should succeed");

        let result = account
            .close()
            .expect("close should complete with a rejection event");

        assert!(matches!(
            result,
            super::AccountCloseResult::Rejected {
                reason: super::AccountCloseRejectionReason::BalanceRemaining
            }
        ));
    }

    #[test]
    fn close_rejects_reserved_balance_remaining() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");
        account
            .deposit(CurrencyAmount::new(1))
            .expect("deposit should succeed");
        account
            .reserve_funds(CurrencyAmount::new(1))
            .expect("reserve should succeed");

        let result = account
            .close()
            .expect("close should complete with a rejection event");

        assert!(matches!(
            result,
            super::AccountCloseResult::Rejected {
                reason: super::AccountCloseRejectionReason::ReservedBalanceRemaining
            }
        ));
    }

    #[test]
    fn replay_events_after_closed_updates_state() {
        let id = AccountId::new();
        let owner = account_owner();
        let name = account_name();
        let currency_id = CurrencyId::new();
        let opened = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            AccountEventPayload::Opened {
                id,
                owner: owner.clone(),
                name: name.clone(),
                currency_id,
            },
        );
        let closed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            AccountEventPayload::Closed,
        );
        let name_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            AccountEventPayload::NameChanged {
                name: AccountName::try_from("archived").expect("account name should be valid"),
            },
        );
        let mut account = Account::default();

        account
            .replay_events(vec![opened, closed, name_changed], None)
            .expect("events should replay");

        assert_eq!(
            account.name().expect("name should exist"),
            &AccountName::try_from("archived").expect("account name should be valid")
        );
        assert_eq!(
            account.status().expect("status should exist"),
            AccountStatus::Closed
        );
        assert_eq!(account.version().value(), 3);
        assert!(account.uncommitted_events().is_empty());
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = AccountId::new();
        let owner = account_owner();
        let name = account_name();
        let currency_id = CurrencyId::new();
        let opened = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            AccountEventPayload::Opened {
                id,
                owner: owner.clone(),
                name: name.clone(),
                currency_id,
            },
        );
        let name_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            AccountEventPayload::NameChanged {
                name: AccountName::try_from("savings").expect("account name should be valid"),
            },
        );
        let deposited = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            AccountEventPayload::Deposited {
                amount: CurrencyAmount::new(100),
            },
        );
        let frozen = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(4).expect("version should be valid"),
            AccountEventPayload::Frozen,
        );
        let mut account = Account::default();

        account
            .replay_events(vec![opened, name_changed, deposited, frozen], None)
            .expect("events should replay");

        assert_eq!(account.owner().expect("owner should exist"), owner);
        assert_eq!(
            account.name().expect("name should exist"),
            &AccountName::try_from("savings").expect("account name should be valid")
        );
        assert_eq!(
            account.currency_id().expect("currency id should exist"),
            &currency_id
        );
        assert!(account.is_frozen().expect("frozen state should exist"));
        assert_eq!(
            account.balance().expect("balance should exist"),
            &CurrencyAmount::new(100)
        );
        assert_eq!(account.version().value(), 4);
        assert!(account.uncommitted_events().is_empty());
    }

    #[test]
    fn open_rejects_already_opened_account() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");

        let error = account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect_err("duplicate open should fail");

        assert!(matches!(error, super::AccountError::AlreadyOpened));
    }

    #[test]
    fn deposit_and_withdraw_update_balance() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");

        account
            .deposit(CurrencyAmount::new(150))
            .expect("deposit should succeed");
        account
            .withdraw(CurrencyAmount::new(40))
            .expect("withdraw should succeed");

        assert_eq!(
            account.balance().expect("balance should exist"),
            &CurrencyAmount::new(110)
        );
        assert_eq!(
            account
                .available_balance()
                .expect("available balance should be valid"),
            CurrencyAmount::new(110)
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
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");

        let result = account
            .withdraw(CurrencyAmount::new(1))
            .expect("withdraw should complete with a rejection event");

        assert!(matches!(
            result,
            super::AccountWithdrawResult::Rejected {
                reason: super::AccountWithdrawRejectionReason::InsufficientBalance
            }
        ));
        assert_eq!(
            account.uncommitted_events()[1].payload().name(),
            AccountEventPayload::WITHDRAW_REJECTED
        );
    }

    #[test]
    fn movement_rejects_frozen_account() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");
        account.freeze().expect("freeze should succeed");

        let deposit_result = account
            .deposit(CurrencyAmount::new(1))
            .expect("deposit should complete with a rejection event");
        let withdraw_result = account
            .withdraw(CurrencyAmount::new(1))
            .expect("withdraw should complete with a rejection event");

        assert!(matches!(
            deposit_result,
            super::AccountDepositResult::Rejected {
                reason: super::AccountDepositRejectionReason::Frozen
            }
        ));
        assert_eq!(
            account.uncommitted_events()[2].payload().name(),
            AccountEventPayload::DEPOSIT_REJECTED
        );
        assert!(matches!(
            withdraw_result,
            super::AccountWithdrawResult::Rejected {
                reason: super::AccountWithdrawRejectionReason::Frozen
            }
        ));
    }

    #[test]
    fn operations_reject_closed_account() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");
        account.close().expect("close should succeed");

        let freeze_result = account
            .freeze()
            .expect("freeze should complete with a rejection event");
        let thaw_result = account
            .thaw()
            .expect("thaw should complete with a rejection event");
        let name_change_result = account
            .change_name(AccountName::try_from("savings").expect("account name should be valid"))
            .expect("name change should complete with a rejection event");
        let deposit_result = account
            .deposit(CurrencyAmount::new(1))
            .expect("deposit should complete with a rejection event");
        let withdraw_result = account
            .withdraw(CurrencyAmount::new(1))
            .expect("withdraw should complete with a rejection event");
        let reserve_result = account
            .reserve_funds(CurrencyAmount::new(1))
            .expect("reserve should complete with a rejection event");
        let release_result = account
            .release_reserved_funds(CurrencyAmount::new(1))
            .expect("release should complete with a rejection event");
        let commit_result = account
            .commit_reserved_funds(CurrencyAmount::new(1))
            .expect("commit should complete with a rejection event");

        assert!(matches!(
            freeze_result,
            super::AccountFreezeResult::Rejected {
                reason: super::AccountFreezeRejectionReason::Closed
            }
        ));
        assert!(matches!(
            thaw_result,
            super::AccountThawResult::Rejected {
                reason: super::AccountThawRejectionReason::Closed
            }
        ));
        assert!(matches!(
            name_change_result,
            super::AccountNameChangeResult::Rejected {
                reason: super::AccountNameChangeRejectionReason::Closed
            }
        ));
        assert!(matches!(
            deposit_result,
            super::AccountDepositResult::Rejected {
                reason: super::AccountDepositRejectionReason::Closed
            }
        ));
        assert!(matches!(
            withdraw_result,
            super::AccountWithdrawResult::Rejected {
                reason: super::AccountWithdrawRejectionReason::Closed
            }
        ));
        assert!(matches!(
            reserve_result,
            super::AccountFundsReserveResult::Rejected {
                reason: super::AccountFundsReserveRejectionReason::Closed
            }
        ));
        assert!(matches!(
            release_result,
            super::AccountReservedFundsReleaseResult::Rejected {
                reason: super::AccountReservedFundsReleaseRejectionReason::Closed
            }
        ));
        assert!(matches!(
            commit_result,
            super::AccountReservedFundsCommitResult::Rejected {
                reason: super::AccountReservedFundsCommitRejectionReason::Closed
            }
        ));
    }

    #[test]
    fn reserve_release_and_commit_update_reserved_balance() {
        let mut account = Account::default();
        account
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");
        account
            .deposit(CurrencyAmount::new(150))
            .expect("deposit should succeed");

        account
            .reserve_funds(CurrencyAmount::new(40))
            .expect("reserve should succeed");
        account
            .release_reserved_funds(CurrencyAmount::new(10))
            .expect("release should succeed");
        account
            .commit_reserved_funds(CurrencyAmount::new(20))
            .expect("commit should succeed");

        assert_eq!(
            account.balance().expect("balance should exist"),
            &CurrencyAmount::new(130)
        );
        assert_eq!(
            account
                .reserved_balance()
                .expect("reserved balance should exist"),
            &CurrencyAmount::new(10)
        );
        assert_eq!(
            account
                .available_balance()
                .expect("available balance should be valid"),
            CurrencyAmount::new(120)
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
            .open(account_owner(), account_name(), CurrencyId::new())
            .expect("open should succeed");
        account
            .deposit(CurrencyAmount::new(100))
            .expect("deposit should succeed");
        account
            .reserve_funds(CurrencyAmount::new(80))
            .expect("reserve should succeed");

        let result = account
            .reserve_funds(CurrencyAmount::new(30))
            .expect("reserve should complete with a rejection event");

        assert!(matches!(
            result,
            super::AccountFundsReserveResult::Rejected {
                reason: super::AccountFundsReserveRejectionReason::InsufficientAvailableBalance
            }
        ));
        assert_eq!(
            account.uncommitted_events()[3].payload().name(),
            AccountEventPayload::FUNDS_RESERVE_REJECTED
        );
    }
}
