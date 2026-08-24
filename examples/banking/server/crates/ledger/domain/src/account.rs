mod account_balance;
mod account_balance_error;
mod account_close_rejection_reason;
mod account_close_result;
mod account_deposit_rejection_reason;
mod account_deposit_result;
mod account_description;
mod account_description_change_rejection_reason;
mod account_description_change_result;
mod account_description_error;
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
mod account_open_result;
mod account_opening;
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

pub use account_balance::AccountBalance;
pub use account_balance_error::AccountBalanceError;
pub use account_close_rejection_reason::AccountCloseRejectionReason;
pub use account_close_result::AccountCloseResult;
pub use account_deposit_rejection_reason::AccountDepositRejectionReason;
pub use account_deposit_result::AccountDepositResult;
pub use account_description::AccountDescription;
pub use account_description_change_rejection_reason::AccountDescriptionChangeRejectionReason;
pub use account_description_change_result::AccountDescriptionChangeResult;
pub use account_description_error::AccountDescriptionError;
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
pub use account_open_result::AccountOpenResult;
pub use account_opening::AccountOpening;
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

use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

/// Represents the `Account` aggregate root.
#[aggregate(type = "account", error = AccountError)]
pub struct Account {
    core: AggregateCore<AccountId, AccountState, AccountEventPayload>,
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

    pub fn description(&self) -> Result<Option<&AccountDescription>, AccountError> {
        Ok(self.state_required()?.description.as_ref())
    }

    /// Returns the immutable account currency identity.
    pub fn currency_id(&self) -> Result<&CurrencyId, AccountError> {
        Ok(&self.state_required()?.currency_id)
    }

    /// Returns the current balance.
    pub fn balance(&self) -> Result<AccountBalance, AccountError> {
        Ok(self.state_required()?.balance)
    }

    /// Returns the current reserved balance.
    pub fn reserved_balance(&self) -> Result<CurrencyAmount, AccountError> {
        Ok(self.state_required()?.balance.reserved())
    }

    /// Returns the current available balance.
    pub fn available_balance(&self) -> Result<CurrencyAmount, AccountError> {
        Ok(self.state_required()?.balance.available()?)
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
    pub fn open(&mut self, opening: AccountOpening) -> Result<AccountOpenResult, AccountError> {
        if self.state().is_some() {
            return Err(AccountError::AlreadyOpened);
        }

        let (owner, name, description, currency_id) = opening.into_parts();
        self.append_event(AccountEventPayload::Opened {
            owner,
            name,
            description,
            currency_id,
        })?;

        Ok(AccountOpenResult::Opened)
    }

    /// Transfers ownership of the account.
    pub fn transfer_ownership(
        &mut self,
        owner: AccountOwner,
    ) -> Result<AccountOwnershipTransferResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountOwnershipTransferRejectionReason::Closed;
            self.reject_transfer_ownership(owner, reason)?;
            return Ok(AccountOwnershipTransferResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::OwnershipTransferred { owner })?;
        Ok(AccountOwnershipTransferResult::Transferred)
    }

