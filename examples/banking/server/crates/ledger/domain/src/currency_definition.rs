mod currency_definition_error;
mod currency_definition_event_payload;
mod currency_definition_event_payload_error;
mod currency_definition_id;
mod currency_definition_owner;
mod currency_definition_state;
mod currency_definition_state_error;
mod currency_definition_status;
mod currency_name;
mod currency_name_error;

pub use currency_definition_error::CurrencyDefinitionError;
pub use currency_definition_event_payload::CurrencyDefinitionEventPayload;
pub use currency_definition_event_payload_error::CurrencyDefinitionEventPayloadError;
pub use currency_definition_id::CurrencyDefinitionId;
pub use currency_definition_owner::CurrencyDefinitionOwner;
pub use currency_definition_state::CurrencyDefinitionState;
pub use currency_definition_state_error::CurrencyDefinitionStateError;
pub use currency_definition_status::CurrencyDefinitionStatus;
pub use currency_name::CurrencyName;
pub use currency_name_error::CurrencyNameError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{CurrencyAmount, CurrencyAmountError};
use crate::core::{CurrencyDecimals, CurrencySymbol};

/// Represents the `CurrencyDefinition` aggregate root.
#[aggregate(type = "currency_definition", error = CurrencyDefinitionError)]
pub struct CurrencyDefinition {
    core: AggregateCore<CurrencyDefinitionState, CurrencyDefinitionEventPayload>,
}

impl CurrencyDefinition {
    /// Returns the current owner.
    pub fn owner(&self) -> Result<CurrencyDefinitionOwner, CurrencyDefinitionError> {
        Ok(self.state_required()?.owner)
    }

    /// Returns the current symbol.
    pub fn symbol(&self) -> Result<&CurrencySymbol, CurrencyDefinitionError> {
        Ok(&self.state_required()?.symbol)
    }

    /// Returns the current name.
    pub fn name(&self) -> Result<&CurrencyName, CurrencyDefinitionError> {
        Ok(&self.state_required()?.name)
    }

    /// Returns the current decimals.
    pub fn decimals(&self) -> Result<&CurrencyDecimals, CurrencyDefinitionError> {
        Ok(&self.state_required()?.decimals)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<CurrencyDefinitionStatus, CurrencyDefinitionError> {
        Ok(self.state_required()?.status)
    }

    /// Returns the total supply.
    pub fn supply(&self) -> Result<&CurrencyAmount, CurrencyDefinitionError> {
        Ok(&self.state_required()?.supply)
    }

    /// Returns whether the currency is active.
    pub fn is_active(&self) -> Result<bool, CurrencyDefinitionError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Defines a new currency.
    pub fn define(
        &mut self,
        owner: CurrencyDefinitionOwner,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
    ) -> Result<(), CurrencyDefinitionError> {
        if self.state().is_some() {
            return Err(CurrencyDefinitionError::AlreadyDefined);
        }

        self.append_event(CurrencyDefinitionEventPayload::Defined {
            id: CurrencyDefinitionId::new(),
            owner,
            symbol,
            name,
            decimals,
        })
    }

    /// Changes the current currency symbol.
    pub fn change_symbol(&mut self, symbol: CurrencySymbol) -> Result<(), CurrencyDefinitionError> {
        self.ensure_not_removed()?;

        if self.state_required()?.symbol.eq(&symbol) {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::SymbolChanged { symbol })
    }

    /// Changes the current currency name.
    pub fn change_name(&mut self, name: CurrencyName) -> Result<(), CurrencyDefinitionError> {
        self.ensure_not_removed()?;

        if self.state_required()?.name.eq(&name) {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::NameChanged { name })
    }

    /// Activates the currency.
    pub fn activate(&mut self) -> Result<(), CurrencyDefinitionError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_active() {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::Activated)
    }

    /// Deactivates the currency.
    pub fn deactivate(&mut self) -> Result<(), CurrencyDefinitionError> {
        self.ensure_not_removed()?;

        if self.state_required()?.status.is_inactive() {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::Deactivated)
    }

