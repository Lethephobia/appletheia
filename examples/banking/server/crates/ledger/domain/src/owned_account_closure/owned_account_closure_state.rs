use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::account::AccountOwner;

use super::{OwnedAccountClosureId, OwnedAccountClosureStateError, OwnedAccountClosureStatus};

/// Stores the materialized state of an `OwnedAccountClosure` aggregate.
#[aggregate_state(error = OwnedAccountClosureStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_ref_value),
    entry(key = "owner_organization", value = owner_organization_ref_value)
)]
pub struct OwnedAccountClosureState {
    pub(super) id: OwnedAccountClosureId,
    pub(super) owner: AccountOwner,
    pub(super) closed_account_count: u32,
    pub(super) rejected_account_count: u32,
    pub(super) status: OwnedAccountClosureStatus,
}

impl OwnedAccountClosureState {
    pub(super) fn closed_account_count(&self) -> u32 {
        self.closed_account_count
    }

    pub(super) fn rejected_account_count(&self) -> u32 {
        self.rejected_account_count
    }
}

fn owner_user_ref_value(
    state: &OwnedAccountClosureState,
) -> Result<Option<banking_iam_domain::UserId>, OwnedAccountClosureStateError> {
    Ok(state.owner.user_id().copied())
}

fn owner_organization_ref_value(
    state: &OwnedAccountClosureState,
) -> Result<Option<banking_iam_domain::OrganizationId>, OwnedAccountClosureStateError> {
    Ok(state.owner.organization_id().copied())
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, ReferenceIndexes, ReferenceValues};
    use banking_iam_domain::{OrganizationId, UserId};

    use crate::account::AccountOwner;

    use super::{OwnedAccountClosureId, OwnedAccountClosureState, OwnedAccountClosureStatus};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OwnedAccountClosureId::new();
        let owner = AccountOwner::User(UserId::new());
        let state = OwnedAccountClosureState {
            id,
            owner,
            closed_account_count: 0,
            rejected_account_count: 0,
            status: OwnedAccountClosureStatus::Requested,
        };

        assert_eq!(state.id(), id);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn user_owned_closure_returns_user_reference_entry() {
        let user_id = UserId::new();
        let state = OwnedAccountClosureState {
            id: OwnedAccountClosureId::new(),
            owner: AccountOwner::User(user_id),
            closed_account_count: 0,
            rejected_account_count: 0,
            status: OwnedAccountClosureStatus::Requested,
        };

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(OwnedAccountClosureState::OWNER_USER_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(OwnedAccountClosureState::OWNER_ORGANIZATION_REF)
                .map(ReferenceValues::len),
            None
        );
    }

    #[test]
    fn organization_owned_closure_returns_organization_reference_entry() {
        let organization_id = OrganizationId::new();
        let state = OwnedAccountClosureState {
            id: OwnedAccountClosureId::new(),
            owner: AccountOwner::Organization(organization_id),
            closed_account_count: 0,
            rejected_account_count: 0,
            status: OwnedAccountClosureStatus::Requested,
        };

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(OwnedAccountClosureState::OWNER_USER_REF)
                .map(ReferenceValues::len),
            None
        );
        assert_eq!(
            entries
                .get(OwnedAccountClosureState::OWNER_ORGANIZATION_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