    /// Rejects an account ownership transfer attempt.
    pub fn reject_transfer_ownership(
        &mut self,
        owner: AccountOwner,
        reason: AccountOwnershipTransferRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::OwnershipTransferRejected { owner, reason })?;
        Ok(())
    }

    /// Changes the account name.
    pub fn change_name(
        &mut self,
        name: AccountName,
    ) -> Result<AccountNameChangeResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountNameChangeRejectionReason::Closed;
            self.reject_change_name(name, reason)?;
            return Ok(AccountNameChangeResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::NameChanged { name })?;
        Ok(AccountNameChangeResult::Changed)
    }

    /// Rejects an account name change attempt.
    pub fn reject_change_name(
        &mut self,
        name: AccountName,
        reason: AccountNameChangeRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::NameChangeRejected { name, reason })?;
        Ok(())
    }

    pub fn change_description(
        &mut self,
        description: Option<AccountDescription>,
    ) -> Result<AccountDescriptionChangeResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountDescriptionChangeRejectionReason::Closed;
            self.reject_change_description(description, reason)?;
            return Ok(AccountDescriptionChangeResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::DescriptionChanged { description })?;
        Ok(AccountDescriptionChangeResult::Changed)
    }

    pub fn reject_change_description(
        &mut self,
        description: Option<AccountDescription>,
        reason: AccountDescriptionChangeRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::DescriptionChangeRejected {
            description,
            reason,
        })?;
        Ok(())
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
                self.reject_deposit(amount, reason)?;
                return Ok(AccountDepositResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountDepositRejectionReason::Closed;
                self.reject_deposit(amount, reason)?;
                return Ok(AccountDepositResult::Rejected { reason });
            }
        }

        self.append_event(AccountEventPayload::Deposited { amount })?;

        Ok(AccountDepositResult::Deposited)
    }

    /// Rejects an account deposit attempt.
    pub fn reject_deposit(
        &mut self,
        amount: CurrencyAmount,
        reason: AccountDepositRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::DepositRejected { amount, reason })?;
        Ok(())
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
                self.reject_withdraw(amount, reason)?;
                return Ok(AccountWithdrawResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountWithdrawRejectionReason::Closed;
                self.reject_withdraw(amount, reason)?;
                return Ok(AccountWithdrawResult::Rejected { reason });
            }
        }

        if self.available_balance()? < amount {
            let reason = AccountWithdrawRejectionReason::InsufficientBalance;
            self.reject_withdraw(amount, reason)?;
            return Ok(AccountWithdrawResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Withdrawn { amount })?;
        Ok(AccountWithdrawResult::Withdrawn)
    }

    /// Rejects an account withdrawal attempt.
    pub fn reject_withdraw(
        &mut self,
        amount: CurrencyAmount,
        reason: AccountWithdrawRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::WithdrawRejected { amount, reason })?;
        Ok(())
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
                self.reject_reserve_funds(amount, reason)?;
                return Ok(AccountFundsReserveResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountFundsReserveRejectionReason::Closed;
                self.reject_reserve_funds(amount, reason)?;
                return Ok(AccountFundsReserveResult::Rejected { reason });
            }
        }

        if self.available_balance()? < amount {
            let reason = AccountFundsReserveRejectionReason::InsufficientAvailableBalance;
            self.reject_reserve_funds(amount, reason)?;
            return Ok(AccountFundsReserveResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::FundsReserved { amount })?;

        Ok(AccountFundsReserveResult::Reserved)
    }

    /// Rejects an account funds reservation attempt.
    pub fn reject_reserve_funds(
        &mut self,
        amount: CurrencyAmount,
        reason: AccountFundsReserveRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::FundsReserveRejected { amount, reason })?;
        Ok(())
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
                self.reject_release_reserved_funds(amount, reason)?;
                return Ok(AccountReservedFundsReleaseResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountReservedFundsReleaseRejectionReason::Closed;
                self.reject_release_reserved_funds(amount, reason)?;
                return Ok(AccountReservedFundsReleaseResult::Rejected { reason });
            }
        }

        if self.state_required()?.balance.reserved() < amount {
            let reason = AccountReservedFundsReleaseRejectionReason::InsufficientReservedBalance;
            self.reject_release_reserved_funds(amount, reason)?;
            return Ok(AccountReservedFundsReleaseResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::ReservedFundsReleased { amount })?;

        Ok(AccountReservedFundsReleaseResult::Released)
    }

    /// Rejects a reserved funds release attempt.
    pub fn reject_release_reserved_funds(
        &mut self,
        amount: CurrencyAmount,
        reason: AccountReservedFundsReleaseRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::ReservedFundsReleaseRejected { amount, reason })?;
        Ok(())
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
                self.reject_commit_reserved_funds(amount, reason)?;
                return Ok(AccountReservedFundsCommitResult::Rejected { reason });
            }
            AccountStatus::Closed => {
                let reason = AccountReservedFundsCommitRejectionReason::Closed;
                self.reject_commit_reserved_funds(amount, reason)?;
                return Ok(AccountReservedFundsCommitResult::Rejected { reason });
            }
        }

        if self.state_required()?.balance.reserved() < amount {
            let reason = AccountReservedFundsCommitRejectionReason::InsufficientReservedBalance;
            self.reject_commit_reserved_funds(amount, reason)?;
            return Ok(AccountReservedFundsCommitResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::ReservedFundsCommitted { amount })?;

        Ok(AccountReservedFundsCommitResult::Committed)
    }

    /// Rejects a reserved funds commit attempt.
    pub fn reject_commit_reserved_funds(
        &mut self,
        amount: CurrencyAmount,
        reason: AccountReservedFundsCommitRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::ReservedFundsCommitRejected { amount, reason })?;
        Ok(())
    }

    /// Freezes the account.
    pub fn freeze(&mut self) -> Result<AccountFreezeResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountFreezeRejectionReason::Closed;
            self.reject_freeze(reason)?;
            return Ok(AccountFreezeResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Frozen)?;
        Ok(AccountFreezeResult::Frozen)
    }

    /// Rejects an account freeze attempt.
    pub fn reject_freeze(
        &mut self,
        reason: AccountFreezeRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::FreezeRejected { reason })?;
        Ok(())
    }

    /// Thaws the account.
    pub fn thaw(&mut self) -> Result<AccountThawResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountThawRejectionReason::Closed;
            self.reject_thaw(reason)?;
            return Ok(AccountThawResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Thawed)?;
        Ok(AccountThawResult::Thawed)
    }

    /// Rejects an account thaw attempt.
    pub fn reject_thaw(&mut self, reason: AccountThawRejectionReason) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::ThawRejected { reason })?;
        Ok(())
    }

    /// Closes the account permanently.
    pub fn close(&mut self) -> Result<AccountCloseResult, AccountError> {
        if self.state_required()?.status.is_closed() {
            let reason = AccountCloseRejectionReason::AlreadyClosed;
            self.reject_close(reason)?;
            return Ok(AccountCloseResult::Rejected { reason });
        }

        let state = self.state_required()?;
        if !state.balance.reserved().is_zero() {
            let reason = AccountCloseRejectionReason::ReservedBalanceRemaining;
            self.reject_close(reason)?;
            return Ok(AccountCloseResult::Rejected { reason });
        }

        if !state.balance.total().is_zero() {
            let reason = AccountCloseRejectionReason::BalanceRemaining;
            self.reject_close(reason)?;
            return Ok(AccountCloseResult::Rejected { reason });
        }

        self.append_event(AccountEventPayload::Closed)?;
        Ok(AccountCloseResult::Closed)
    }

    /// Rejects an account close attempt.
    pub fn reject_close(
        &mut self,
        reason: AccountCloseRejectionReason,
    ) -> Result<(), AccountError> {
        self.append_event(AccountEventPayload::CloseRejected { reason })?;
        Ok(())
    }
}