    /// Permanently removes the currency definition.
    pub fn remove(&mut self) -> Result<(), CurrencyDefinitionError> {
        self.ensure_not_removed()?;

        self.append_event(CurrencyDefinitionEventPayload::Removed)
    }

    /// Increases the total supply.
    pub fn increase_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<(), CurrencyDefinitionError> {
        self.ensure_active()?;

        if amount.is_zero() {
            return Ok(());
        }

        self.append_event(CurrencyDefinitionEventPayload::SupplyIncreased { amount })
    }

    /// Decreases the total supply.
    pub fn decrease_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<(), CurrencyDefinitionError> {
        self.ensure_not_removed()?;

        if amount.is_zero() {
            return Ok(());
        }

        if self.state_required()?.supply.value() < amount.value() {
            return Err(CurrencyDefinitionError::InsufficientSupply);
        }

        self.append_event(CurrencyDefinitionEventPayload::SupplyDecreased { amount })
    }

    fn ensure_not_removed(&self) -> Result<(), CurrencyDefinitionError> {
        if self.state_required()?.status.is_removed() {
            return Err(CurrencyDefinitionError::Removed);
        }

        Ok(())
    }

    fn ensure_active(&self) -> Result<(), CurrencyDefinitionError> {
        match self.state_required()?.status {
            CurrencyDefinitionStatus::Active => Ok(()),
            CurrencyDefinitionStatus::Inactive => Err(CurrencyDefinitionError::Inactive),
            CurrencyDefinitionStatus::Removed => Err(CurrencyDefinitionError::Removed),
        }
    }
}

