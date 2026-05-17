mod currency_activate_rejection_reason;
mod currency_activate_result;
mod currency_deactivate_rejection_reason;
mod currency_deactivate_result;
mod currency_decimals;
mod currency_description;
mod currency_description_change_rejection_reason;
mod currency_description_change_result;
mod currency_description_error;
mod currency_error;
mod currency_event_payload;
mod currency_event_payload_error;
mod currency_id;
mod currency_image_change_rejection_reason;
mod currency_image_change_result;
mod currency_image_object_name;
mod currency_image_object_name_error;
mod currency_image_ref;
mod currency_image_url;
mod currency_image_url_error;
mod currency_name;
mod currency_name_change_rejection_reason;
mod currency_name_change_result;
mod currency_name_error;
mod currency_owner;
mod currency_ownership_transfer_rejection_reason;
mod currency_ownership_transfer_result;
mod currency_remove_rejection_reason;
mod currency_remove_result;
mod currency_state;
mod currency_state_error;
mod currency_status;
mod currency_supply_decrease_rejection_reason;
mod currency_supply_decrease_result;
mod currency_supply_increase_rejection_reason;
mod currency_supply_increase_result;
mod currency_symbol;
mod currency_symbol_change_rejection_reason;
mod currency_symbol_change_result;
mod currency_symbol_error;

