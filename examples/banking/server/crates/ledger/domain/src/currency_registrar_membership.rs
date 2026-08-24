mod currency_registrar_membership_create_rejection_reason;
mod currency_registrar_membership_create_result;
mod currency_registrar_membership_error;
mod currency_registrar_membership_event_payload;
mod currency_registrar_membership_event_payload_error;
mod currency_registrar_membership_id;
mod currency_registrar_membership_remove_rejection_reason;
mod currency_registrar_membership_remove_result;
mod currency_registrar_membership_state;
mod currency_registrar_membership_state_error;
mod currency_registrar_membership_status;

pub use currency_registrar_membership_create_rejection_reason::CurrencyRegistrarMembershipCreateRejectionReason;
pub use currency_registrar_membership_create_result::CurrencyRegistrarMembershipCreateResult;
pub use currency_registrar_membership_error::CurrencyRegistrarMembershipError;
pub use currency_registrar_membership_event_payload::CurrencyRegistrarMembershipEventPayload;
pub use currency_registrar_membership_event_payload_error::CurrencyRegistrarMembershipEventPayloadError;
pub use currency_registrar_membership_id::CurrencyRegistrarMembershipId;
pub use currency_registrar_membership_remove_rejection_reason::CurrencyRegistrarMembershipRemoveRejectionReason;
pub use currency_registrar_membership_remove_result::CurrencyRegistrarMembershipRemoveResult;
pub use currency_registrar_membership_state::CurrencyRegistrarMembershipState;
pub use currency_registrar_membership_state_error::CurrencyRegistrarMembershipStateError;
pub use currency_registrar_membership_status::CurrencyRegistrarMembershipStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};
use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

/// Represents one user's membership in a CurrencyRegistrar.
#[aggregate(type = "currency_registrar_membership", error = CurrencyRegistrarMembershipError)]
pub struct CurrencyRegistrarMembership {
    core: AggregateCore<
        CurrencyRegistrarMembershipId,
        CurrencyRegistrarMembershipState,
        CurrencyRegistrarMembershipEventPayload,
    >,
}

impl CurrencyRegistrarMembership {
    /// Returns the registrar this membership belongs to.
    pub fn currency_registrar_id(
        &self,
    ) -> Result<&CurrencyRegistrarId, CurrencyRegistrarMembershipError> {
        Ok(&self.state_required()?.currency_registrar_id)
    }

    /// Returns the member user.
    pub fn user_id(&self) -> Result<&UserId, CurrencyRegistrarMembershipError> {
        Ok(&self.state_required()?.user_id)
    }

    /// Returns the current membership status.
    pub fn status(
        &self,
    ) -> Result<CurrencyRegistrarMembershipStatus, CurrencyRegistrarMembershipError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the membership is currently active.
    pub fn is_active(&self) -> Result<bool, CurrencyRegistrarMembershipError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Creates a membership without a separate role assignment.
    pub fn create(
        &mut self,
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    ) -> Result<CurrencyRegistrarMembershipCreateResult, CurrencyRegistrarMembershipError> {
        if self.state().is_some() {
            return Err(CurrencyRegistrarMembershipError::AlreadyCreated);
        }

        self.append_event(CurrencyRegistrarMembershipEventPayload::Created {
            currency_registrar_id,
            user_id,
        })?;
        Ok(CurrencyRegistrarMembershipCreateResult::Created)
    }

    pub fn reject_create(
        &mut self,
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
        reason: CurrencyRegistrarMembershipCreateRejectionReason,
    ) -> Result<(), CurrencyRegistrarMembershipError> {
        if self.state().is_some() {
            return Err(CurrencyRegistrarMembershipError::AlreadyCreated);
        }
        self.append_event(CurrencyRegistrarMembershipEventPayload::CreateRejected {
            currency_registrar_id,
            user_id,
            reason,
        })?;
        Ok(())
    }

    /// Removes the membership and terminates this aggregate lifecycle.
    pub fn remove(
        &mut self,
    ) -> Result<CurrencyRegistrarMembershipRemoveResult, CurrencyRegistrarMembershipError> {
        if !self.state_required()?.status.is_active() {
            let reason = CurrencyRegistrarMembershipRemoveRejectionReason::AlreadyRemoved;
            self.reject_remove(reason)?;
            return Ok(CurrencyRegistrarMembershipRemoveResult::Rejected { reason });
        }

        let (currency_registrar_id, user_id) = {
            let state = self.state_required()?;
            (state.currency_registrar_id, state.user_id)
        };
        self.append_event(CurrencyRegistrarMembershipEventPayload::Removed {
            currency_registrar_id,
            user_id,
        })?;
        Ok(CurrencyRegistrarMembershipRemoveResult::Removed)
    }

