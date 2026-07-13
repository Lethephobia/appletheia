mod wallet_bookmark_description;
mod wallet_bookmark_description_change_rejection_reason;
mod wallet_bookmark_description_change_result;
mod wallet_bookmark_description_error;
mod wallet_bookmark_display_name;
mod wallet_bookmark_display_name_change_rejection_reason;
mod wallet_bookmark_display_name_change_result;
mod wallet_bookmark_display_name_error;
mod wallet_bookmark_error;
mod wallet_bookmark_event_payload;
mod wallet_bookmark_event_payload_error;
mod wallet_bookmark_id;
mod wallet_bookmark_owner;
mod wallet_bookmark_register_result;
mod wallet_bookmark_registration;
mod wallet_bookmark_remove_rejection_reason;
mod wallet_bookmark_remove_result;
mod wallet_bookmark_state;
mod wallet_bookmark_state_error;
mod wallet_bookmark_status;

pub use wallet_bookmark_description::WalletBookmarkDescription;
pub use wallet_bookmark_description_change_rejection_reason::WalletBookmarkDescriptionChangeRejectionReason;
pub use wallet_bookmark_description_change_result::WalletBookmarkDescriptionChangeResult;
pub use wallet_bookmark_description_error::WalletBookmarkDescriptionError;
pub use wallet_bookmark_display_name::WalletBookmarkDisplayName;
pub use wallet_bookmark_display_name_change_rejection_reason::WalletBookmarkDisplayNameChangeRejectionReason;
pub use wallet_bookmark_display_name_change_result::WalletBookmarkDisplayNameChangeResult;
pub use wallet_bookmark_display_name_error::WalletBookmarkDisplayNameError;
pub use wallet_bookmark_error::WalletBookmarkError;
pub use wallet_bookmark_event_payload::WalletBookmarkEventPayload;
pub use wallet_bookmark_event_payload_error::WalletBookmarkEventPayloadError;
pub use wallet_bookmark_id::WalletBookmarkId;
pub use wallet_bookmark_owner::WalletBookmarkOwner;
pub use wallet_bookmark_register_result::WalletBookmarkRegisterResult;
pub use wallet_bookmark_registration::WalletBookmarkRegistration;
pub use wallet_bookmark_remove_rejection_reason::WalletBookmarkRemoveRejectionReason;
pub use wallet_bookmark_remove_result::WalletBookmarkRemoveResult;
pub use wallet_bookmark_state::WalletBookmarkState;
pub use wallet_bookmark_state_error::WalletBookmarkStateError;
pub use wallet_bookmark_status::WalletBookmarkStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::TokenAccountOwnerAddress;

/// Represents the `WalletBookmark` aggregate root.
#[aggregate(type = "wallet_bookmark", error = WalletBookmarkError)]
pub struct WalletBookmark {
    core: AggregateCore<WalletBookmarkState, WalletBookmarkEventPayload>,
}

impl WalletBookmark {
    /// Returns the owner that registered this wallet bookmark.
    pub fn owner(&self) -> Result<&WalletBookmarkOwner, WalletBookmarkError> {
        Ok(&self.state_required()?.owner)
    }

    /// Returns the user-facing display name.
    pub fn display_name(&self) -> Result<Option<&WalletBookmarkDisplayName>, WalletBookmarkError> {
        Ok(self.state_required()?.display_name.as_ref())
    }

    /// Returns the user-facing description.
    pub fn description(&self) -> Result<Option<&WalletBookmarkDescription>, WalletBookmarkError> {
        Ok(self.state_required()?.description.as_ref())
    }

