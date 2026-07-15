use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use banking_iam_domain::{OrganizationId, UserId};
use uuid::Uuid;

use crate::account::AccountOwner;

use super::{OwnedAccountClosureStateError, OwnedAccountClosureStatus};

/// Stores the materialized state of an `OwnedAccountClosure` aggregate.
#[aggregate_state(error = OwnedAccountClosureStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_ref_value),
    entry(key = "owner_organization", value = owner_organization_ref_value)
)]
pub struct OwnedAccountClosureState {
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

    use super::{OwnedAccountClosureState, OwnedAccountClosureStatus};

    #[test]
    fn state_stores_domain_attributes() {
        let owner = AccountOwner::User(UserId::new());
        let state = OwnedAccountClosureState {
            owner,
            closed_account_count: 0,
            rejected_account_count: 0,
            status: OwnedAccountClosureStatus::Requested,
        };
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn user_owned_closure_returns_user_reference_entry() {
        let user_id = UserId::new();
        let state = OwnedAccountClosureState {
            owner: AccountOwner::User(user_id),
            closed_account_count: 0,
            rejected_account_count: 0,
            status: OwnedAccountClosureStatus::Requested,
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
            closed_account_count: 0,
            rejected_account_count: 0,
            status: OwnedAccountClosureStatus::Requested,
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