    /// Records a rejected membership removal.
    pub fn reject_remove(
        &mut self,
        reason: CurrencyRegistrarMembershipRemoveRejectionReason,
    ) -> Result<(), CurrencyRegistrarMembershipError> {
        let (currency_registrar_id, user_id) = {
            let state = self.state_required()?;
            (state.currency_registrar_id, state.user_id)
        };
        self.append_event(CurrencyRegistrarMembershipEventPayload::RemoveRejected {
            currency_registrar_id,
            user_id,
            reason,
        })?;
        Ok(())
    }
}

impl AggregateApply<CurrencyRegistrarMembershipEventPayload, CurrencyRegistrarMembershipError>
    for CurrencyRegistrarMembership
{
    fn apply(
        &mut self,
        payload: &CurrencyRegistrarMembershipEventPayload,
    ) -> Result<(), CurrencyRegistrarMembershipError> {
        match payload {
            CurrencyRegistrarMembershipEventPayload::Created {
                currency_registrar_id,
                user_id,
            } => {
                self.set_state(Some(CurrencyRegistrarMembershipState {
                    currency_registrar_id: *currency_registrar_id,
                    user_id: *user_id,
                    status: CurrencyRegistrarMembershipStatus::Active,
                }));
            }
            CurrencyRegistrarMembershipEventPayload::CreateRejected { .. } => {}
            CurrencyRegistrarMembershipEventPayload::Removed { .. } => {
                self.state_required_mut()?.status = CurrencyRegistrarMembershipStatus::Removed;
            }
            CurrencyRegistrarMembershipEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId, UniqueConstraints, UniqueValues};
    use banking_iam_domain::UserId;

    use super::{
        CurrencyRegistrarMembership, CurrencyRegistrarMembershipEventPayload,
        CurrencyRegistrarMembershipRemoveRejectionReason, CurrencyRegistrarMembershipRemoveResult,
        CurrencyRegistrarMembershipState, CurrencyRegistrarMembershipStatus,
    };
    use crate::currency_registrar::CurrencyRegistrarId;

    #[test]
    fn removal_terminates_one_membership_and_releases_the_pair() {
        let currency_registrar_id = CurrencyRegistrarId::new();
        let user_id = UserId::new();
        let mut removed_membership = CurrencyRegistrarMembership::new();
        let removed_membership_id = removed_membership.aggregate_id();
        removed_membership
            .create(currency_registrar_id, user_id)
            .expect("initial membership should be created");
        assert_eq!(
            removed_membership.remove().expect("removal should succeed"),
            CurrencyRegistrarMembershipRemoveResult::Removed
        );
        assert!(
            !removed_membership
                .is_active()
                .expect("removed membership state should exist")
        );
        assert_eq!(
            removed_membership
                .status()
                .expect("removed membership state should exist"),
            CurrencyRegistrarMembershipStatus::Removed
        );
        assert_eq!(
            removed_membership
                .state()
                .expect("removed membership state should exist")
                .unique_entries(removed_membership_id.value())
                .expect("removed unique entries should build")
                .get(CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY),
            None
        );

        let mut new_membership = CurrencyRegistrarMembership::new();
        let new_membership_id = new_membership.aggregate_id();
        new_membership
            .create(currency_registrar_id, user_id)
            .expect("a new membership should be created");
        assert_ne!(new_membership_id, removed_membership_id);
        assert_eq!(
            new_membership
                .state()
                .expect("new membership state should exist")
                .unique_entries(new_membership_id.value())
                .expect("new unique entries should build")
                .get(CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn repeated_removal_appends_a_rejection_event() {
        let currency_registrar_id = CurrencyRegistrarId::new();
        let user_id = UserId::new();
        let mut membership = CurrencyRegistrarMembership::new();
        membership
            .create(currency_registrar_id, user_id)
            .expect("membership should be created");
        assert_eq!(
            membership.remove().expect("first removal should succeed"),
            CurrencyRegistrarMembershipRemoveResult::Removed
        );
        let event_count = membership.uncommitted_events().len();

        assert_eq!(
            membership
                .remove()
                .expect("repeated removal should be recorded"),
            CurrencyRegistrarMembershipRemoveResult::Rejected {
                reason: CurrencyRegistrarMembershipRemoveRejectionReason::AlreadyRemoved,
            }
        );
        assert_eq!(membership.uncommitted_events().len(), event_count + 1);
        assert!(matches!(
            membership
                .uncommitted_events()
                .last()
                .expect("rejection event should exist")
                .payload(),
            CurrencyRegistrarMembershipEventPayload::RemoveRejected {
                currency_registrar_id: event_currency_registrar_id,
                user_id: event_user_id,
                reason: CurrencyRegistrarMembershipRemoveRejectionReason::AlreadyRemoved,
            } if *event_currency_registrar_id == currency_registrar_id && *event_user_id == user_id
        ));
    }
}
