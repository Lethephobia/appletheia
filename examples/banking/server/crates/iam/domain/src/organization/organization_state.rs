use appletheia::aggregate_state;
use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use super::{
    OrganizationHandle, OrganizationId, OrganizationName, OrganizationOwner,
    OrganizationStateError, OrganizationStatus,
};

/// Stores the materialized state of an `Organization` aggregate.
#[aggregate_state(error = OrganizationStateError)]
#[unique_constraints(entry(key = "handle", values = handle_values))]
pub struct OrganizationState {
    pub(super) id: OrganizationId,
    pub(super) status: OrganizationStatus,
    pub(super) handle: OrganizationHandle,
    pub(super) name: OrganizationName,
    pub(super) owner: OrganizationOwner,
}

impl OrganizationState {
    /// Creates a new organization state.
    pub(super) fn new(
        id: OrganizationId,
        owner: OrganizationOwner,
        handle: OrganizationHandle,
        name: OrganizationName,
    ) -> Self {
        Self {
            id,
            status: OrganizationStatus::Active,
            handle,
            name,
            owner,
        }
    }
}

fn handle_values(
    state: &OrganizationState,
) -> Result<Option<UniqueValues>, OrganizationStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let part = UniqueValuePart::try_from(state.handle.as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueValues};

    use super::{
        OrganizationHandle, OrganizationId, OrganizationName, OrganizationOwner, OrganizationState,
        OrganizationStatus,
    };

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationId::new();
        let owner = OrganizationOwner::User(crate::UserId::new());
        let handle = OrganizationHandle::try_from("acme-labs").expect("handle should be valid");
        let state = OrganizationState::new(
            id,
            owner,
            handle.clone(),
            OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        );

        assert_eq!(state.id(), id);
        assert_eq!(state.handle, handle);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn active_state_returns_unique_entries_for_handle() {
        let state = OrganizationState::new(
            OrganizationId::new(),
            OrganizationOwner::User(crate::UserId::new()),
            OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        );

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationState::HANDLE_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn removed_state_has_no_handle_unique_entry() {
        let mut state = OrganizationState::new(
            OrganizationId::new(),
            OrganizationOwner::User(crate::UserId::new()),
            OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        );
        state.status = OrganizationStatus::Removed;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationState::HANDLE_KEY)
                .map(UniqueValues::len),
            None
        );
    }
}