    /// Returns the wallet bookmark token account owner address.
    pub fn token_account_owner_address(
        &self,
    ) -> Result<&TokenAccountOwnerAddress, WalletBookmarkError> {
        Ok(&self.state_required()?.token_account_owner_address)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<&WalletBookmarkStatus, WalletBookmarkError> {
        Ok(&self.state_required()?.status)
    }

    /// Changes the user-facing display name.
    pub fn change_display_name(
        &mut self,
        display_name: Option<WalletBookmarkDisplayName>,
    ) -> Result<WalletBookmarkDisplayNameChangeResult, WalletBookmarkError> {
        match self.state_required()?.status {
            WalletBookmarkStatus::Active => {}
            WalletBookmarkStatus::Removed => {
                let reason = WalletBookmarkDisplayNameChangeRejectionReason::Removed;
                self.reject_change_display_name(display_name, reason)?;
                return Ok(WalletBookmarkDisplayNameChangeResult::Rejected { reason });
            }
        }

        self.append_event(WalletBookmarkEventPayload::DisplayNameChanged { display_name })?;
        Ok(WalletBookmarkDisplayNameChangeResult::Changed)
    }

    /// Rejects a wallet bookmark display name change attempt.
    pub fn reject_change_display_name(
        &mut self,
        display_name: Option<WalletBookmarkDisplayName>,
        reason: WalletBookmarkDisplayNameChangeRejectionReason,
    ) -> Result<(), WalletBookmarkError> {
        self.append_event(WalletBookmarkEventPayload::DisplayNameChangeRejected {
            display_name,
            reason,
        })?;
        Ok(())
    }

    /// Changes the user-facing description.
    pub fn change_description(
        &mut self,
        description: Option<WalletBookmarkDescription>,
    ) -> Result<WalletBookmarkDescriptionChangeResult, WalletBookmarkError> {
        match self.state_required()?.status {
            WalletBookmarkStatus::Active => {}
            WalletBookmarkStatus::Removed => {
                let reason = WalletBookmarkDescriptionChangeRejectionReason::Removed;
                self.reject_change_description(description, reason)?;
                return Ok(WalletBookmarkDescriptionChangeResult::Rejected { reason });
            }
        }

        self.append_event(WalletBookmarkEventPayload::DescriptionChanged { description })?;
        Ok(WalletBookmarkDescriptionChangeResult::Changed)
    }

    /// Rejects a wallet bookmark description change attempt.
    pub fn reject_change_description(
        &mut self,
        description: Option<WalletBookmarkDescription>,
        reason: WalletBookmarkDescriptionChangeRejectionReason,
    ) -> Result<(), WalletBookmarkError> {
        self.append_event(WalletBookmarkEventPayload::DescriptionChangeRejected {
            description,
            reason,
        })?;
        Ok(())
    }

    /// Registers a wallet bookmark.
    pub fn register(
        &mut self,
        registration: WalletBookmarkRegistration,
    ) -> Result<WalletBookmarkRegisterResult, WalletBookmarkError> {
        if self.state().is_some() {
            return Err(WalletBookmarkError::AlreadyRegistered);
        }

        let wallet_bookmark_id = WalletBookmarkId::new();
        let (owner, display_name, description, token_account_owner_address) =
            registration.into_parts();
        self.append_event(WalletBookmarkEventPayload::Registered {
            id: wallet_bookmark_id,
            owner,
            display_name,
            description,
            token_account_owner_address,
        })?;

        Ok(WalletBookmarkRegisterResult::Registered { wallet_bookmark_id })
    }

    /// Removes a wallet bookmark.
    pub fn remove(&mut self) -> Result<WalletBookmarkRemoveResult, WalletBookmarkError> {
        match self.state_required()?.status {
            WalletBookmarkStatus::Active => {}
            WalletBookmarkStatus::Removed => {
                let reason = WalletBookmarkRemoveRejectionReason::AlreadyRemoved;
                self.reject_remove(reason)?;
                return Ok(WalletBookmarkRemoveResult::Rejected { reason });
            }
        }

        self.append_event(WalletBookmarkEventPayload::Removed)?;
        Ok(WalletBookmarkRemoveResult::Removed)
    }

    /// Rejects removing a wallet bookmark.
    pub fn reject_remove(
        &mut self,
        reason: WalletBookmarkRemoveRejectionReason,
    ) -> Result<(), WalletBookmarkError> {
        self.append_event(WalletBookmarkEventPayload::RemoveRejected { reason })?;
        Ok(())
    }
}

impl AggregateApply<WalletBookmarkEventPayload, WalletBookmarkError> for WalletBookmark {
    fn apply(&mut self, payload: &WalletBookmarkEventPayload) -> Result<(), WalletBookmarkError> {
        match payload {
            WalletBookmarkEventPayload::Registered {
                id,
                owner,
                display_name,
                description,
                token_account_owner_address,
            } => self.set_state(Some(WalletBookmarkState {
                id: *id,
                owner: *owner,
                display_name: display_name.clone(),
                description: description.clone(),
                token_account_owner_address: token_account_owner_address.clone(),
                status: WalletBookmarkStatus::Active,
            })),
            WalletBookmarkEventPayload::Removed => {
                self.state_required_mut()?.status = WalletBookmarkStatus::Removed;
            }
            WalletBookmarkEventPayload::RemoveRejected { .. } => {}
            WalletBookmarkEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?.display_name = display_name.clone();
            }
            WalletBookmarkEventPayload::DisplayNameChangeRejected { .. } => {}
            WalletBookmarkEventPayload::DescriptionChanged { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            WalletBookmarkEventPayload::DescriptionChangeRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};
    use banking_iam_domain::UserId;

    use crate::core::TokenAccountOwnerAddress;

    use super::{
        WalletBookmark, WalletBookmarkDescription, WalletBookmarkDisplayName,
        WalletBookmarkEventPayload, WalletBookmarkOwner, WalletBookmarkRegistration,
        WalletBookmarkRemoveRejectionReason, WalletBookmarkRemoveResult, WalletBookmarkStatus,
    };

    fn token_account_owner_address() -> TokenAccountOwnerAddress {
        TokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("address should be valid")
    }

    fn wallet_bookmark_owner() -> WalletBookmarkOwner {
        WalletBookmarkOwner::User(UserId::new())
    }

    fn wallet_bookmark_display_name() -> WalletBookmarkDisplayName {
        WalletBookmarkDisplayName::try_from("Main wallet").expect("display name should be valid")
    }

    fn wallet_bookmark_description() -> WalletBookmarkDescription {
        WalletBookmarkDescription::try_from("Personal main wallet")
            .expect("description should be valid")
    }

    #[test]
    fn register_initializes_state_and_records_event() {
        let owner = wallet_bookmark_owner();
        let display_name = wallet_bookmark_display_name();
        let description = wallet_bookmark_description();
        let token_account_owner_address = token_account_owner_address();
        let mut wallet_bookmark = WalletBookmark::new();

        wallet_bookmark
            .register(WalletBookmarkRegistration {
                owner,
                display_name: Some(display_name.clone()),
                description: Some(description.clone()),
                token_account_owner_address: token_account_owner_address.clone(),
            })
            .expect("register should succeed");

        assert_eq!(wallet_bookmark.owner().expect("owner should exist"), &owner);
        assert_eq!(
            wallet_bookmark
                .display_name()
                .expect("display name lookup should succeed"),
            Some(&display_name)
        );
        assert_eq!(
            wallet_bookmark
                .description()
                .expect("description lookup should succeed"),
            Some(&description)
        );
        assert_eq!(
            wallet_bookmark
                .token_account_owner_address()
                .expect("address should exist"),
            &token_account_owner_address
        );
        assert_eq!(
            wallet_bookmark.status().expect("status should exist"),
            &WalletBookmarkStatus::Active
        );

        let events = wallet_bookmark.uncommitted_events();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].payload().name(),
            WalletBookmarkEventPayload::REGISTERED
        );
    }

