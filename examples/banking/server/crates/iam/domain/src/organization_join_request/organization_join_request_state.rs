use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationJoinRequestId, OrganizationJoinRequestStateError, OrganizationJoinRequestStatus,
};

/// Stores the materialized state of an `OrganizationJoinRequest` aggregate.
#[aggregate_state(error = OrganizationJoinRequestStateError)]
#[unique_constraints(
    entry(key = "organization_requester", value = organization_requester_value)
)]
#[reference_indexes(
    entry(key = "organization", value = organization_value),
    entry(key = "requester", value = requester_value)
)]
pub struct OrganizationJoinRequestState {
    pub(super) id: OrganizationJoinRequestId,
    pub(super) organization_id: OrganizationId,
    pub(super) requester_id: UserId,
    pub(super) status: OrganizationJoinRequestStatus,
}

fn organization_requester_value(
    state: &OrganizationJoinRequestState,
) -> Result<Option<UniqueValue>, OrganizationJoinRequestStateError> {
    if !state.status.is_pending() {
        return Ok(None);
    }

    let organization_id = state.organization_id.value().to_string();
    let requester_id = state.requester_id.value().to_string();
    let value = UniqueValue::from_strings([organization_id.as_str(), requester_id.as_str()])?;

    Ok(Some(value))
}

fn organization_value(
    state: &OrganizationJoinRequestState,
) -> Result<Option<OrganizationId>, OrganizationJoinRequestStateError> {
    Ok(Some(state.organization_id))
}

fn requester_value(
    state: &OrganizationJoinRequestState,
) -> Result<Option<UserId>, OrganizationJoinRequestStateError> {
    Ok(Some(state.requester_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{
        AggregateState, ReferenceIndexes, ReferenceValues, UniqueConstraints, UniqueValues,
    };

    use crate::{OrganizationId, UserId};

    use super::{
        OrganizationJoinRequestId, OrganizationJoinRequestState, OrganizationJoinRequestStatus,
    };

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationJoinRequestId::new();
        let state = OrganizationJoinRequestState {
            id,
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
            status: OrganizationJoinRequestStatus::Pending,
        };

        assert_eq!(state.id(), id);
    }

    #[test]
    fn pending_state_returns_unique_entries_for_organization_and_requester() {
        let state = OrganizationJoinRequestState {
            id: OrganizationJoinRequestId::new(),
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
            status: OrganizationJoinRequestStatus::Pending,
        };

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationJoinRequestState::ORGANIZATION_REQUESTER_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn non_pending_state_has_no_unique_entry() {
        let mut state = OrganizationJoinRequestState {
            id: OrganizationJoinRequestId::new(),
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
            status: OrganizationJoinRequestStatus::Pending,
        };
        state.status = OrganizationJoinRequestStatus::Approved;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationJoinRequestState::ORGANIZATION_REQUESTER_KEY)
                .map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn returns_reference_entries_for_organization_and_requester() {
        let organization_id = OrganizationId::new();
        let requester_id = UserId::new();
        let state = OrganizationJoinRequestState {
            id: OrganizationJoinRequestId::new(),
            organization_id,
            requester_id,
            status: OrganizationJoinRequestStatus::Pending,
        };

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(OrganizationJoinRequestState::ORGANIZATION_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(OrganizationJoinRequestState::REQUESTER_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
