use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use banking_iam_domain::{OrganizationId, UserId};
use uuid::Uuid;

use crate::account::{AccountId, AccountOwner};

use super::{OwnedAccountClosureCount, OwnedAccountClosureStateError, OwnedAccountClosureStatus};

/// Stores the materialized state of an `OwnedAccountClosure` aggregate.
#[aggregate_state(error = OwnedAccountClosureStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_ref_value),
    entry(key = "owner_organization", value = owner_organization_ref_value)
)]
pub struct OwnedAccountClosureState {
    pub(super) owner: AccountOwner,
    pub(super) requested_count: OwnedAccountClosureCount,
    pub(super) succeeded_count: OwnedAccountClosureCount,
    pub(super) failed_count: OwnedAccountClosureCount,
    pub(super) next_cursor: Option<AccountId>,
    pub(super) scan_completed: bool,
    pub(super) status: OwnedAccountClosureStatus,
}

fn owner_user_ref_value(
    state: &OwnedAccountClosureState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, OwnedAccountClosureStateError> {
    Ok(state.owner.user_id().copied())
}

fn owner_organization_ref_value(
    state: &OwnedAccountClosureState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, OwnedAccountClosureStateError> {
    Ok(state.owner.organization_id().copied())
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{ReferenceIndexes, ReferenceValues};
    use banking_iam_domain::{OrganizationId, UserId};
    use uuid::Uuid;

    use crate::account::AccountOwner;

    use super::{OwnedAccountClosureCount, OwnedAccountClosureState, OwnedAccountClosureStatus};

    #[test]
    fn state_stores_domain_attributes() {
        let owner = AccountOwner::User(UserId::new());
        let state = OwnedAccountClosureState {
            owner,
            requested_count: OwnedAccountClosureCount::default(),
            succeeded_count: OwnedAccountClosureCount::default(),
            failed_count: OwnedAccountClosureCount::default(),
            next_cursor: None,
            scan_completed: false,
            status: OwnedAccountClosureStatus::InProgress,
        };
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn user_owned_closure_returns_user_reference_entry() {
        let user_id = UserId::new();
        let state = OwnedAccountClosureState {
            owner: AccountOwner::User(user_id),
            requested_count: OwnedAccountClosureCount::default(),
            succeeded_count: OwnedAccountClosureCount::default(),
            failed_count: OwnedAccountClosureCount::default(),
            next_cursor: None,
            scan_completed: false,
            status: OwnedAccountClosureStatus::InProgress,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
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
            owner: AccountOwner::Organization(organization_id),
            requested_count: OwnedAccountClosureCount::default(),
            succeeded_count: OwnedAccountClosureCount::default(),
            failed_count: OwnedAccountClosureCount::default(),
            next_cursor: None,
            scan_completed: false,
            status: OwnedAccountClosureStatus::InProgress,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
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
