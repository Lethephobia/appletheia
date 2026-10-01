mod account_balance;
mod account_balance_error;
mod account_description;
mod account_description_error;
mod account_error;
mod account_event_payload;
mod account_event_payload_error;
mod account_id;
mod account_name;
mod account_name_error;
mod account_opening;
mod account_owner;
mod account_state;
mod account_state_error;
mod account_status;

pub use account_balance::AccountBalance;
pub use account_balance_error::AccountBalanceError;
pub use account_description::AccountDescription;
pub use account_description_error::AccountDescriptionError;
pub use account_error::AccountError;
pub use account_event_payload::AccountEventPayload;
pub use account_event_payload_error::AccountEventPayloadError;
pub use account_id::AccountId;
pub use account_name::AccountName;
pub use account_name_error::AccountNameError;
pub use account_opening::AccountOpening;
pub use account_owner::AccountOwner;
pub use account_state::AccountState;
pub use account_state_error::AccountStateError;
pub use account_status::AccountStatus;

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
    pub fn open(&mut self, opening: AccountOpening) -> Result<(), AccountError> {
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

        Ok(())
    }

    /// Transfers ownership of the account.
    pub fn transfer_ownership(&mut self, owner: AccountOwner) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Err(AccountError::Closed);
        }

        self.append_event(AccountEventPayload::OwnershipTransferred { owner })?;
        Ok(())
    }

    /// Changes the account name.
    pub fn change_name(&mut self, name: AccountName) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Err(AccountError::Closed);
        }

        self.append_event(AccountEventPayload::NameChanged { name })?;
        Ok(())
    }

    pub fn change_description(
        &mut self,
        description: Option<AccountDescription>,
    ) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Err(AccountError::Closed);
        }

        self.append_event(AccountEventPayload::DescriptionChanged { description })?;
        Ok(())
    }

    /// Deposits balance into the account.
    pub fn deposit(&mut self, amount: CurrencyAmount) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                return Err(AccountError::Frozen);
            }
            AccountStatus::Closed => {
                return Err(AccountError::Closed);
            }
        }

        self.append_event(AccountEventPayload::Deposited { amount })?;

        Ok(())
    }

    /// Withdraws balance from the account.
    pub fn withdraw(&mut self, amount: CurrencyAmount) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                return Err(AccountError::Frozen);
            }
            AccountStatus::Closed => {
                return Err(AccountError::Closed);
            }
        }

        if self.available_balance()? < amount {
            return Err(AccountError::InsufficientBalance);
        }

        self.append_event(AccountEventPayload::Withdrawn { amount })?;
        Ok(())
    }

    /// Reserves funds in the account.
    pub fn reserve_funds(&mut self, amount: CurrencyAmount) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                return Err(AccountError::Frozen);
            }
            AccountStatus::Closed => {
                return Err(AccountError::Closed);
            }
        }

        if self.available_balance()? < amount {
            return Err(AccountError::InsufficientAvailableBalance);
        }

        self.append_event(AccountEventPayload::FundsReserved { amount })?;

        Ok(())
    }

    /// Releases reserved funds in the account.
    pub fn release_reserved_funds(&mut self, amount: CurrencyAmount) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                return Err(AccountError::Frozen);
            }
            AccountStatus::Closed => {
                return Err(AccountError::Closed);
            }
        }

        if self.state_required()?.balance.reserved() < amount {
            return Err(AccountError::InsufficientReservedBalance);
        }

        self.append_event(AccountEventPayload::ReservedFundsReleased { amount })?;

        Ok(())
    }

    /// Commits reserved funds and deducts them from the account.
    pub fn commit_reserved_funds(&mut self, amount: CurrencyAmount) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                return Err(AccountError::Frozen);
            }
            AccountStatus::Closed => {
                return Err(AccountError::Closed);
            }
        }

        if self.state_required()?.balance.reserved() < amount {
            return Err(AccountError::InsufficientReservedBalance);
        }

        self.append_event(AccountEventPayload::ReservedFundsCommitted { amount })?;

        Ok(())
    }

    /// Freezes the account.
    pub fn freeze(&mut self) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Err(AccountError::Closed);
        }

        self.append_event(AccountEventPayload::Frozen)?;
        Ok(())
    }

    /// Thaws the account.
    pub fn thaw(&mut self) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Err(AccountError::Closed);
        }

        self.append_event(AccountEventPayload::Thawed)?;
        Ok(())
    }

    /// Closes the account permanently.
    pub fn close(&mut self) -> Result<(), AccountError> {
        if self.state_required()?.status.is_closed() {
            return Err(AccountError::Closed);
        }

        let state = self.state_required()?;
        if !state.balance.reserved().is_zero() {
            return Err(AccountError::ReservedBalanceRemaining);
        }

        if !state.balance.total().is_zero() {
            return Err(AccountError::BalanceRemaining);
        }

        self.append_event(AccountEventPayload::Closed)?;
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
            AccountEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone()
            }
            AccountEventPayload::DescriptionChanged { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            AccountEventPayload::Deposited { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.deposit(*amount)?;
            }
            AccountEventPayload::Withdrawn { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.withdraw(*amount)?;
            }
            AccountEventPayload::FundsReserved { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.reserve(*amount)?;
            }
            AccountEventPayload::ReservedFundsReleased { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.release(*amount)?;
            }
            AccountEventPayload::ReservedFundsCommitted { amount } => {
                let state = self.state_required_mut()?;
                state.balance = state.balance.commit(*amount)?;
            }
            AccountEventPayload::Frozen => {
                self.state_required_mut()?.status = AccountStatus::Frozen;
            }
            AccountEventPayload::Thawed => {
                self.state_required_mut()?.status = AccountStatus::Active;
            }
            AccountEventPayload::Closed => {
                self.state_required_mut()?.status = AccountStatus::Closed;
            }
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

    use super::{
        Account, AccountError, AccountEventPayload, AccountName, AccountOpening, AccountOwner,
    };

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

    #[test]
    fn failed_deposit_does_not_append_an_event() {
        let mut account = Account::new();
        account
            .open(AccountOpening {
                owner: AccountOwner::from(UserId::new()),
                name: AccountName::try_from("main").expect("valid name"),
                description: None,
                currency_id: CurrencyId::new(),
            })
            .expect("open should succeed");
        account.close().expect("close should succeed");
        let event_count = account.uncommitted_events().len();

        let error = account
            .deposit(CurrencyAmount::new(125))
            .expect_err("deposit to closed account should fail");

        assert!(matches!(error, AccountError::Closed));
        assert_eq!(account.uncommitted_events().len(), event_count);
    }
}