impl AggregateApply<AccountEventPayload, AccountError> for Account {
    fn apply(&mut self, payload: &AccountEventPayload) -> Result<(), AccountError> {
        match payload {
            AccountEventPayload::Opened {
                owner,
                name,
                description,
                currency_id,
            } => {
                self.set_state(Some(AccountState {
                    owner: *owner,
                    name: name.clone(),
                    description: description.clone(),
                    currency_id: *currency_id,
                    balance: AccountBalance::new(),
                    status: AccountStatus::Active,
                }));
            }
            AccountEventPayload::OwnershipTransferred { owner } => {
                self.state_required_mut()?.owner = *owner;
            }
            AccountEventPayload::OwnershipTransferRejected { .. } => {}
            AccountEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone()
            }
            AccountEventPayload::NameChangeRejected { .. } => {}
            AccountEventPayload::DescriptionChanged { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            AccountEventPayload::DescriptionChangeRejected { .. } => {}
            AccountEventPayload::Deposited { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.deposit(*amount)?;
            }
            AccountEventPayload::DepositRejected { .. } => {}
            AccountEventPayload::Withdrawn { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.withdraw(*amount)?;
            }
            AccountEventPayload::WithdrawRejected { .. } => {}
            AccountEventPayload::FundsReserved { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.reserve(*amount)?;
            }
            AccountEventPayload::FundsReserveRejected { .. } => {}
            AccountEventPayload::ReservedFundsReleased { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.release(*amount)?;
            }
            AccountEventPayload::ReservedFundsReleaseRejected { .. } => {}
            AccountEventPayload::ReservedFundsCommitted { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.commit(*amount)?;
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
    use appletheia::domain::Aggregate;
    use banking_iam_domain::UserId;

    use crate::core::CurrencyAmount;
    use crate::currency::CurrencyId;

    use super::{Account, AccountEventPayload, AccountName, AccountOpening, AccountOwner};

    #[test]
    fn monetary_events_store_only_the_smallest_unit_amount() {
        let mut account = Account::new();
        account
            .open(AccountOpening {
                owner: AccountOwner::from(UserId::new()),
                name: AccountName::try_from("main").expect("valid name"),
                description: None,
                currency_id: CurrencyId::new(),
            })
            .expect("open should succeed");
        account
            .deposit(CurrencyAmount::new(125))
            .expect("deposit should succeed");

        assert!(matches!(
            account.uncommitted_events()[1].payload(),
            AccountEventPayload::Deposited { amount }
                if *amount == CurrencyAmount::new(125)
        ));
    }
}