pub use currency_activate_rejection_reason::CurrencyActivateRejectionReason;
pub use currency_activate_result::CurrencyActivateResult;
pub use currency_deactivate_rejection_reason::CurrencyDeactivateRejectionReason;
pub use currency_deactivate_result::CurrencyDeactivateResult;
pub use currency_decimals::CurrencyDecimals;
pub use currency_description::CurrencyDescription;
pub use currency_description_change_rejection_reason::CurrencyDescriptionChangeRejectionReason;
pub use currency_description_change_result::CurrencyDescriptionChangeResult;
pub use currency_description_error::CurrencyDescriptionError;
pub use currency_error::CurrencyError;
pub use currency_event_payload::CurrencyEventPayload;
pub use currency_event_payload_error::CurrencyEventPayloadError;
pub use currency_id::CurrencyId;
pub use currency_image_change_rejection_reason::CurrencyImageChangeRejectionReason;
pub use currency_image_change_result::CurrencyImageChangeResult;
pub use currency_image_object_name::CurrencyImageObjectName;
pub use currency_image_object_name_error::CurrencyImageObjectNameError;
pub use currency_image_ref::CurrencyImageRef;
pub use currency_image_url::CurrencyImageUrl;
pub use currency_image_url_error::CurrencyImageUrlError;
pub use currency_name::CurrencyName;
pub use currency_name_change_rejection_reason::CurrencyNameChangeRejectionReason;
pub use currency_name_change_result::CurrencyNameChangeResult;
pub use currency_name_error::CurrencyNameError;
pub use currency_owner::CurrencyOwner;
pub use currency_ownership_transfer_rejection_reason::CurrencyOwnershipTransferRejectionReason;
pub use currency_ownership_transfer_result::CurrencyOwnershipTransferResult;
pub use currency_remove_rejection_reason::CurrencyRemoveRejectionReason;
pub use currency_remove_result::CurrencyRemoveResult;
pub use currency_state::CurrencyState;
pub use currency_state_error::CurrencyStateError;
pub use currency_status::CurrencyStatus;
pub use currency_supply_decrease_rejection_reason::CurrencySupplyDecreaseRejectionReason;
pub use currency_supply_decrease_result::CurrencySupplyDecreaseResult;
pub use currency_supply_increase_rejection_reason::CurrencySupplyIncreaseRejectionReason;
pub use currency_supply_increase_result::CurrencySupplyIncreaseResult;
pub use currency_symbol::CurrencySymbol;
pub use currency_symbol_change_rejection_reason::CurrencySymbolChangeRejectionReason;
pub use currency_symbol_change_result::CurrencySymbolChangeResult;
pub use currency_symbol_error::CurrencySymbolError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{CurrencyAmount, CurrencyAmountError};

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

    /// Returns the current description.
    pub fn description(&self) -> Result<Option<&CurrencyDescription>, CurrencyError> {
        Ok(self.state_required()?.description.as_ref())
    }

    /// Returns the current image reference.
    pub fn image(&self) -> Result<Option<&CurrencyImageRef>, CurrencyError> {
        Ok(self.state_required()?.image.as_ref())
    }

    /// Returns the total supply.
    pub fn supply(&self) -> Result<&CurrencyAmount, CurrencyError> {
        Ok(&self.state_required()?.supply)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<CurrencyStatus, CurrencyError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the currency is active.
    pub fn is_active(&self) -> Result<bool, CurrencyError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Returns whether the currency is removed.
    pub fn is_removed(&self) -> Result<bool, CurrencyError> {
        Ok(self.state_required()?.status.is_removed())
    }

    /// Defines a new currency.
    pub fn define(
        &mut self,
        owner: CurrencyOwner,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        description: Option<CurrencyDescription>,
        image: Option<CurrencyImageRef>,
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
            description,
            image,
        })
    }

    /// Transfers ownership of the currency.
    pub fn transfer_ownership(
        &mut self,
        owner: CurrencyOwner,
    ) -> Result<CurrencyOwnershipTransferResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyOwnershipTransferRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::OwnershipTransferRejected { owner, reason })?;
            return Ok(CurrencyOwnershipTransferResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::OwnershipTransferred { owner })?;
        Ok(CurrencyOwnershipTransferResult::Transferred)
    }

    /// Changes the current currency symbol.
    pub fn change_symbol(
        &mut self,
        symbol: CurrencySymbol,
    ) -> Result<CurrencySymbolChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencySymbolChangeRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::SymbolChangeRejected { symbol, reason })?;
            return Ok(CurrencySymbolChangeResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::SymbolChanged { symbol })?;
        Ok(CurrencySymbolChangeResult::Changed)
    }

    /// Changes the current currency name.
    pub fn change_name(
        &mut self,
        name: CurrencyName,
    ) -> Result<CurrencyNameChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyNameChangeRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::NameChangeRejected { name, reason })?;
            return Ok(CurrencyNameChangeResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::NameChanged { name })?;
        Ok(CurrencyNameChangeResult::Changed)
    }

    /// Changes the current currency description.
    pub fn change_description(
        &mut self,
        description: Option<CurrencyDescription>,
    ) -> Result<CurrencyDescriptionChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyDescriptionChangeRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::DescriptionChangeRejected {
                description,
                reason,
            })?;
            return Ok(CurrencyDescriptionChangeResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::DescriptionChanged { description })?;
        Ok(CurrencyDescriptionChangeResult::Changed)
    }

    /// Changes the current currency image reference.
    pub fn change_image(
        &mut self,
        image: Option<CurrencyImageRef>,
    ) -> Result<CurrencyImageChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyImageChangeRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::ImageChangeRejected { image, reason })?;
            return Ok(CurrencyImageChangeResult::Rejected { reason });
        }

        let old_image = self.state_required()?.image.clone();
        self.append_event(CurrencyEventPayload::ImageChanged { image, old_image })?;
        Ok(CurrencyImageChangeResult::Changed)
    }

    /// Increases the total supply.
    pub fn increase_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<CurrencySupplyIncreaseResult, CurrencyError> {
        match self.state_required()?.status {
            CurrencyStatus::Active => {}
            CurrencyStatus::Inactive => {
                let reason = CurrencySupplyIncreaseRejectionReason::Inactive;
                self.append_event(CurrencyEventPayload::SupplyIncreaseRejected { amount, reason })?;
                return Ok(CurrencySupplyIncreaseResult::Rejected { reason });
            }
            CurrencyStatus::Removed => {
                let reason = CurrencySupplyIncreaseRejectionReason::Removed;
                self.append_event(CurrencyEventPayload::SupplyIncreaseRejected { amount, reason })?;
                return Ok(CurrencySupplyIncreaseResult::Rejected { reason });
            }
        }

        self.append_event(CurrencyEventPayload::SupplyIncreased { amount })?;
        Ok(CurrencySupplyIncreaseResult::Increased)
    }

    /// Decreases the total supply.
    pub fn decrease_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<CurrencySupplyDecreaseResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencySupplyDecreaseRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::SupplyDecreaseRejected { amount, reason })?;
            return Ok(CurrencySupplyDecreaseResult::Rejected { reason });
        }

        if self.state_required()?.supply.value() < amount.value() {
            let reason = CurrencySupplyDecreaseRejectionReason::InsufficientSupply;
            self.append_event(CurrencyEventPayload::SupplyDecreaseRejected { amount, reason })?;
            return Ok(CurrencySupplyDecreaseResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::SupplyDecreased { amount })?;
        Ok(CurrencySupplyDecreaseResult::Decreased)
    }

    /// Activates the currency.
    pub fn activate(&mut self) -> Result<CurrencyActivateResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyActivateRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::ActivateRejected { reason })?;
            return Ok(CurrencyActivateResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::Activated)?;
        Ok(CurrencyActivateResult::Activated)
    }

    /// Deactivates the currency.
    pub fn deactivate(&mut self) -> Result<CurrencyDeactivateResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyDeactivateRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::DeactivateRejected { reason })?;
            return Ok(CurrencyDeactivateResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::Deactivated)?;
        Ok(CurrencyDeactivateResult::Deactivated)
    }

    /// Permanently removes the currency.
    pub fn remove(&mut self) -> Result<CurrencyRemoveResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyRemoveRejectionReason::Removed;
            self.append_event(CurrencyEventPayload::RemoveRejected { reason })?;
            return Ok(CurrencyRemoveResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::Removed)?;
        Ok(CurrencyRemoveResult::Removed)
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
                description,
                image,
            } => {
                self.set_state(Some(CurrencyState {
                    id: *id,
                    owner: *owner,
                    symbol: symbol.clone(),
                    name: name.clone(),
                    decimals: *decimals,
                    description: description.clone(),
                    image: image.clone(),
                    supply: CurrencyAmount::zero(),
                    status: CurrencyStatus::Active,
                }));
            }
            CurrencyEventPayload::OwnershipTransferred { owner } => {
                self.state_required_mut()?.owner = *owner;
            }
            CurrencyEventPayload::OwnershipTransferRejected { .. } => {}
            CurrencyEventPayload::SymbolChanged { symbol } => {
                self.state_required_mut()?.symbol = symbol.clone();
            }
            CurrencyEventPayload::SymbolChangeRejected { .. } => {}
            CurrencyEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone();
            }
            CurrencyEventPayload::NameChangeRejected { .. } => {}
            CurrencyEventPayload::DescriptionChanged { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            CurrencyEventPayload::DescriptionChangeRejected { .. } => {}
            CurrencyEventPayload::ImageChanged { image, .. } => {
                self.state_required_mut()?.image = image.clone();
            }
            CurrencyEventPayload::ImageChangeRejected { .. } => {}
            CurrencyEventPayload::SupplyIncreased { amount } => {
                let state = self.state_required_mut()?;
                state.supply = state.supply.try_add(*amount).map_err(|error| match error {
                    CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                    CurrencyAmountError::InsufficientBalance => CurrencyError::InsufficientSupply,
                })?;
            }
            CurrencyEventPayload::SupplyIncreaseRejected { .. } => {}
            CurrencyEventPayload::SupplyDecreased { amount } => {
                let state = self.state_required_mut()?;
                state.supply = state.supply.try_sub(*amount).map_err(|error| match error {
                    CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                    CurrencyAmountError::InsufficientBalance => CurrencyError::InsufficientSupply,
                })?;
            }
            CurrencyEventPayload::SupplyDecreaseRejected { .. } => {}
            CurrencyEventPayload::Activated => {
                self.state_required_mut()?.status = CurrencyStatus::Active;
            }
            CurrencyEventPayload::ActivateRejected { .. } => {}
            CurrencyEventPayload::Deactivated => {
                self.state_required_mut()?.status = CurrencyStatus::Inactive;
            }
            CurrencyEventPayload::DeactivateRejected { .. } => {}
            CurrencyEventPayload::Removed => {
                self.state_required_mut()?.status = CurrencyStatus::Removed;
            }
            CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::core::CurrencyAmount;
    use banking_iam_domain::{OrganizationId, UserId};

    use super::{
        Currency, CurrencyDecimals, CurrencyDescription, CurrencyEventPayload, CurrencyId,
        CurrencyImageRef, CurrencyImageUrl, CurrencyName, CurrencyOwner, CurrencyStatus,
        CurrencySymbol,
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
            .define(
                owner,
                symbol.clone(),
                name.clone(),
                decimals,
                Some(
                    CurrencyDescription::try_from("Stablecoin backed by USD")
                        .expect("description should be valid"),
                ),
                Some(CurrencyImageRef::external_url(
                    CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
                        .expect("image URL should be valid"),
                )),
            )
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
        assert_eq!(
            currency
                .description()
                .expect("description should exist")
                .map(CurrencyDescription::value),
            Some("Stablecoin backed by USD")
        );
        assert_eq!(
            currency
                .image()
                .expect("image should exist")
                .and_then(CurrencyImageRef::as_external_url)
                .map(|value| value.value().as_str()),
            Some("https://cdn.example.com/currencies/usdc.png")
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
                description: Some(
                    CurrencyDescription::try_from("Stablecoin backed by USD")
                        .expect("description should be valid"),
                ),
                image: Some(CurrencyImageRef::external_url(
                    CurrencyImageUrl::try_from("https://cdn.example.com/currencies/usdc.png")
                        .expect("image URL should be valid"),
                )),
            }
        );
    }

    #[test]
    fn changing_to_same_values_and_same_status_appends_success_events() {
        let owner = user_owner();
        let symbol = CurrencySymbol::try_from("usdc").expect("symbol should be valid");
        let name = CurrencyName::try_from("USD Coin").expect("name should be valid");
        let decimals = CurrencyDecimals::new(6);
        let mut currency = Currency::default();
        currency
            .define(owner, symbol.clone(), name.clone(), decimals, None, None)
            .expect("definition should succeed");

        currency
            .change_symbol(symbol)
            .expect("symbol change should succeed");
        currency
            .change_name(name)
            .expect("name change should succeed");
        currency.activate().expect("activation should succeed");

        assert_eq!(currency.uncommitted_events().len(), 4);
    }

    #[test]
    fn transferring_to_same_owner_appends_success_event() {
        let owner = user_owner();
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
                None,
                None,
            )
            .expect("definition should succeed");

        currency
            .transfer_ownership(owner)
            .expect("same owner transfer should succeed");

        assert_eq!(currency.uncommitted_events().len(), 2);
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
                None,
                None,
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
    fn transfer_ownership_updates_owner_and_records_event() {
        let original_owner = user_owner();
        let transferred_owner = organization_owner();
        let mut currency = Currency::default();
        currency
            .define(
                original_owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
                None,
                None,
            )
            .expect("definition should succeed");

        currency
            .transfer_ownership(transferred_owner)
            .expect("ownership transfer should succeed");

        assert_eq!(
            currency.owner().expect("owner should exist"),
            transferred_owner
        );
        assert_eq!(currency.uncommitted_events().len(), 2);
        assert_eq!(
            currency.uncommitted_events()[1].payload().name(),
            CurrencyEventPayload::OWNERSHIP_TRANSFERRED
        );
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
                owner,
                symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                name: CurrencyName::try_from("USD Coin").expect("name should be valid"),
                decimals: CurrencyDecimals::new(6),
                description: None,
                image: None,
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
            .define(owner, symbol, name, CurrencyDecimals::new(6), None, None)
            .expect("definition should succeed");

        assert_eq!(currency.owner().expect("owner should exist"), owner);
    }

    #[test]
    fn define_rejects_already_defined_currency() {
        let owner = user_owner();
        let mut currency = Currency::default();
        currency
            .define(
                owner,
                CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
                CurrencyName::try_from("USD Coin").expect("name should be valid"),
                CurrencyDecimals::new(6),
                None,
                None,
            )
            .expect("definition should succeed");

        let error = currency
            .define(
                owner,
                CurrencySymbol::try_from("sol").expect("symbol should be valid"),
                CurrencyName::try_from("Solana").expect("name should be valid"),
                CurrencyDecimals::new(9),
                None,
                None,
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
                None,
                None,
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
                None,
                None,
            )
            .expect("definition should succeed");
        currency.deactivate().expect("deactivate should succeed");

        let result = currency
            .increase_supply(CurrencyAmount::new(1))
            .expect("increase should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencySupplyIncreaseResult::Rejected {
                reason: super::CurrencySupplyIncreaseRejectionReason::Inactive
            }
        ));
        assert_eq!(
            currency.uncommitted_events()[2].payload().name(),
            CurrencyEventPayload::SUPPLY_INCREASE_REJECTED
        );
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
                None,
                None,
            )
            .expect("definition should succeed");

        currency.remove().expect("remove should succeed");
        let duplicate_remove_result = currency
            .remove()
            .expect("duplicate remove should complete with a rejection event");

        assert_eq!(
            currency.status().expect("status should exist"),
            CurrencyStatus::Removed
        );
        assert_eq!(currency.uncommitted_events().len(), 3);
        assert!(matches!(
            duplicate_remove_result,
            super::CurrencyRemoveResult::Rejected {
                reason: super::CurrencyRemoveRejectionReason::Removed
            }
        ));
        assert_eq!(
            currency.uncommitted_events()[1].payload().name(),
            CurrencyEventPayload::REMOVED
        );
        assert_eq!(
            currency.uncommitted_events()[2].payload().name(),
            CurrencyEventPayload::REMOVE_REJECTED
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
                None,
                None,
            )
            .expect("definition should succeed");
        currency.remove().expect("remove should succeed");

        let activate_result = currency
            .activate()
            .expect("activate should complete with a rejection event");
        let deactivate_result = currency
            .deactivate()
            .expect("deactivate should complete with a rejection event");
        let symbol_result = currency
            .change_symbol(CurrencySymbol::try_from("usdce").expect("symbol should be valid"))
            .expect("symbol change should complete with a rejection event");
        let name_result = currency
            .change_name(CurrencyName::try_from("USD Coin Example").expect("name should be valid"))
            .expect("name change should complete with a rejection event");
        let remove_result = currency
            .remove()
            .expect("remove should complete with a rejection event");

        assert!(matches!(
            activate_result,
            super::CurrencyActivateResult::Rejected {
                reason: super::CurrencyActivateRejectionReason::Removed
            }
        ));
        assert!(matches!(
            deactivate_result,
            super::CurrencyDeactivateResult::Rejected {
                reason: super::CurrencyDeactivateRejectionReason::Removed
            }
        ));
        assert!(matches!(
            symbol_result,
            super::CurrencySymbolChangeResult::Rejected {
                reason: super::CurrencySymbolChangeRejectionReason::Removed
            }
        ));
        assert!(matches!(
            name_result,
            super::CurrencyNameChangeResult::Rejected {
                reason: super::CurrencyNameChangeRejectionReason::Removed
            }
        ));
        assert!(matches!(
            remove_result,
            super::CurrencyRemoveResult::Rejected {
                reason: super::CurrencyRemoveRejectionReason::Removed
            }
        ));
    }
}
