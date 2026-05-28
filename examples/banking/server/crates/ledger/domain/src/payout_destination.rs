mod payout_destination_error;
mod payout_destination_event_payload;
mod payout_destination_event_payload_error;
mod payout_destination_id;
mod payout_destination_owner;
mod payout_destination_register_result;
mod payout_destination_registration;
mod payout_destination_remove_rejection_reason;
mod payout_destination_remove_result;
mod payout_destination_state;
mod payout_destination_state_error;
mod payout_destination_status;
mod payout_destination_token_account_owner_address;
mod payout_destination_token_account_owner_address_error;

pub use payout_destination_error::PayoutDestinationError;
pub use payout_destination_event_payload::PayoutDestinationEventPayload;
pub use payout_destination_event_payload_error::PayoutDestinationEventPayloadError;
pub use payout_destination_id::PayoutDestinationId;
pub use payout_destination_owner::PayoutDestinationOwner;
pub use payout_destination_register_result::PayoutDestinationRegisterResult;
pub use payout_destination_registration::PayoutDestinationRegistration;
pub use payout_destination_remove_rejection_reason::PayoutDestinationRemoveRejectionReason;
pub use payout_destination_remove_result::PayoutDestinationRemoveResult;
pub use payout_destination_state::PayoutDestinationState;
pub use payout_destination_state_error::PayoutDestinationStateError;
pub use payout_destination_status::PayoutDestinationStatus;
pub use payout_destination_token_account_owner_address::PayoutDestinationTokenAccountOwnerAddress;
pub use payout_destination_token_account_owner_address_error::PayoutDestinationTokenAccountOwnerAddressError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

/// Represents the `PayoutDestination` aggregate root.
#[aggregate(type = "payout_destination", error = PayoutDestinationError)]
pub struct PayoutDestination {
    core: AggregateCore<PayoutDestinationState, PayoutDestinationEventPayload>,
}

impl PayoutDestination {
    /// Returns the owner that registered this payout destination.
    pub fn owner(&self) -> Result<&PayoutDestinationOwner, PayoutDestinationError> {
        Ok(&self.state_required()?.owner)
    }

    /// Returns the payout destination token account owner address.
    pub fn token_account_owner_address(
        &self,
    ) -> Result<&PayoutDestinationTokenAccountOwnerAddress, PayoutDestinationError> {
        Ok(&self.state_required()?.token_account_owner_address)
    }

    /// Returns the current status.
    pub fn status(&self) -> Result<&PayoutDestinationStatus, PayoutDestinationError> {
        Ok(&self.state_required()?.status)
    }

    /// Registers a payout destination.
    pub fn register(
        &mut self,
        registration: PayoutDestinationRegistration,
    ) -> Result<PayoutDestinationRegisterResult, PayoutDestinationError> {
        if self.state().is_some() {
            return Err(PayoutDestinationError::AlreadyRegistered);
        }

        let payout_destination_id = PayoutDestinationId::new();
        let (owner, token_account_owner_address) = registration.into_parts();
        self.append_event(PayoutDestinationEventPayload::Registered {
            id: payout_destination_id,
            owner,
            token_account_owner_address,
        })?;

        Ok(PayoutDestinationRegisterResult::Registered {
            payout_destination_id,
        })
    }

    /// Removes a payout destination.
    pub fn remove(&mut self) -> Result<PayoutDestinationRemoveResult, PayoutDestinationError> {
        match self.state_required()?.status {
            PayoutDestinationStatus::Active => {}
            PayoutDestinationStatus::Removed => {
                let reason = PayoutDestinationRemoveRejectionReason::AlreadyRemoved;
                self.reject_remove(reason)?;
                return Ok(PayoutDestinationRemoveResult::Rejected { reason });
            }
        }

        self.append_event(PayoutDestinationEventPayload::Removed)?;
        Ok(PayoutDestinationRemoveResult::Removed)
    }

    /// Rejects removing a payout destination.
    pub fn reject_remove(
        &mut self,
        reason: PayoutDestinationRemoveRejectionReason,
    ) -> Result<(), PayoutDestinationError> {
        self.append_event(PayoutDestinationEventPayload::RemoveRejected { reason })?;
        Ok(())
    }
}

