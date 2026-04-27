use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationJoinRequestId, OrganizationJoinRequestStateError, OrganizationJoinRequestStatus,
};

/// Stores the materialized state of an `OrganizationJoinRequest` aggregate.
#[aggregate_state(error = OrganizationJoinRequestStateError)]
#[unique_constraints(
    entry(key = "organization_requester", values = organization_requester_values)
)]
pub struct OrganizationJoinRequestState {
    pub(super) id: OrganizationJoinRequestId,
    pub(super) organization_id: OrganizationId,
    pub(super) requester_id: UserId,
    pub(super) status: OrganizationJoinRequestStatus,
}

impl OrganizationJoinRequestState {
    /// Creates a new organization join request state.
    pub(super) fn new(
        id: OrganizationJoinRequestId,
        organization_id: OrganizationId,
        requester_id: UserId,
    ) -> Self {
        Self {
            id,
            organization_id,
            requester_id,
            status: OrganizationJoinRequestStatus::Pending,
        }
    }
}

fn organization_requester_values(
    state: &OrganizationJoinRequestState,
) -> Result<Option<UniqueValues>, OrganizationJoinRequestStateError> {
    if !state.status.is_pending() {
        return Ok(None);
    }

    let organization_id = state.organization_id.value().to_string();
    let requester_id = state.requester_id.value().to_string();
    let organization_part = UniqueValuePart::try_from(organization_id.as_str())?;
    let requester_part = UniqueValuePart::try_from(requester_id.as_str())?;
    let value = UniqueValue::new(vec![organization_part, requester_part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueValues};

    use crate::{OrganizationId, UserId};

    use super::{
        OrganizationJoinRequestId, OrganizationJoinRequestState, OrganizationJoinRequestStatus,
    };

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationJoinRequestId::new();
        let state = OrganizationJoinRequestState::new(id, OrganizationId::new(), UserId::new());

        assert_eq!(state.id(), id);
    }

    #[test]
    fn pending_state_returns_unique_entries_for_organization_and_requester() {
        let state = OrganizationJoinRequestState::new(
            OrganizationJoinRequestId::new(),
            OrganizationId::new(),
            UserId::new(),
        );

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
        let mut state = OrganizationJoinRequestState::new(
            OrganizationJoinRequestId::new(),
            OrganizationId::new(),
            UserId::new(),
        );
        state.status = OrganizationJoinRequestStatus::Approved;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationJoinRequestState::ORGANIZATION_REQUESTER_KEY)
                .map(UniqueValues::len),
            None
        );
    }
}
