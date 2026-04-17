mod currency_error;
mod currency_event_payload;
mod currency_event_payload_error;
mod currency_id;
mod currency_name;
mod currency_name_error;
mod currency_owner;
mod currency_state;
mod currency_state_error;
mod currency_status;

pub use currency_error::CurrencyError;
pub use currency_event_payload::CurrencyEventPayload;
pub use currency_event_payload_error::CurrencyEventPayloadError;
pub use currency_id::CurrencyId;
pub use currency_name::CurrencyName;
pub use currency_name_error::CurrencyNameError;
pub use currency_owner::CurrencyOwner;
pub use currency_state::CurrencyState;
pub use currency_state_error::CurrencyStateError;
pub use currency_status::CurrencyStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{CurrencyAmount, CurrencyAmountError};
use crate::core::{CurrencyDecimals, CurrencySymbol};

/// Represents the `Currency` aggregate root.
#[aggregate(type = "currency", error = CurrencyError)]
pub struct Currency {
    core: AggregateCore<CurrencyState, CurrencyEventPayload>,
}

impl Currency {
    /// Returns the current owner.
    pub fn owner(&self) -> Result<CurrencyOwner, CurrencyError> {
        Ok(self.state_required()?.owner)
    }

    /// Returns the current symbol.
    pub fn symbol(&self) -> Result<&CurrencySymbol, CurrencyError> {
        Ok(&self.state_required()?.symbol)
    }

    /// Returns the current name.
    pub fn name(&self) -> Result<&CurrencyName, CurrencyError> {
        Ok(&self.state_required()?.name)
    }

    /// Returns the current decimals.
    pub fn decimals(&self) -> Result<&CurrencyDecimals, CurrencyError> {
        Ok(&self.state_required()?.decimals)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<CurrencyStatus, CurrencyError> {
        Ok(self.state_required()?.status)
    }

    /// Returns the total supply.
    pub fn supply(&self) -> Result<&CurrencyAmount, CurrencyError> {
        Ok(&self.state_required()?.supply)
    }

    /// Returns whether the currency is active.
    pub fn is_active(&self) -> Result<bool, CurrencyError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Defines a new currency.
    pub fn define(
        &mut self,
        owner: CurrencyOwner,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
    ) -> Result<(), CurrencyError> {
        if self.state().is_some() {
            return Err(CurrencyError::AlreadyDefined);
        }

        self.append_event(CurrencyEventPayload::Defined {
            id: CurrencyId::new(),
            owner,
            symbol,
            name,
            decimals,
        })
    }

    /// Changes the current currency symbol.
    pub fn change_symbol(&mut self, symbol: CurrencySymbol) -> Result<(), CurrencyError> {
        self.ensure_not_removed()?;

        if self.state_required()?.symbol.eq(&symbol) {
            return Ok(());
        }

        self.append_event(CurrencyEventPayload::SymbolChanged { symbol })
    }

    /// Changes the current currency name.
    pub fn change_name(&mut self, name: CurrencyName) -> Result<(), CurrencyError> {
        self.ensure_not_removed()?;

        if self.state_required()?.name.eq(&name) {
            return Ok(());
        }

        self.append_event(CurrencyEventPayload::NameChanged { name })
    }

    /// Activates the currency.
    pub fn activate(&mut self) -> Result<(), CurrencyError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_active() {
            return Ok(());
        }

        self.append_event(CurrencyEventPayload::Activated)
    }

    /// Deactivates the currency.
    pub fn deactivate(&mut self) -> Result<(), CurrencyError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_inactive() {
            return Ok(());
        }

        self.append_event(CurrencyEventPayload::Deactivated)
    }

    /// Permanently removes the currency.
    pub fn remove(&mut self) -> Result<(), CurrencyError> {
        self.ensure_not_removed()?;

        self.append_event(CurrencyEventPayload::Removed)
    }

    /// Increases the total supply.
    pub fn increase_supply(&mut self, amount: CurrencyAmount) -> Result<(), CurrencyError> {
        self.ensure_active()?;

        if amount.is_zero() {
            return Ok(());
        }

        self.append_event(CurrencyEventPayload::SupplyIncreased { amount })
    }

    /// Decreases the total supply.
    pub fn decrease_supply(&mut self, amount: CurrencyAmount) -> Result<(), CurrencyError> {
        self.ensure_not_removed()?;

        if amount.is_zero() {
            return Ok(());
        }

        if self.state_required()?.supply.value() < amount.value() {
            return Err(CurrencyError::InsufficientSupply);
        }

        self.append_event(CurrencyEventPayload::SupplyDecreased { amount })
    }

    fn ensure_not_removed(&self) -> Result<(), CurrencyError> {
        if self.state_required()?.status.is_removed() {
            return Err(CurrencyError::Removed);
        }

        Ok(())
    }

    fn ensure_active(&self) -> Result<(), CurrencyError> {
        match self.state_required()?.status {
            CurrencyStatus::Active => Ok(()),
            CurrencyStatus::Inactive => Err(CurrencyError::Inactive),
            CurrencyStatus::Removed => Err(CurrencyError::Removed),
        }
    }
}

