use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::{aggregate_state, unique_constraints};

use super::{RoleId, RoleName, RoleStateError};

/// Stores the materialized state of a `Role` aggregate.
#[aggregate_state(error = RoleStateError)]
#[unique_constraints(entry(key = "name", values = role_name_values))]
pub struct RoleState {
    pub(super) id: RoleId,
    pub(super) name: RoleName,
}

impl RoleState {
    /// Creates a new role state.
    pub(super) fn new(id: RoleId, name: RoleName) -> Self {
        Self { id, name }
    }
}

fn role_name_values(state: &RoleState) -> Result<Option<UniqueValues>, RoleStateError> {
    let part = UniqueValuePart::try_from(state.name.as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use super::{RoleId, RoleName, RoleState};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let name = RoleName::admin();
        let id = RoleId::from_name(&name);
        let state = RoleState::new(id, name);

        assert_eq!(state.id(), id);
    }

    #[test]
    fn returns_unique_entries_for_role_name() {
        let name = RoleName::admin();
        let state = RoleState::new(RoleId::from_name(&name), name);

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries.get(UniqueKey::new("name")).map(UniqueValues::len),
            Some(1)
        );
    }
}
