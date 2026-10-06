mod wallet_bookmark_description;
mod wallet_bookmark_description_error;
mod wallet_bookmark_display_name;
mod wallet_bookmark_display_name_error;
mod wallet_bookmark_error;
mod wallet_bookmark_event_payload;
mod wallet_bookmark_event_payload_error;
mod wallet_bookmark_id;
mod wallet_bookmark_owner;
mod wallet_bookmark_state;
mod wallet_bookmark_state_error;
mod wallet_bookmark_status;

pub use wallet_bookmark_description::WalletBookmarkDescription;
pub use wallet_bookmark_description_error::WalletBookmarkDescriptionError;
pub use wallet_bookmark_display_name::WalletBookmarkDisplayName;
pub use wallet_bookmark_display_name_error::WalletBookmarkDisplayNameError;
pub use wallet_bookmark_error::WalletBookmarkError;
pub use wallet_bookmark_event_payload::WalletBookmarkEventPayload;
pub use wallet_bookmark_event_payload_error::WalletBookmarkEventPayloadError;
pub use wallet_bookmark_id::WalletBookmarkId;
pub use wallet_bookmark_owner::WalletBookmarkOwner;
pub use wallet_bookmark_state::WalletBookmarkState;
pub use wallet_bookmark_state_error::WalletBookmarkStateError;
pub use wallet_bookmark_status::WalletBookmarkStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::TokenOwnerAddress;

