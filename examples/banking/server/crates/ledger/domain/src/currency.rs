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
mod currency_mint_account_metadata_sync_rejection_reason;
mod currency_name;
mod currency_name_change_rejection_reason;
mod currency_name_change_result;
mod currency_name_error;
mod currency_owner;
mod currency_ownership_transfer_rejection_reason;
mod currency_ownership_transfer_result;
mod currency_pool_token_account_address;
mod currency_pool_token_account_address_error;
mod currency_provision_rejection_reason;
mod currency_provision_result;
mod currency_provisioning_status;
mod currency_remove_rejection_reason;
mod currency_remove_result;
mod currency_state;
mod currency_state_error;
mod currency_status;
mod currency_supply_commit_rejection_reason;
mod currency_supply_commit_result;
mod currency_supply_release_rejection_reason;
mod currency_supply_release_result;
mod currency_supply_reserve_rejection_reason;
mod currency_supply_reserve_result;
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
pub use currency_mint_account_metadata_sync_rejection_reason::CurrencyMintAccountMetadataSyncRejectionReason;
pub use currency_name::CurrencyName;
pub use currency_name_change_rejection_reason::CurrencyNameChangeRejectionReason;
pub use currency_name_change_result::CurrencyNameChangeResult;
pub use currency_name_error::CurrencyNameError;
pub use currency_owner::CurrencyOwner;
pub use currency_ownership_transfer_rejection_reason::CurrencyOwnershipTransferRejectionReason;
pub use currency_ownership_transfer_result::CurrencyOwnershipTransferResult;
pub use currency_pool_token_account_address::CurrencyPoolTokenAccountAddress;
pub use currency_pool_token_account_address_error::CurrencyPoolTokenAccountAddressError;
pub use currency_provision_rejection_reason::CurrencyProvisionRejectionReason;
pub use currency_provision_result::CurrencyProvisionResult;
pub use currency_provisioning_status::CurrencyProvisioningStatus;
pub use currency_remove_rejection_reason::CurrencyRemoveRejectionReason;
pub use currency_remove_result::CurrencyRemoveResult;
pub use currency_state::CurrencyState;
pub use currency_state_error::CurrencyStateError;
pub use currency_status::CurrencyStatus;
pub use currency_supply_commit_rejection_reason::CurrencySupplyCommitRejectionReason;
pub use currency_supply_commit_result::CurrencySupplyCommitResult;
pub use currency_supply_release_rejection_reason::CurrencySupplyReleaseRejectionReason;
pub use currency_supply_release_result::CurrencySupplyReleaseResult;
pub use currency_supply_reserve_rejection_reason::CurrencySupplyReserveRejectionReason;
pub use currency_supply_reserve_result::CurrencySupplyReserveResult;
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
        Ok(self.state_required()?.provisioning_status.mint_account())
    }

    /// Returns the total supply.
    pub fn supply(&self) -> Result<&CurrencyAmount, CurrencyError> {
        Ok(&self.state_required()?.supply)
    }

    /// Returns the pending supply that has not yet been committed.
    pub fn pending_supply(&self) -> Result<&CurrencyAmount, CurrencyError> {
        Ok(&self.state_required()?.pending_supply)
    }

    /// Returns the target on-chain supply derived from confirmed and pending supply.
    pub fn target_supply(&self) -> Result<CurrencyAmount, CurrencyError> {
        self.state_required()?
            .supply
            .try_add(self.state_required()?.pending_supply)
            .map_err(|error| match error {
                CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                CurrencyAmountError::InsufficientBalance => CurrencyError::InsufficientSupply,
            })
    }

    /// Returns the current provisioning status.
    pub fn provisioning_status(&self) -> Result<CurrencyProvisioningStatus, CurrencyError> {
        Ok(self.state_required()?.provisioning_status.clone())
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

    /// Returns whether the currency has completed provisioning.
    pub fn is_provisioned(&self) -> Result<bool, CurrencyError> {
        Ok(self.state_required()?.provisioning_status.is_provisioned())
    }

    /// Returns whether the currency provisioning has failed.
    pub fn is_provisioning_failed(&self) -> Result<bool, CurrencyError> {
        Ok(self.state_required()?.provisioning_status.is_failed())
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

    /// Completes currency provisioning with the created on-chain mint account.
    pub fn provision(
        &mut self,
        mint_account: CurrencyMintAccount,
    ) -> Result<CurrencyProvisionResult, CurrencyError> {
        if self.state_required()?.status.is_removed() {
            let reason = CurrencyProvisionRejectionReason::Removed;
            self.reject_provision(Some(mint_account), reason)?;
            return Ok(CurrencyProvisionResult::Rejected { reason });
        }

        if self.state_required()?.provisioning_status.is_provisioned() {
            let reason = CurrencyProvisionRejectionReason::AlreadyProvisioned;
            self.reject_provision(Some(mint_account), reason)?;
            return Ok(CurrencyProvisionResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::Provisioned {
            mint_account: mint_account.clone(),
        })?;
        Ok(CurrencyProvisionResult::Provisioned { mint_account })
    }

    /// Rejects currency provisioning.
    pub fn reject_provision(
        &mut self,
        mint_account: Option<CurrencyMintAccount>,
        reason: CurrencyProvisionRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::ProvisionRejected {
            mint_account,
            reason,
        })?;
        Ok(())
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

    /// Records that mint account metadata has been synced to the current currency metadata.
    pub fn record_mint_account_metadata_synced(&mut self) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::MintAccountMetadataSynced)?;
        Ok(())
    }

    /// Rejects a mint account metadata sync attempt.
    pub fn reject_mint_account_metadata_sync(
        &mut self,
        reason: CurrencyMintAccountMetadataSyncRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::MintAccountMetadataSyncRejected { reason })?;
        Ok(())
    }

    /// Reserves supply for an in-flight issuance before it is committed.
    pub fn reserve_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<CurrencySupplyReserveResult, CurrencyError> {
        if !self.state_required()?.provisioning_status.is_provisioned() {
            let reason = CurrencySupplyReserveRejectionReason::ProvisioningPending;
            self.reject_reserve_supply(amount, reason)?;
            return Ok(CurrencySupplyReserveResult::Rejected { reason });
        }

        match self.state_required()?.status {
            CurrencyStatus::Active => {}
            CurrencyStatus::Inactive => {
                let reason = CurrencySupplyReserveRejectionReason::Inactive;
                self.reject_reserve_supply(amount, reason)?;
                return Ok(CurrencySupplyReserveResult::Rejected { reason });
            }
            CurrencyStatus::Removed => {
                let reason = CurrencySupplyReserveRejectionReason::Removed;
                self.reject_reserve_supply(amount, reason)?;
                return Ok(CurrencySupplyReserveResult::Rejected { reason });
            }
        }

        if self.target_supply()?.try_add(amount).is_err() {
            let reason = CurrencySupplyReserveRejectionReason::SupplyOverflow;
            self.reject_reserve_supply(amount, reason)?;
            return Ok(CurrencySupplyReserveResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::SupplyReserved { amount })?;
        Ok(CurrencySupplyReserveResult::Reserved)
    }

    /// Rejects a currency supply reservation attempt.
    pub fn reject_reserve_supply(
        &mut self,
        amount: CurrencyAmount,
        reason: CurrencySupplyReserveRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::SupplyReserveRejected { amount, reason })?;
        Ok(())
    }

    /// Records that on-chain mint supply has been synced to the current target supply.
    pub fn record_mint_supply_synced(
        &mut self,
        supply: CurrencyAmount,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::MintSupplySynced { supply })?;
        Ok(())
    }

    /// Commits previously reserved supply into confirmed supply.
    pub fn commit_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<CurrencySupplyCommitResult, CurrencyError> {
        if self.state_required()?.pending_supply.value() < amount.value() {
            let reason = CurrencySupplyCommitRejectionReason::InsufficientPendingSupply;
            self.reject_commit_supply(amount, reason)?;
            return Ok(CurrencySupplyCommitResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::SupplyCommitted { amount })?;
        Ok(CurrencySupplyCommitResult::Committed)
    }

    /// Rejects a currency supply commit attempt.
    pub fn reject_commit_supply(
        &mut self,
        amount: CurrencyAmount,
        reason: CurrencySupplyCommitRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::SupplyCommitRejected { amount, reason })?;
        Ok(())
    }

    /// Releases previously reserved supply without changing confirmed supply.
    pub fn release_supply(
        &mut self,
        amount: CurrencyAmount,
    ) -> Result<CurrencySupplyReleaseResult, CurrencyError> {
        if self.state_required()?.pending_supply.value() < amount.value() {
            let reason = CurrencySupplyReleaseRejectionReason::InsufficientPendingSupply;
            self.reject_release_supply(amount, reason)?;
            return Ok(CurrencySupplyReleaseResult::Rejected { reason });
        }

        self.append_event(CurrencyEventPayload::SupplyReleased { amount })?;
        Ok(CurrencySupplyReleaseResult::Released)
    }

    /// Rejects a currency supply release attempt.
    pub fn reject_release_supply(
        &mut self,
        amount: CurrencyAmount,
        reason: CurrencySupplyReleaseRejectionReason,
    ) -> Result<(), CurrencyError> {
        self.append_event(CurrencyEventPayload::SupplyReleaseRejected { amount, reason })?;
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
                    supply: CurrencyAmount::zero(),
                    pending_supply: CurrencyAmount::zero(),
                    provisioning_status: CurrencyProvisioningStatus::Pending,
                    status: CurrencyStatus::Active,
                }));
            }
            CurrencyEventPayload::Provisioned { mint_account } => {
                let state = self.state_required_mut()?;
                state.provisioning_status = CurrencyProvisioningStatus::Provisioned {
                    mint_account: mint_account.clone(),
                };
            }
            CurrencyEventPayload::ProvisionRejected { reason, .. } => match reason {
                CurrencyProvisionRejectionReason::AlreadyProvisioned => {}
                CurrencyProvisionRejectionReason::Removed => {
                    self.state_required_mut()?.provisioning_status =
                        CurrencyProvisioningStatus::Failed;
                }
            },
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
            CurrencyEventPayload::MintAccountMetadataSynced => {}
            CurrencyEventPayload::MintAccountMetadataSyncRejected { .. } => {}
            CurrencyEventPayload::SupplyReserved { amount } => {
                let state = self.state_required_mut()?;
                state.pending_supply =
                    state
                        .pending_supply
                        .try_add(*amount)
                        .map_err(|error| match error {
                            CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                            CurrencyAmountError::InsufficientBalance => {
                                CurrencyError::InsufficientPendingSupply
                            }
                        })?;
            }
            CurrencyEventPayload::SupplyReserveRejected { .. } => {}
            CurrencyEventPayload::MintSupplySynced { .. } => {}
            CurrencyEventPayload::SupplyCommitted { amount } => {
                let state = self.state_required_mut()?;
                state.pending_supply =
                    state
                        .pending_supply
                        .try_sub(*amount)
                        .map_err(|error| match error {
                            CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                            CurrencyAmountError::InsufficientBalance => {
                                CurrencyError::InsufficientPendingSupply
                            }
                        })?;
                state.supply = state.supply.try_add(*amount).map_err(|error| match error {
                    CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                    CurrencyAmountError::InsufficientBalance => CurrencyError::InsufficientSupply,
                })?;
            }
            CurrencyEventPayload::SupplyCommitRejected { .. } => {}
            CurrencyEventPayload::SupplyReleased { amount } => {
                let state = self.state_required_mut()?;
                state.pending_supply =
                    state
                        .pending_supply
                        .try_sub(*amount)
                        .map_err(|error| match error {
                            CurrencyAmountError::BalanceOverflow => CurrencyError::SupplyOverflow,
                            CurrencyAmountError::InsufficientBalance => {
                                CurrencyError::InsufficientPendingSupply
                            }
                        })?;
            }
            CurrencyEventPayload::SupplyReleaseRejected { .. } => {}
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
        CurrencyMintAccountMetadataSyncRejectionReason, CurrencyName, CurrencyOwner,
        CurrencyPoolTokenAccountAddress, CurrencyProvisioningStatus, CurrencyStatus,
        CurrencySymbol,
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
            CurrencyPoolTokenAccountAddress::try_from("Pool111111111111111111111111111111111111")
                .expect("pool account address should be valid"),
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
        assert_eq!(
            currency
                .provisioning_status()
                .expect("provisioning status should exist"),
            CurrencyProvisioningStatus::Pending
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
        assert_eq!(
            currency
                .provisioning_status()
                .expect("provisioning status should exist"),
            CurrencyProvisioningStatus::Pending
        );
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
    fn supply_methods_update_supply_and_pending_supply() {
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
            .provision(make_mint_account(
                "Mint111111111111111111111111111111111111",
            ))
            .expect("provision should succeed");

        currency
            .reserve_supply(CurrencyAmount::new(100))
            .expect("reserve should succeed");
        currency
            .commit_supply(CurrencyAmount::new(60))
            .expect("commit should succeed");
        currency
            .release_supply(CurrencyAmount::new(40))
            .expect("release should succeed");

        assert_eq!(
            currency.supply().expect("supply should exist"),
            &CurrencyAmount::new(60)
        );
        assert_eq!(
            currency
                .pending_supply()
                .expect("pending supply should exist"),
            &CurrencyAmount::zero()
        );
        assert_eq!(currency.uncommitted_events().len(), 5);
    }

    #[test]
    fn record_mint_supply_synced_records_event() {
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
        currency.core_mut().clear_uncommitted_events();

        currency
            .record_mint_supply_synced(CurrencyAmount::new(100))
            .expect("mint supply sync record should succeed");

        assert_eq!(
            currency.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintSupplySynced {
                supply: CurrencyAmount::new(100),
            }
        );
    }

    #[test]
    fn record_mint_account_metadata_synced_records_event() {
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
        currency.core_mut().clear_uncommitted_events();

        currency
            .record_mint_account_metadata_synced()
            .expect("metadata sync record should succeed");

        assert_eq!(
            currency.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountMetadataSynced
        );
    }

    #[test]
    fn reject_mint_account_metadata_sync_records_event() {
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
        currency.core_mut().clear_uncommitted_events();

        currency
            .reject_mint_account_metadata_sync(
                CurrencyMintAccountMetadataSyncRejectionReason::NotProvisioned,
            )
            .expect("metadata sync rejection should succeed");

        assert_eq!(
            currency.uncommitted_events()[0].payload(),
            &CurrencyEventPayload::MintAccountMetadataSyncRejected {
                reason: CurrencyMintAccountMetadataSyncRejectionReason::NotProvisioned,
            }
        );
    }

    #[test]
    fn provision_updates_state_and_records_event() {
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
        currency.core_mut().clear_uncommitted_events();

        let result = currency
            .provision(mint_account.clone())
            .expect("provision should succeed");

        assert!(matches!(
            result,
            super::CurrencyProvisionResult::Provisioned {
                mint_account: recorded
            } if recorded == mint_account
        ));
        assert_eq!(
            currency.mint_account().expect("mint account should exist"),
            Some(&mint_account)
        );
        assert_eq!(
            currency
                .provisioning_status()
                .expect("provisioning status should exist"),
            CurrencyProvisioningStatus::Provisioned {
                mint_account: mint_account.clone(),
            }
        );
        assert_eq!(
            currency.uncommitted_events()[0].payload().name(),
            CurrencyEventPayload::PROVISIONED
        );
    }

    #[test]
    fn provision_rejects_duplicate_with_event() {
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
            .provision(mint_account.clone())
            .expect("provision should succeed");

        let result = currency
            .provision(duplicate_mint_account)
            .expect("duplicate provision should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencyProvisionResult::Rejected {
                reason: super::CurrencyProvisionRejectionReason::AlreadyProvisioned
            }
        ));
        assert_eq!(
            currency.mint_account().expect("mint account should exist"),
            Some(&mint_account)
        );
        assert_eq!(
            currency.uncommitted_events()[2].payload().name(),
            CurrencyEventPayload::PROVISION_REJECTED
        );
    }

    #[test]
    fn provision_rejects_removed_currency_with_event() {
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
            .provision(make_mint_account(
                "Mint111111111111111111111111111111111111",
            ))
            .expect("removed provision should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencyProvisionResult::Rejected {
                reason: super::CurrencyProvisionRejectionReason::Removed
            }
        ));
        assert_eq!(
            currency
                .provisioning_status()
                .expect("provisioning status should exist"),
            CurrencyProvisioningStatus::Failed
        );
        assert_eq!(
            currency.mint_account().expect("mint account should exist"),
            None
        );
        assert_eq!(
            currency.uncommitted_events()[2].payload().name(),
            CurrencyEventPayload::PROVISION_REJECTED
        );
    }

    #[test]
    fn reserve_supply_rejects_unprovisioned_currency() {
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
            .reserve_supply(CurrencyAmount::new(1))
            .expect("reserve should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencySupplyReserveResult::Rejected {
                reason: super::CurrencySupplyReserveRejectionReason::ProvisioningPending
            }
        ));
        assert_eq!(
            currency.uncommitted_events()[1].payload().name(),
            CurrencyEventPayload::SUPPLY_RESERVE_REJECTED
        );
    }

    #[test]
    fn reserve_supply_rejects_inactive_currency() {
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
            .provision(make_mint_account(
                "Mint111111111111111111111111111111111111",
            ))
            .expect("provision should succeed");
        currency.deactivate().expect("deactivate should succeed");

        let result = currency
            .reserve_supply(CurrencyAmount::new(1))
            .expect("reserve should complete with a rejection event");

        assert!(matches!(
            result,
            super::CurrencySupplyReserveResult::Rejected {
                reason: super::CurrencySupplyReserveRejectionReason::Inactive
            }
        ));
        assert_eq!(
            currency.uncommitted_events()[3].payload().name(),
            CurrencyEventPayload::SUPPLY_RESERVE_REJECTED
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
