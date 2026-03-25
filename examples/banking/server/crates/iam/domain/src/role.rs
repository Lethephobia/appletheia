mod role_error;
mod role_event_payload;
mod role_event_payload_error;
mod role_id;
mod role_id_error;
mod role_name;
mod role_name_error;
mod role_state;
mod role_state_error;

pub use role_error::RoleError;
pub use role_event_payload::RoleEventPayload;
pub use role_event_payload_error::RoleEventPayloadError;
pub use role_id::RoleId;
pub use role_id_error::RoleIdError;
pub use role_name::RoleName;
pub use role_name_error::RoleNameError;
pub use role_state::RoleState;
pub use role_state_error::RoleStateError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

/// Represents the `Role` aggregate root.
#[aggregate(type = "role", error = RoleError)]
pub struct Role {
    core: AggregateCore<RoleState, RoleEventPayload>,
}

impl Role {
    /// Returns the current role name.
    pub fn name(&self) -> Result<&RoleName, RoleError> {
        Ok(&self.state_required()?.name)
    }

    /// Creates a new role.
    pub fn create(&mut self, name: RoleName) -> Result<(), RoleError> {
        if let Some(state) = self.state()
            && state.name == name
        {
            return Ok(());
        }

        let id = RoleId::from_name(&name);
        self.append_event(RoleEventPayload::Created { id, name })
    }

    fn ensure_not_created(&self) -> Result<(), RoleError> {
        if self.state().is_some() {
            return Err(RoleError::AlreadyCreated);
        }

        Ok(())
    }
}

impl AggregateApply<RoleEventPayload, RoleError> for Role {
    fn apply(&mut self, payload: &RoleEventPayload) -> Result<(), RoleError> {
        match payload {
            RoleEventPayload::Created { id, name } => {
                self.ensure_not_created()?;
                self.set_state(Some(RoleState::new(*id, name.clone())));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};

    use super::{Role, RoleEventPayload, RoleId, RoleName};

    #[test]
    fn create_initializes_state_and_records_event() {
        let mut role = Role::default();
        let name = RoleName::try_from("admin").expect("role name should be valid");

        role.create(name.clone()).expect("create should succeed");

        assert_eq!(
            role.aggregate_id().expect("aggregate id should exist"),
            RoleId::from_name(&name)
        );
        assert_eq!(role.name().expect("name should exist"), &name);
        assert_eq!(role.uncommitted_events().len(), 1);
        assert_eq!(
            role.uncommitted_events()[0].payload().name(),
            RoleEventPayload::CREATED
        );
    }

    #[test]
    fn create_is_noop_when_role_already_exists_with_same_name() {
        let mut role = Role::default();
        let name = RoleName::try_from("admin").expect("role name should be valid");
        role.create(name.clone()).expect("create should succeed");

        role.create(name)
            .expect("repeated create should be ignored");

        assert_eq!(role.uncommitted_events().len(), 1);
    }
}