impl AggregateApply<CurrencyEventPayload, CurrencyError> for Currency {
    fn apply(&mut self, payload: &CurrencyEventPayload) -> Result<(), CurrencyError> {
        match payload {
            CurrencyEventPayload::Defined {
                id,
                owner,
                symbol,
                name,
                decimals,
            } => {
                let state =
                    CurrencyState::new(*id, *owner, symbol.clone(), name.clone(), *decimals);
                self.set_state(Some(state));
            }
            CurrencyEventPayload::SymbolChanged { symbol } => {
                self.state_required_mut()?.symbol = symbol.clone();
            }
            CurrencyEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone();
            }
            CurrencyEventPayload::SupplyIncreased { amount } => {
                let state = self.state_required_mut()?;
                state.supply = state.supply.try_add(*amount).map_err(|error| match error {
                    CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                    CurrencyAmountError::InsufficientBalance => CurrencyError::InsufficientSupply,
                })?;
            }
            CurrencyEventPayload::SupplyDecreased { amount } => {
                let state = self.state_required_mut()?;
                state.supply = state.supply.try_sub(*amount).map_err(|error| match error {
                    CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                    CurrencyAmountError::InsufficientBalance => CurrencyError::InsufficientSupply,
                })?;
            }
            CurrencyEventPayload::Activated => {
                self.state_required_mut()?.status = CurrencyStatus::Active;
            }
            CurrencyEventPayload::Deactivated => {
                self.state_required_mut()?.status = CurrencyStatus::Inactive;
            }
            CurrencyEventPayload::Removed => {
                self.state_required_mut()?.status = CurrencyStatus::Removed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::core::CurrencyAmount;
    use crate::core::{CurrencyDecimals, CurrencySymbol};
    use banking_iam_domain::{OrganizationId, UserId};

    use super::{
        Currency, CurrencyEventPayload, CurrencyId, CurrencyName, CurrencyOwner, CurrencyStatus,
    };

    fn user_owner() -> CurrencyOwner {
        CurrencyOwner::user(UserId::new())
    }

    fn organization_owner() -> CurrencyOwner {
        CurrencyOwner::organization(OrganizationId::new())
    }

    #[test]
    fn define_initializes_state_and_records_event() {
        let owner = user_owner();
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let decimals = CurrencyDecimals::new(6);
        let mut currency = Currency::default();

        currency
            .define(owner.clone(), symbol.clone(), name.clone(), decimals)
            .expect("definition should succeed");

        assert_eq!(
            currency.aggregate_id().expect("aggregate id should exist"),
            currency.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(currency.symbol().expect("symbol should exist"), &symbol);
        assert_eq!(currency.name().expect("name should exist"), &name);
        assert_eq!(currency.owner().expect("owner should exist"), owner);
        assert_eq!(
            currency.decimals().expect("decimals should exist"),
            &decimals
        );
        assert!(currency.is_active().expect("active state should exist"));
        assert_eq!(
            currency.supply().expect("supply should exist"),
            &CurrencyAmount::zero()
        );
        assert_eq!(
            currency.status().expect("status should exist"),
            CurrencyStatus::Active
        );
        assert_eq!(currency.uncommitted_events().len(), 1);
        assert_eq!(
            currency.uncommitted_events()[0].payload().name(),
            CurrencyEventPayload::DEFINED
        );
        assert_eq!(
            currency.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::Defined {
                id: currency.aggregate_id().expect("aggregate id should exist"),
                owner,
                symbol,
                name,
                decimals,
            }
        );
    }

    #[test]
    fn changing_to_same_values_and_same_status_is_a_no_op() {
        let owner = user_owner();
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let decimals = CurrencyDecimals::new(6);
        let mut currency = Currency::default();
        currency
            .define(owner, symbol.clone(), name.clone(), decimals)
            .expect("definition should succeed");

        currency
            .change_symbol(symbol)
            .expect("no-op symbol change should succeed");
        currency
            .change_name(name)
            .expect("no-op name change should succeed");
        currency
            .activate()
            .expect("no-op activation should succeed");

        assert_eq!(currency.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_methods_append_events_and_update_state() {
        let owner = user_owner();
        let initial_symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let initial_name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let changed_symbol = CurrencySymbol::try_from("usdce").expect("symbol should be valid");
        let changed_name =
            CurrencyName::try_from("USD Coin Example").expect("name should be valid");
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                initial_symbol,
                initial_name,
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        currency
            .change_symbol(changed_symbol.clone())
            .expect("symbol change should succeed");
        currency
            .change_name(changed_name.clone())
            .expect("name change should succeed");
        currency.deactivate().expect("deactivation should succeed");

        assert_eq!(
            currency.symbol().expect("symbol should exist"),
            &changed_symbol
        );
        assert_eq!(currency.name().expect("name should exist"), &changed_name);
        assert_eq!(
            currency.decimals().expect("decimals should exist"),
            &CurrencyDecimals::new(6)
        );
        assert!(!currency.is_active().expect("active state should exist"));
        assert_eq!(currency.uncommitted_events().len(), 4);
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let owner = user_owner();
        let id = CurrencyId::new();
        let defined = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            CurrencyEventPayload::Defined {
                id,
                owner: owner.clone(),
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
            },
        );
        let deactivated = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            CurrencyEventPayload::Deactivated,
        );
        let mut currency = Currency::default();

        currency
            .replay_events(vec![defined, deactivated], None)
            .expect("events should replay");

        assert_eq!(
            currency.symbol().expect("symbol should exist").value(),
            "USDC"
        );
        assert_eq!(currency.owner().expect("owner should exist"), owner);
        assert!(!currency.is_active().expect("active state should exist"));
        assert_eq!(currency.version().value(), 2);
        assert!(currency.uncommitted_events().is_empty());
    }

    #[test]
    fn define_supports_organization_owner() {
        let owner = organization_owner();
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let mut currency = Currency::default();

        currency
            .define(owner.clone(), symbol, name, CurrencyDecimals::new(6))
            .expect("definition should succeed");

        assert_eq!(currency.owner().expect("owner should exist"), owner);
    }

    #[test]
    fn define_rejects_already_defined_currency() {
        let owner = user_owner();
        let mut currency = Currency::default();
        currency
            .define(
                owner.clone(),
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        let error = currency
            .define(
                owner,
                CurrencySymbol::try_from("sol").expect("symbol should be valid"),
                CurrencyName::try_from("Solana").expect("name should be valid"),
                CurrencyDecimals::new(9),
            )
            .expect_err("duplicate definition should fail");

        assert!(matches!(error, super::CurrencyError::AlreadyDefined));
    }

    #[test]
    fn supply_methods_update_supply() {
        let owner = user_owner();
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        currency
            .increase_supply(CurrencyAmount::new(100))
            .expect("increase should succeed");
        currency
            .decrease_supply(CurrencyAmount::new(40))
            .expect("decrease should succeed");

        assert_eq!(
            currency.supply().expect("supply should exist"),
            &CurrencyAmount::new(60)
        );
        assert_eq!(currency.uncommitted_events().len(), 3);
    }

    #[test]
    fn increase_supply_rejects_inactive_currency() {
        let owner = user_owner();
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");
        currency.deactivate().expect("deactivate should succeed");

        let error = currency
            .increase_supply(CurrencyAmount::new(1))
            .expect_err("increase should fail");

        assert!(matches!(error, super::CurrencyError::Inactive));
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let owner = user_owner();
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        currency.remove().expect("remove should succeed");
        let duplicate_remove_error = currency.remove().expect_err("duplicate remove should fail");

        assert_eq!(
            currency.status().expect("status should exist"),
            CurrencyStatus::Removed
        );
        assert_eq!(currency.uncommitted_events().len(), 2);
        assert!(matches!(
            duplicate_remove_error,
            super::CurrencyError::Removed
        ));
        assert_eq!(
            currency.uncommitted_events()[1].payload().name(),
            CurrencyEventPayload::REMOVED
        );
    }

    #[test]
    fn operations_reject_removed_currency() {
        let owner = user_owner();
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");
        currency.remove().expect("remove should succeed");

        let activate_error = currency.activate().expect_err("activate should fail");
        let deactivate_error = currency.deactivate().expect_err("deactivate should fail");
        let symbol_error = currency
            .change_symbol(CurrencySymbol::try_from("usdce").expect("symbol should be valid"))
            .expect_err("symbol change should fail");
        let name_error = currency
            .change_name(CurrencyName::try_from("USD Coin Example").expect("name should be valid"))
            .expect_err("name change should fail");
        let remove_error = currency.remove().expect_err("remove should fail");

        assert!(matches!(activate_error, super::CurrencyError::Removed));
        assert!(matches!(deactivate_error, super::CurrencyError::Removed));
        assert!(matches!(symbol_error, super::CurrencyError::Removed));
        assert!(matches!(name_error, super::CurrencyError::Removed));
        assert!(matches!(remove_error, super::CurrencyError::Removed));
    }
}