    #[test]
    fn remove_marks_bookmark_removed() {
        let owner = wallet_bookmark_owner();
        let display_name = wallet_bookmark_display_name();
        let description = wallet_bookmark_description();
        let token_account_owner_address = token_account_owner_address();
        let mut wallet_bookmark = WalletBookmark::new();
        wallet_bookmark
            .register(WalletBookmarkRegistration {
                owner,
                display_name: Some(display_name),
                description: Some(description),
                token_account_owner_address,
            })
            .expect("register should succeed");
        wallet_bookmark.core_mut().clear_uncommitted_events();

        let result = wallet_bookmark.remove().expect("remove should succeed");

        assert_eq!(result, WalletBookmarkRemoveResult::Removed);
        assert_eq!(
            wallet_bookmark.status().expect("status should exist"),
            &WalletBookmarkStatus::Removed
        );
        let events = wallet_bookmark.uncommitted_events();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].payload().name(),
            WalletBookmarkEventPayload::REMOVED
        );
    }

    #[test]
    fn remove_rejects_when_already_removed() {
        let owner = wallet_bookmark_owner();
        let display_name = wallet_bookmark_display_name();
        let description = wallet_bookmark_description();
        let token_account_owner_address = token_account_owner_address();
        let mut wallet_bookmark = WalletBookmark::new();
        wallet_bookmark
            .register(WalletBookmarkRegistration {
                owner,
                display_name: Some(display_name),
                description: Some(description),
                token_account_owner_address,
            })
            .expect("register should succeed");
        wallet_bookmark.core_mut().clear_uncommitted_events();
        wallet_bookmark
            .remove()
            .expect("first remove should succeed");
        wallet_bookmark.core_mut().clear_uncommitted_events();

        let result = wallet_bookmark
            .remove()
            .expect("second remove should succeed");

        assert_eq!(
            result,
            WalletBookmarkRemoveResult::Rejected {
                reason: WalletBookmarkRemoveRejectionReason::AlreadyRemoved,
            }
        );
        let events = wallet_bookmark.uncommitted_events();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].payload().name(),
            WalletBookmarkEventPayload::REMOVE_REJECTED
        );
    }
}
