use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::{aggregate_state, unique_constraints};

use crate::{RoleId, UserId};

use super::{UserRoleAssignmentId, UserRoleAssignmentStateError, UserRoleAssignmentStatus};

/// Stores the materialized state of a `UserRoleAssignment` aggregate.
#[aggregate_state(error = UserRoleAssignmentStateError)]
#[unique_constraints(entry(key = "role_id_user_id", values = role_id_user_id_values))]
pub struct UserRoleAssignmentState {
    pub(super) id: UserRoleAssignmentId,
    pub(super) role_id: RoleId,
    pub(super) user_id: UserId,
    pub(super) status: UserRoleAssignmentStatus,
}

impl UserRoleAssignmentState {
    /// Creates a new user-role-assignment state.
    pub(super) fn new(id: UserRoleAssignmentId, role_id: RoleId, user_id: UserId) -> Self {
        Self {
            id,
            role_id,
            user_id,
            status: UserRoleAssignmentStatus::Assigned,
        }
    }
}

fn role_id_user_id_values(
    state: &UserRoleAssignmentState,
) -> Result<Option<UniqueValues>, UserRoleAssignmentStateError> {
    if state.status.is_revoked() {
        return Ok(None);
    }

    let role_id = UniqueValuePart::try_from(state.role_id.to_string())?;
    let user_id = UniqueValuePart::try_from(state.user_id.to_string())?;
    let value = UniqueValue::new(vec![role_id, user_id])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use crate::{RoleId, UserId};

    use super::{UserRoleAssignmentId, UserRoleAssignmentState, UserRoleAssignmentStatus};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = UserRoleAssignmentId::new();
        let role_id = RoleId::admin();
        let state = UserRoleAssignmentState::new(id, role_id, UserId::new());

        assert_eq!(state.id(), id);
    }

    #[test]
    fn assigned_state_returns_unique_entries() {
        let role_id = RoleId::admin();
        let state =
            UserRoleAssignmentState::new(UserRoleAssignmentId::new(), role_id, UserId::new());

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UniqueKey::new("role_id_user_id"))
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn revoked_state_has_no_unique_entries() {
        let role_id = RoleId::admin();
        let mut state =
            UserRoleAssignmentState::new(UserRoleAssignmentId::new(), role_id, UserId::new());
        state.status = UserRoleAssignmentStatus::Revoked;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UniqueKey::new("role_id_user_id"))
                .map(UniqueValues::len),
            None
        );
    }
}