impl AggregateApply<CurrencyDefinitionEventPayload, CurrencyDefinitionError>
    for CurrencyDefinition
{
    fn apply(
        &mut self,
        payload: &CurrencyDefinitionEventPayload,
    ) -> Result<(), CurrencyDefinitionError> {
        match payload {
            CurrencyDefinitionEventPayload::Defined {
                id,
                owner,
                symbol,
                name,
                decimals,
            } => {
                let state = CurrencyDefinitionState::new(
                    *id,
                    *owner,
                    symbol.clone(),
                    name.clone(),
                    *decimals,
                );
                self.set_state(Some(state));
            }
            CurrencyDefinitionEventPayload::SymbolChanged { symbol } => {
                self.state_required_mut()?.symbol = symbol.clone();
            }
            CurrencyDefinitionEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone();
            }
            CurrencyDefinitionEventPayload::SupplyIncreased { amount } => {
                let state = self.state_required_mut()?;
                state.supply = state.supply.try_add(*amount).map_err(|error| match error {
                    CurrencyAmountError::BalanceOverflow => CurrencyDefinitionError::SupplyOverflow,
                    CurrencyAmountError::InsufficientBalance => {
                        CurrencyDefinitionError::InsufficientSupply
                    }
                })?;
            }
            CurrencyDefinitionEventPayload::SupplyDecreased { amount } => {
                let state = self.state_required_mut()?;
                state.supply = state.supply.try_sub(*amount).map_err(|error| match error {
                    CurrencyAmountError::BalanceOverflow => CurrencyDefinitionError::SupplyOverflow,
                    CurrencyAmountError::InsufficientBalance => {
                        CurrencyDefinitionError::InsufficientSupply
                    }
                })?;
            }
            CurrencyDefinitionEventPayload::Activated => {
                self.state_required_mut()?.status = CurrencyDefinitionStatus::Active;
            }
            CurrencyDefinitionEventPayload::Deactivated => {
                self.state_required_mut()?.status = CurrencyDefinitionStatus::Inactive;
            }
            CurrencyDefinitionEventPayload::Removed => {
                self.state_required_mut()?.status = CurrencyDefinitionStatus::Removed;
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
        CurrencyDefinition, CurrencyDefinitionEventPayload, CurrencyDefinitionId,
        CurrencyDefinitionOwner, CurrencyDefinitionStatus, CurrencyName,
    };

    fn user_owner() -> CurrencyDefinitionOwner {
        CurrencyDefinitionOwner::user(UserId::new())
    }

    fn organization_owner() -> CurrencyDefinitionOwner {
        CurrencyDefinitionOwner::organization(OrganizationId::new())
    }

    #[test]
    fn define_initializes_state_and_records_event() {
        let owner = user_owner();
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let decimals = CurrencyDecimals::new(6);
        let mut currency_definition = CurrencyDefinition::default();

        currency_definition
            .define(owner.clone(), symbol.clone(), name.clone(), decimals)
            .expect("definition should succeed");

        assert_eq!(
            currency_definition
                .aggregate_id()
                .expect("aggregate id should exist"),
            currency_definition
                .aggregate_id()
                .expect("aggregate id should exist")
        );
        assert_eq!(
            currency_definition.symbol().expect("symbol should exist"),
            &symbol
        );
        assert_eq!(
            currency_definition.name().expect("name should exist"),
            &name
        );
        assert_eq!(
            currency_definition.owner().expect("owner should exist"),
            owner
        );
        assert_eq!(
            currency_definition
                .decimals()
                .expect("decimals should exist"),
            &decimals
        );
        assert!(
            currency_definition
                .is_active()
                .expect("active state should exist")
        );
        assert_eq!(
            currency_definition.supply().expect("supply should exist"),
            &CurrencyAmount::zero()
        );
        assert_eq!(
            currency_definition.status().expect("status should exist"),
            CurrencyDefinitionStatus::Active
        );
        assert_eq!(currency_definition.uncommitted_events().len(), 1);
        assert_eq!(
            currency_definition.uncommitted_events()[0].payload().name(),
            CurrencyDefinitionEventPayload::DEFINED
        );
        assert_eq!(
            currency_definition.uncommitted_events()[0].payload(),
            &CurrencyDefinitionEventPayload::Defined {
                id: currency_definition
                    .aggregate_id()
                    .expect("aggregate id should exist"),
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
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(owner, symbol.clone(), name.clone(), decimals)
            .expect("definition should succeed");

        currency_definition
            .change_symbol(symbol)
            .expect("no-op symbol change should succeed");
        currency_definition
            .change_name(name)
            .expect("no-op name change should succeed");
        currency_definition
            .activate()
            .expect("no-op activation should succeed");

        assert_eq!(currency_definition.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_methods_append_events_and_update_state() {
        let owner = user_owner();
        let initial_symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let initial_name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let changed_symbol = CurrencySymbol::try_from("usdce").expect("symbol should be valid");
        let changed_name =
            CurrencyName::try_from("USD Coin Example").expect("name should be valid");
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                owner,
                initial_symbol,
                initial_name,
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        currency_definition
            .change_symbol(changed_symbol.clone())
            .expect("symbol change should succeed");
        currency_definition
            .change_name(changed_name.clone())
            .expect("name change should succeed");
        currency_definition
            .deactivate()
            .expect("deactivation should succeed");

        assert_eq!(
            currency_definition.symbol().expect("symbol should exist"),
            &changed_symbol
        );
        assert_eq!(
            currency_definition.name().expect("name should exist"),
            &changed_name
        );
        assert_eq!(
            currency_definition
                .decimals()
                .expect("decimals should exist"),
            &CurrencyDecimals::new(6)
        );
        assert!(
            !currency_definition
                .is_active()
                .expect("active state should exist")
        );
        assert_eq!(currency_definition.uncommitted_events().len(), 4);
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let owner = user_owner();
        let id = CurrencyDefinitionId::new();
        let defined = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            CurrencyDefinitionEventPayload::Defined {
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
            CurrencyDefinitionEventPayload::Deactivated,
        );
        let mut currency_definition = CurrencyDefinition::default();

        currency_definition
            .replay_events(vec![defined, deactivated], None)
            .expect("events should replay");

        assert_eq!(
            currency_definition
                .symbol()
                .expect("symbol should exist")
                .value(),
            "USDC"
        );
        assert_eq!(
            currency_definition.owner().expect("owner should exist"),
            owner
        );
        assert!(
            !currency_definition
                .is_active()
                .expect("active state should exist")
        );
        assert_eq!(currency_definition.version().value(), 2);
        assert!(currency_definition.uncommitted_events().is_empty());
    }

    #[test]
    fn define_supports_organization_owner() {
        let owner = organization_owner();
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let mut currency_definition = CurrencyDefinition::default();

        currency_definition
            .define(owner.clone(), symbol, name, CurrencyDecimals::new(6))
            .expect("definition should succeed");

        assert_eq!(
            currency_definition.owner().expect("owner should exist"),
            owner
        );
    }

    #[test]
    fn define_rejects_already_defined_currency() {
        let owner = user_owner();
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                owner.clone(),
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        let error = currency_definition
            .define(
                owner,
                CurrencySymbol::try_from("sol").expect("symbol should be valid"),
                CurrencyName::try_from("Solana").expect("name should be valid"),
                CurrencyDecimals::new(9),
            )
            .expect_err("duplicate definition should fail");

        assert!(matches!(
            error,
            super::CurrencyDefinitionError::AlreadyDefined
        ));
    }

    #[test]
    fn supply_methods_update_supply() {
        let owner = user_owner();
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        currency_definition
            .increase_supply(CurrencyAmount::new(100))
            .expect("increase should succeed");
        currency_definition
            .decrease_supply(CurrencyAmount::new(40))
            .expect("decrease should succeed");

        assert_eq!(
            currency_definition.supply().expect("supply should exist"),
            &CurrencyAmount::new(60)
        );
        assert_eq!(currency_definition.uncommitted_events().len(), 3);
    }

    #[test]
    fn increase_supply_rejects_inactive_currency() {
        let owner = user_owner();
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");
        currency_definition
            .deactivate()
            .expect("deactivate should succeed");

        let error = currency_definition
            .increase_supply(CurrencyAmount::new(1))
            .expect_err("increase should fail");

        assert!(matches!(error, super::CurrencyDefinitionError::Inactive));
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let owner = user_owner();
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");

        currency_definition.remove().expect("remove should succeed");
        let duplicate_remove_error = currency_definition
            .remove()
            .expect_err("duplicate remove should fail");

        assert_eq!(
            currency_definition.status().expect("status should exist"),
            CurrencyDefinitionStatus::Removed
        );
        assert_eq!(currency_definition.uncommitted_events().len(), 2);
        assert!(matches!(
            duplicate_remove_error,
            super::CurrencyDefinitionError::Removed
        ));
        assert_eq!(
            currency_definition.uncommitted_events()[1].payload().name(),
            CurrencyDefinitionEventPayload::REMOVED
        );
    }

    #[test]
    fn operations_reject_removed_currency_definition() {
        let owner = user_owner();
        let mut currency_definition = CurrencyDefinition::default();
        currency_definition
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
            )
            .expect("definition should succeed");
        currency_definition.remove().expect("remove should succeed");

        let activate_error = currency_definition
            .activate()
            .expect_err("activate should fail");
        let deactivate_error = currency_definition
            .deactivate()
            .expect_err("deactivate should fail");
        let symbol_error = currency_definition
            .change_symbol(CurrencySymbol::try_from("usdce").expect("symbol should be valid"))
            .expect_err("symbol change should fail");
        let name_error = currency_definition
            .change_name(CurrencyName::try_from("USD Coin Example").expect("name should be valid"))
            .expect_err("name change should fail");
        let remove_error = currency_definition
            .remove()
            .expect_err("remove should fail");

        assert!(matches!(
            activate_error,
            super::CurrencyDefinitionError::Removed
        ));
        assert!(matches!(
            deactivate_error,
            super::CurrencyDefinitionError::Removed
        ));
        assert!(matches!(
            symbol_error,
            super::CurrencyDefinitionError::Removed
        ));
        assert!(matches!(
            name_error,
            super::CurrencyDefinitionError::Removed
        ));
        assert!(matches!(
            remove_error,
            super::CurrencyDefinitionError::Removed
        ));
    }
}