/// Represents the `WalletBookmark` aggregate root.
#[aggregate(type = "wallet_bookmark", error = WalletBookmarkError)]
pub struct WalletBookmark {
    core: AggregateCore<WalletBookmarkId, WalletBookmarkState, WalletBookmarkEventPayload>,
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
    pub fn token_owner_address(&self) -> Result<&TokenOwnerAddress, WalletBookmarkError> {
        Ok(&self.state_required()?.token_owner_address)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<&WalletBookmarkStatus, WalletBookmarkError> {
        Ok(&self.state_required()?.status)
    }

    /// Registers a wallet bookmark.
    pub fn register(
        &mut self,
        owner: WalletBookmarkOwner,
        token_owner_address: TokenOwnerAddress,
    ) -> Result<(), WalletBookmarkError> {
        if self.state().is_some() {
            return Err(WalletBookmarkError::AlreadyRegistered);
        }

        self.append_event(WalletBookmarkEventPayload::Registered {
            owner,
            token_owner_address,
        })?;

        Ok(())
    }

    /// Sets the user-facing display name.
    pub fn set_display_name(
        &mut self,
        display_name: Option<WalletBookmarkDisplayName>,
    ) -> Result<(), WalletBookmarkError> {
        match self.state_required()?.status {
            WalletBookmarkStatus::Active => {}
            WalletBookmarkStatus::Removed => {
                return Err(WalletBookmarkError::Removed);
            }
        }

        self.append_event(WalletBookmarkEventPayload::DisplayNameSet { display_name })?;
        Ok(())
    }

    /// Sets the user-facing description.
    pub fn set_description(
        &mut self,
        description: Option<WalletBookmarkDescription>,
    ) -> Result<(), WalletBookmarkError> {
        match self.state_required()?.status {
            WalletBookmarkStatus::Active => {}
            WalletBookmarkStatus::Removed => {
                return Err(WalletBookmarkError::Removed);
            }
        }

        self.append_event(WalletBookmarkEventPayload::DescriptionSet { description })?;
        Ok(())
    }

    /// Removes a wallet bookmark.
    pub fn remove(&mut self) -> Result<(), WalletBookmarkError> {
        match self.state_required()?.status {
            WalletBookmarkStatus::Active => {}
            WalletBookmarkStatus::Removed => {
                return Err(WalletBookmarkError::Removed);
            }
        }

        self.append_event(WalletBookmarkEventPayload::Removed)?;
        Ok(())
    }
}

impl AggregateApply<WalletBookmarkEventPayload, WalletBookmarkError> for WalletBookmark {
    fn apply(&mut self, payload: &WalletBookmarkEventPayload) -> Result<(), WalletBookmarkError> {
        match payload {
            WalletBookmarkEventPayload::Registered {
                owner,
                token_owner_address,
            } => self.set_state(Some(WalletBookmarkState {
                owner: *owner,
                display_name: None,
                description: None,
                token_owner_address: *token_owner_address,
                status: WalletBookmarkStatus::Active,
            })),
            WalletBookmarkEventPayload::DisplayNameSet { display_name } => {
                self.state_required_mut()?.display_name = display_name.clone();
            }
            WalletBookmarkEventPayload::DescriptionSet { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            WalletBookmarkEventPayload::Removed => {
                self.state_required_mut()?.status = WalletBookmarkStatus::Removed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateApply, EventPayload};
    use banking_iam_domain::UserId;

    use crate::core::{SolanaAccountAddress, SolanaTokenAccountOwnerAddress, TokenOwnerAddress};

    use super::{
        WalletBookmark, WalletBookmarkDescription, WalletBookmarkDisplayName,
        WalletBookmarkEventPayload, WalletBookmarkOwner, WalletBookmarkStatus,
    };

    fn token_owner_address() -> TokenOwnerAddress {
        let address = SolanaAccountAddress::try_from("11111111111111111111111111111111")
            .expect("address should be valid");
        TokenOwnerAddress::Solana(SolanaTokenAccountOwnerAddress::new(address))
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
        let token_owner_address = token_owner_address();
        let mut wallet_bookmark = WalletBookmark::new();

        wallet_bookmark
            .register(owner, token_owner_address)
            .expect("register should succeed");

        assert_eq!(wallet_bookmark.owner().expect("owner should exist"), &owner);
        assert_eq!(
            wallet_bookmark
                .display_name()
                .expect("display name lookup should succeed"),
            None
        );
        assert_eq!(
            wallet_bookmark
                .description()
                .expect("description lookup should succeed"),
            None
        );
        assert_eq!(
            wallet_bookmark
                .token_owner_address()
                .expect("address should exist"),
            &token_owner_address
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
        let token_owner_address = token_owner_address();
        let mut wallet_bookmark = WalletBookmark::new();
        wallet_bookmark
            .register(owner, token_owner_address)
            .expect("register should succeed");
        wallet_bookmark.core_mut().clear_uncommitted_events();

        wallet_bookmark.remove().expect("remove should succeed");

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
        let token_owner_address = token_owner_address();
        let mut wallet_bookmark = WalletBookmark::new();
        wallet_bookmark
            .register(owner, token_owner_address)
            .expect("register should succeed");
        wallet_bookmark.core_mut().clear_uncommitted_events();
        wallet_bookmark
            .remove()
            .expect("first remove should succeed");
        wallet_bookmark.core_mut().clear_uncommitted_events();

        wallet_bookmark
            .remove()
            .expect_err("second remove should fail");
        let events = wallet_bookmark.uncommitted_events();
        assert!(events.is_empty());
    }

    #[test]
    fn optional_fields_are_set_separately_and_replayable() {
        let mut wallet_bookmark = WalletBookmark::new();
        wallet_bookmark
            .register(wallet_bookmark_owner(), token_owner_address())
            .unwrap();
        assert_eq!(wallet_bookmark.uncommitted_events().len(), 1);
        assert_eq!(wallet_bookmark.display_name().unwrap(), None);
        let value = wallet_bookmark_display_name();
        wallet_bookmark
            .set_display_name(Some(value.clone()))
            .unwrap();
        assert_eq!(wallet_bookmark.display_name().unwrap(), Some(&value));
        wallet_bookmark.set_display_name(None).unwrap();
        wallet_bookmark.set_display_name(None).unwrap();
        assert_eq!(wallet_bookmark.display_name().unwrap(), None);
        assert_eq!(wallet_bookmark.description().unwrap(), None);
        let value = wallet_bookmark_description();
        wallet_bookmark
            .set_description(Some(value.clone()))
            .unwrap();
        assert_eq!(wallet_bookmark.description().unwrap(), Some(&value));
        wallet_bookmark.set_description(None).unwrap();
        wallet_bookmark.set_description(None).unwrap();
        assert_eq!(wallet_bookmark.description().unwrap(), None);
        assert_eq!(wallet_bookmark.uncommitted_events().len(), 7);
        let mut replayed = WalletBookmark::new();
        for event in wallet_bookmark.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), wallet_bookmark.state());
    }
}
