mod currency_activate_rejection_reason;
mod currency_activate_result;
mod currency_deactivate_rejection_reason;
mod currency_deactivate_result;
mod currency_decimals;
mod currency_define_result;
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
mod currency_mint_account;
mod currency_mint_account_address;
mod currency_mint_account_address_error;
mod currency_mint_account_creation_request_rejection_reason;
mod currency_mint_account_creation_request_result;
mod currency_mint_account_record_rejection_reason;
mod currency_mint_account_record_result;
mod currency_mint_token_program_id;
mod currency_mint_token_program_id_error;
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
pub use currency_define_result::CurrencyDefineResult;
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
pub use currency_mint_account::CurrencyMintAccount;
pub use currency_mint_account_address::CurrencyMintAccountAddress;
pub use currency_mint_account_address_error::CurrencyMintAccountAddressError;
pub use currency_mint_account_creation_request_rejection_reason::CurrencyMintAccountCreationRequestRejectionReason;
pub use currency_mint_account_creation_request_result::CurrencyMintAccountCreationRequestResult;
pub use currency_mint_account_record_rejection_reason::CurrencyMintAccountRecordRejectionReason;
pub use currency_mint_account_record_result::CurrencyMintAccountRecordResult;
pub use currency_mint_token_program_id::CurrencyMintTokenProgramId;
pub use currency_mint_token_program_id_error::CurrencyMintTokenProgramIdError;
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

    /// Returns the linked on-chain mint account.
    pub fn mint_account(&self) -> Result<Option<&CurrencyMintAccount>, CurrencyError> {
        Ok(self.state_required()?.mint_account.as_ref())
    }

    /// Returns whether mint account creation has been requested.
    pub fn is_mint_account_creation_requested(&self) -> Result<bool, CurrencyError> {
        Ok(self.state_required()?.mint_account_creation_requested)
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
    ) -> Result<CurrencyDefineResult, CurrencyError> {
        if self.state().is_some() {
            return Err(CurrencyError::AlreadyDefined);
        }

        let currency_id = CurrencyId::new();
        self.append_event(CurrencyEventPayload::Defined {
            id: currency_id,
            owner,
            symbol,
            name,
            decimals,
            description,
            image,
        })?;

        Ok(CurrencyDefineResult::Defined { currency_id })
    }

    /// Transfers ownership of the currency.
    pub fn transfer_ownership(
        &mut self,
        owner: CurrencyOwner,
    ) -> Result<CurrencyOwnershipTransferResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyOwnershipTransferRejectionReason::Removed;
            self.reject_transfer_ownership(owner, reason)?;
            return Ok(CurrencyOwnershipTransferResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::OwnershipTransferred { owner })?;
        Ok(CurrencyOwnershipTransferResult::Transferred)
    }

    /// Rejects a currency ownership transfer attempt.
    pub fn reject_transfer_ownership(
        &mut self,
        owner: CurrencyOwner,
        reason: CurrencyOwnershipTransferRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::OwnershipTransferRejected { owner, reason })?;
        Ok(())
    }

    /// Changes the current currency symbol.
    pub fn change_symbol(
        &mut self,
        symbol: CurrencySymbol,
    ) -> Result<CurrencySymbolChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencySymbolChangeRejectionReason::Removed;
            self.reject_change_symbol(symbol, reason)?;
            return Ok(CurrencySymbolChangeResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::SymbolChanged { symbol })?;
        Ok(CurrencySymbolChangeResult::Changed)
    }

    /// Rejects a currency symbol change attempt.
    pub fn reject_change_symbol(
        &mut self,
        symbol: CurrencySymbol,
        reason: CurrencySymbolChangeRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::SymbolChangeRejected { symbol, reason })?;
        Ok(())
    }

    /// Changes the current currency name.
    pub fn change_name(
        &mut self,
        name: CurrencyName,
    ) -> Result<CurrencyNameChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyNameChangeRejectionReason::Removed;
            self.reject_change_name(name, reason)?;
            return Ok(CurrencyNameChangeResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::NameChanged { name })?;
        Ok(CurrencyNameChangeResult::Changed)
    }

    /// Rejects a currency name change attempt.
    pub fn reject_change_name(
        &mut self,
        name: CurrencyName,
        reason: CurrencyNameChangeRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::NameChangeRejected { name, reason })?;
        Ok(())
    }

    /// Changes the current currency description.
    pub fn change_description(
        &mut self,
        description: Option<CurrencyDescription>,
    ) -> Result<CurrencyDescriptionChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyDescriptionChangeRejectionReason::Removed;
            self.reject_change_description(description, reason)?;
            return Ok(CurrencyDescriptionChangeResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::DescriptionChanged { description })?;
        Ok(CurrencyDescriptionChangeResult::Changed)
    }

    /// Rejects a currency description change attempt.
    pub fn reject_change_description(
        &mut self,
        description: Option<CurrencyDescription>,
        reason: CurrencyDescriptionChangeRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::DescriptionChangeRejected {
            description,
            reason,
        })?;
        Ok(())
    }

    /// Changes the current currency image reference.
    pub fn change_image(
        &mut self,
        image: Option<CurrencyImageRef>,
    ) -> Result<CurrencyImageChangeResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyImageChangeRejectionReason::Removed;
            self.reject_change_image(image, reason)?;
            return Ok(CurrencyImageChangeResult::Rejected { reason });
        }

        let old_image = self.state_required()?.image.clone();
        self.append_event(CurrencyEventPayload::ImageChanged { image, old_image })?;
        Ok(CurrencyImageChangeResult::Changed)
    }

    /// Rejects a currency image change attempt.
    pub fn reject_change_image(
        &mut self,
        image: Option<CurrencyImageRef>,
        reason: CurrencyImageChangeRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::ImageChangeRejected { image, reason })?;
        Ok(())
    }

    /// Requests creation of the on-chain mint account linked to this currency.
    pub fn request_mint_account_creation(
        &mut self,
    ) -> Result<CurrencyMintAccountCreationRequestResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyMintAccountCreationRequestRejectionReason::Removed;
            self.reject_request_mint_account_creation(reason)?;
            return Ok(CurrencyMintAccountCreationRequestResult::Rejected { reason });
        }

        if self.state_required()?.mint_account.is_some() {
            let reason = CurrencyMintAccountCreationRequestRejectionReason::AlreadyRecorded;
            self.reject_request_mint_account_creation(reason)?;
            return Ok(CurrencyMintAccountCreationRequestResult::Rejected { reason });
        }

        if self.state_required()?.mint_account_creation_requested {
            let reason = CurrencyMintAccountCreationRequestRejectionReason::AlreadyRequested;
            self.reject_request_mint_account_creation(reason)?;
            return Ok(CurrencyMintAccountCreationRequestResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::MintAccountCreationRequested)?;
        Ok(CurrencyMintAccountCreationRequestResult::Requested)
    }

    /// Rejects a mint account creation request.
    pub fn reject_request_mint_account_creation(
        &mut self,
        reason: CurrencyMintAccountCreationRequestRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::MintAccountCreationRequestRejected { reason })?;
        Ok(())
    }

    /// Records the on-chain mint account linked to this currency.
    pub fn record_mint_account(
        &mut self,
        mint_account: CurrencyMintAccount,
    ) -> Result<CurrencyMintAccountRecordResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyMintAccountRecordRejectionReason::Removed;
            self.reject_record_mint_account(Some(mint_account), reason)?;
            return Ok(CurrencyMintAccountRecordResult::Rejected { reason });
        }

        if self.state_required()?.mint_account.is_some() {
            let reason = CurrencyMintAccountRecordRejectionReason::AlreadyRecorded;
            self.reject_record_mint_account(Some(mint_account), reason)?;
            return Ok(CurrencyMintAccountRecordResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::MintAccountRecorded {
            mint_account: mint_account.clone(),
        })?;
        Ok(CurrencyMintAccountRecordResult::Recorded { mint_account })
    }

    /// Rejects mint account recording.
    pub fn reject_record_mint_account(
        &mut self,
        mint_account: Option<CurrencyMintAccount>,
        reason: CurrencyMintAccountRecordRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::MintAccountRecordRejected {
            mint_account,
            reason,
        })?;
        Ok(())
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
                self.reject_increase_supply(amount, reason)?;
                return Ok(CurrencySupplyIncreaseResult::Rejected { reason });
            }
            CurrencyStatus::Removed => {
                let reason = CurrencySupplyIncreaseRejectionReason::Removed;
                self.reject_increase_supply(amount, reason)?;
                return Ok(CurrencySupplyIncreaseResult::Rejected { reason });
            }
        }

        self.append_event(CurrencyEventPayload::SupplyIncreased { amount })?;
        Ok(CurrencySupplyIncreaseResult::Increased)
    }

    /// Rejects a currency supply increase attempt.
    pub fn reject_increase_supply(
        &mut self,
        amount: CurrencyAmount,
        reason: CurrencySupplyIncreaseRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::SupplyIncreaseRejected { amount, reason })?;
        Ok(())
    }

    /// Decreases the total supply.
    pub fn decrease_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<CurrencySupplyDecreaseResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencySupplyDecreaseRejectionReason::Removed;
            self.reject_decrease_supply(amount, reason)?;
            return Ok(CurrencySupplyDecreaseResult::Rejected { reason });
        }

        if self.state_required()?.supply.value() < amount.value() {
            let reason = CurrencySupplyDecreaseRejectionReason::InsufficientSupply;
            self.reject_decrease_supply(amount, reason)?;
            return Ok(CurrencySupplyDecreaseResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::SupplyDecreased { amount })?;
        Ok(CurrencySupplyDecreaseResult::Decreased)
    }

    /// Rejects a currency supply decrease attempt.
    pub fn reject_decrease_supply(
        &mut self,
        amount: CurrencyAmount,
        reason: CurrencySupplyDecreaseRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::SupplyDecreaseRejected { amount, reason })?;
        Ok(())
    }

    /// Activates the currency.
    pub fn activate(&mut self) -> Result<CurrencyActivateResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyActivateRejectionReason::Removed;
            self.reject_activate(reason)?;
            return Ok(CurrencyActivateResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::Activated)?;
        Ok(CurrencyActivateResult::Activated)
    }

    /// Rejects a currency activation attempt.
    pub fn reject_activate(
        &mut self,
        reason: CurrencyActivateRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::ActivateRejected { reason })?;
        Ok(())
    }

    /// Deactivates the currency.
    pub fn deactivate(&mut self) -> Result<CurrencyDeactivateResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyDeactivateRejectionReason::Removed;
            self.reject_deactivate(reason)?;
            return Ok(CurrencyDeactivateResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::Deactivated)?;
        Ok(CurrencyDeactivateResult::Deactivated)
    }

    /// Rejects a currency deactivation attempt.
    pub fn reject_deactivate(
        &mut self,
        reason: CurrencyDeactivateRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::DeactivateRejected { reason })?;
        Ok(())
    }

    /// Permanently removes the currency.
    pub fn remove(&mut self) -> Result<CurrencyRemoveResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyRemoveRejectionReason::Removed;
            self.reject_remove(reason)?;
            return Ok(CurrencyRemoveResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::Removed)?;
        Ok(CurrencyRemoveResult::Removed)
    }

    /// Rejects a currency removal attempt.
    pub fn reject_remove(
        &mut self,
        reason: CurrencyRemoveRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::RemoveRejected { reason })?;
        Ok(())
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
                    mint_account_creation_requested: false,
                    mint_account: None,
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
            CurrencyEventPayload::MintAccountCreationRequested => {
                self.state_required_mut()?.mint_account_creation_requested = true;
            }
            CurrencyEventPayload::MintAccountCreationRequestRejected { .. } => {}
            CurrencyEventPayload::MintAccountRecorded { mint_account } => {
                let state = self.state_required_mut()?;
                state.mint_account_creation_requested = false;
                state.mint_account = Some(mint_account.clone());
            }
            CurrencyEventPayload::MintAccountRecordRejected { .. } => {
                self.state_required_mut()?.mint_account_creation_requested = false;
            }
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
        CurrencyImageRef, CurrencyImageUrl, CurrencyMintAccount, CurrencyMintAccountAddress,
        CurrencyMintAccountCreationRequestRejectionReason, CurrencyMintTokenProgramId,
        CurrencyName, CurrencyOwner, CurrencyStatus, CurrencySymbol,
    };

    fn user_owner() -> CurrencyOwner {
        CurrencyOwner::user(UserId::new())
    }

    fn organization_owner() -> CurrencyOwner {
        CurrencyOwner::organization(OrganizationId::new())
    }

    fn make_mint_account(value: &str) -> CurrencyMintAccount {
        CurrencyMintAccount::new(
            CurrencyMintAccountAddress::try_from(value)
                .expect("mint account address should be valid"),
            CurrencyMintTokenProgramId::try_from("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
                .expect("token program ID should be valid"),
        )
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
        assert!(
            !currency
                .is_mint_account_creation_requested()
                .expect("mint account creation request state should exist")
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
    fn request_mint_account_creation_records_event_and_updates_state() {
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

        let result = currency
            .request_mint_account_creation()
            .expect("mint account creation request should succeed");

        assert_eq!(
            result,
            super::CurrencyMintAccountCreationRequestResult::Requested
        );
        assert!(
            currency
                .is_mint_account_creation_requested()
                .expect("mint account creation request state should exist")
        );
        assert_eq!(
            currency.uncommitted_events()[1].payload(),
            &CurrencyEventPayload::MintAccountCreationRequested
        );
    }

    #[test]
    fn request_mint_account_creation_is_idempotent_after_already_requested() {
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
            .request_mint_account_creation()
            .expect("initial request should succeed");
        currency.core_mut().clear_uncommitted_events();

        let result = currency
            .request_mint_account_creation()
            .expect("duplicate request should succeed");

        assert_eq!(
            result,
            super::CurrencyMintAccountCreationRequestResult::Rejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRequested,
            }
        );
        assert_eq!(
            currency.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountCreationRequestRejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRequested,
            }
        );
    }

    #[test]
    fn request_mint_account_creation_rejects_removed_currency_with_event() {
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
        currency.core_mut().clear_uncommitted_events();

        let result = currency
            .request_mint_account_creation()
            .expect("request should complete");

        assert_eq!(
            result,
            super::CurrencyMintAccountCreationRequestResult::Rejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::Removed,
            }
        );
        assert_eq!(
            currency.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountCreationRequestRejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::Removed,
            }
        );
    }

    #[test]
    fn request_mint_account_creation_rejects_already_recorded_currency_with_event() {
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
            .record_mint_account(make_mint_account(
                "Mint111111111111111111111111111111111111",
            ))
            .expect("mint account should be recorded");
        currency.core_mut().clear_uncommitted_events();

        let result = currency
            .request_mint_account_creation()
            .expect("request should complete");

        assert_eq!(
            result,
            super::CurrencyMintAccountCreationRequestResult::Rejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRecorded,
            }
        );
        assert_eq!(
            currency.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountCreationRequestRejected {
                reason: CurrencyMintAccountCreationRequestRejectionReason::AlreadyRecorded,
            }
        );
    }

    #[test]
    fn record_mint_account_updates_state_and_records_event() {
        let owner = user_owner();
        let mint_account = make_mint_account("Mint111111111111111111111111111111111111");
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
            .request_mint_account_creation()
            .expect("mint account creation request should succeed");
        currency.core_mut().clear_uncommitted_events();

        let result = currency
            .record_mint_account(mint_account.clone())
            .expect("mint account record should succeed");

        assert!(matches!(
            result,
            super::CurrencyMintAccountRecordResult::Recorded {
                mint_account: recorded
            } if recorded == mint_account
        ));
        assert_eq!(
            currency.mint_account().expect("mint account should exist"),
            Some(&mint_account)
        );
        assert!(
            !currency
                .is_mint_account_creation_requested()
                .expect("mint account creation request state should exist")
        );
        assert_eq!(
            currency.uncommitted_events()[0].payload().name(),
            CurrencyEventPayload::MINT_ACCOUNT_RECORDED
        );
    }

    #[test]
    fn record_mint_account_rejects_duplicate_with_event() {
        let owner = user_owner();
        let mint_account = make_mint_account("Mint111111111111111111111111111111111111");
        let duplicate_mint_account = make_mint_account("Mint222222222222222222222222222222222222");
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
            .record_mint_account(mint_account.clone())
            .expect("mint account record should succeed");

        let result = currency
            .record_mint_account(duplicate_mint_account)
            .expect("duplicate mint account record should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencyMintAccountRecordResult::Rejected {
                reason: super::CurrencyMintAccountRecordRejectionReason::AlreadyRecorded
            }
        ));
        assert_eq!(
            currency.mint_account().expect("mint account should exist"),
            Some(&mint_account)
        );
        assert_eq!(
            currency.uncommitted_events()[2].payload().name(),
            CurrencyEventPayload::MINT_ACCOUNT_RECORD_REJECTED
        );
    }

    #[test]
    fn record_mint_account_rejects_removed_currency_with_event() {
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

        let result = currency
            .record_mint_account(make_mint_account(
                "Mint111111111111111111111111111111111111",
            ))
            .expect("removed mint account record should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencyMintAccountRecordResult::Rejected {
                reason: super::CurrencyMintAccountRecordRejectionReason::Removed
            }
        ));
        assert_eq!(
            currency.uncommitted_events()[2].payload().name(),
            CurrencyEventPayload::MINT_ACCOUNT_RECORD_REJECTED
        );
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