impl AggregateApply<PayoutDestinationEventPayload, PayoutDestinationError> for PayoutDestination {
    fn apply(
        &mut self,
        payload: &PayoutDestinationEventPayload,
    ) -> Result<(), PayoutDestinationError> {
        match payload {
            PayoutDestinationEventPayload::Registered {
                id,
                owner,
                token_account_owner_address,
            } => self.set_state(Some(PayoutDestinationState {
                id: *id,
                owner: *owner,
                token_account_owner_address: token_account_owner_address.clone(),
                status: PayoutDestinationStatus::Active,
            })),
            PayoutDestinationEventPayload::Removed => {
                self.state_required_mut()?.status = PayoutDestinationStatus::Removed;
            }
            PayoutDestinationEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};
    use banking_iam_domain::UserId;

    use super::{
        PayoutDestination, PayoutDestinationEventPayload, PayoutDestinationOwner,
        PayoutDestinationRegistration, PayoutDestinationRemoveRejectionReason,
        PayoutDestinationRemoveResult, PayoutDestinationStatus,
        PayoutDestinationTokenAccountOwnerAddress,
    };

    fn payout_destination_token_account_owner_address() -> PayoutDestinationTokenAccountOwnerAddress
    {
        PayoutDestinationTokenAccountOwnerAddress::try_from("11111111111111111111111111111111")
            .expect("address should be valid")
    }

    fn payout_destination_owner() -> PayoutDestinationOwner {
        PayoutDestinationOwner::User(UserId::new())
    }

    #[test]
    fn register_initializes_state_and_records_event() {
        let owner = payout_destination_owner();
        let token_account_owner_address = payout_destination_token_account_owner_address();
        let mut payout_destination = PayoutDestination::default();

        payout_destination
            .register(PayoutDestinationRegistration {
                owner,
                token_account_owner_address: token_account_owner_address.clone(),
            })
            .expect("register should succeed");

        assert_eq!(
            payout_destination.owner().expect("owner should exist"),
            &owner
        );
        assert_eq!(
            payout_destination
                .token_account_owner_address()
                .expect("address should exist"),
            &token_account_owner_address
        );
        assert_eq!(
            payout_destination.status().expect("status should exist"),
            &PayoutDestinationStatus::Active
        );

        let events = payout_destination.uncommitted_events();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].payload().name(),
            PayoutDestinationEventPayload::REGISTERED
        );
    }

    #[test]
    fn remove_marks_destination_removed() {
        let owner = payout_destination_owner();
        let token_account_owner_address = payout_destination_token_account_owner_address();
        let mut payout_destination = PayoutDestination::default();
        payout_destination
            .register(PayoutDestinationRegistration {
                owner,
                token_account_owner_address,
            })
            .expect("register should succeed");
        payout_destination.core_mut().clear_uncommitted_events();

        let result = payout_destination.remove().expect("remove should succeed");

        assert_eq!(result, PayoutDestinationRemoveResult::Removed);
        assert_eq!(
            payout_destination.status().expect("status should exist"),
            &PayoutDestinationStatus::Removed
        );
        let events = payout_destination.uncommitted_events();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].payload().name(),
            PayoutDestinationEventPayload::REMOVED
        );
    }

    #[test]
    fn remove_rejects_when_already_removed() {
        let owner = payout_destination_owner();
        let token_account_owner_address = payout_destination_token_account_owner_address();
        let mut payout_destination = PayoutDestination::default();
        payout_destination
            .register(PayoutDestinationRegistration {
                owner,
                token_account_owner_address,
            })
            .expect("register should succeed");
        payout_destination.core_mut().clear_uncommitted_events();
        payout_destination
            .remove()
            .expect("first remove should succeed");
        payout_destination.core_mut().clear_uncommitted_events();

        let result = payout_destination
            .remove()
            .expect("second remove should succeed");

        assert_eq!(
            result,
            PayoutDestinationRemoveResult::Rejected {
                reason: PayoutDestinationRemoveRejectionReason::AlreadyRemoved,
            }
        );
        let events = payout_destination.uncommitted_events();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].payload().name(),
            PayoutDestinationEventPayload::REMOVE_REJECTED
        );
    }
}
