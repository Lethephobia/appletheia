mod user_role_assignment_error;
mod user_role_assignment_event_payload;
mod user_role_assignment_event_payload_error;
mod user_role_assignment_id;
mod user_role_assignment_state;
mod user_role_assignment_state_error;
mod user_role_assignment_status;

pub use user_role_assignment_error::UserRoleAssignmentError;
pub use user_role_assignment_event_payload::UserRoleAssignmentEventPayload;
pub use user_role_assignment_event_payload_error::UserRoleAssignmentEventPayloadError;
pub use user_role_assignment_id::UserRoleAssignmentId;
pub use user_role_assignment_state::UserRoleAssignmentState;
pub use user_role_assignment_state_error::UserRoleAssignmentStateError;
pub use user_role_assignment_status::UserRoleAssignmentStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::{RoleId, UserId};

/// Represents the `UserRoleAssignment` aggregate root.
#[aggregate(type = "user_role_assignment", error = UserRoleAssignmentError)]
pub struct UserRoleAssignment {
    core: AggregateCore<UserRoleAssignmentState, UserRoleAssignmentEventPayload>,
}

impl UserRoleAssignment {
    /// Returns the assigned role ID.
    pub fn role_id(&self) -> Result<&RoleId, UserRoleAssignmentError> {
        Ok(&self.state_required()?.role_id)
    }

    /// Returns the assigned user ID.
    pub fn user_id(&self) -> Result<&UserId, UserRoleAssignmentError> {
        Ok(&self.state_required()?.user_id)
    }

    /// Returns the current assignment status.
    pub fn status(&self) -> Result<UserRoleAssignmentStatus, UserRoleAssignmentError> {
        Ok(self.state_required()?.status)
    }

    /// Assigns a role to a user.
    pub fn assign(
        &mut self,
        role_id: RoleId,
        user_id: UserId,
    ) -> Result<(), UserRoleAssignmentError> {
        self.append_event(UserRoleAssignmentEventPayload::Assigned {
            id: UserRoleAssignmentId::new(),
            role_id,
            user_id,
        })
    }

    /// Revokes the assignment.
    pub fn revoke(&mut self) -> Result<(), UserRoleAssignmentError> {
        let state = self.state_required()?;
        if state.status.is_revoked() {
            return Ok(());
        }

        self.append_event(UserRoleAssignmentEventPayload::Revoked {
            id: state.id,
            role_id: state.role_id,
            user_id: state.user_id,
        })
    }

    fn ensure_not_assigned(&self) -> Result<(), UserRoleAssignmentError> {
        if self.state().is_some() {
            return Err(UserRoleAssignmentError::AlreadyAssigned);
        }

        Ok(())
    }
}

impl AggregateApply<UserRoleAssignmentEventPayload, UserRoleAssignmentError>
    for UserRoleAssignment
{
    fn apply(
        &mut self,
        payload: &UserRoleAssignmentEventPayload,
    ) -> Result<(), UserRoleAssignmentError> {
        match payload {
            UserRoleAssignmentEventPayload::Assigned {
                id,
                role_id,
                user_id,
            } => {
                self.ensure_not_assigned()?;
                self.set_state(Some(UserRoleAssignmentState::new(*id, *role_id, *user_id)));
            }
            UserRoleAssignmentEventPayload::Revoked { .. } => match self.state_required()?.status {
                UserRoleAssignmentStatus::Assigned => {
                    self.state_required_mut()?.status = UserRoleAssignmentStatus::Revoked;
                }
                UserRoleAssignmentStatus::Revoked => {}
            },
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, EventPayload};

    use crate::{RoleId, RoleName, UserId};

    use super::{UserRoleAssignment, UserRoleAssignmentEventPayload, UserRoleAssignmentStatus};

    #[test]
    fn assign_initializes_state_and_records_event() {
        let mut assignment = UserRoleAssignment::default();
        let role_name = RoleName::try_from("admin").expect("role name should be valid");
        let role_id = RoleId::from_name(&role_name);
        let user_id = UserId::new();

        assignment
            .assign(role_id, user_id)
            .expect("assign should succeed");

        assert_eq!(
            assignment.role_id().expect("role id should exist"),
            &role_id
        );
        assert_eq!(
            assignment.user_id().expect("user id should exist"),
            &user_id
        );
        assert_eq!(
            assignment.status().expect("status should exist"),
            UserRoleAssignmentStatus::Assigned
        );
        assert_eq!(assignment.uncommitted_events().len(), 1);
        assert_eq!(
            assignment.uncommitted_events()[0].payload().name(),
            UserRoleAssignmentEventPayload::ASSIGNED
        );
    }

    #[test]
    fn revoke_updates_status() {
        let mut assignment = UserRoleAssignment::default();
        let role_name = RoleName::try_from("admin").expect("role name should be valid");
        let role_id = RoleId::from_name(&role_name);
        let user_id = UserId::new();
        assignment
            .assign(role_id, user_id)
            .expect("assign should succeed");

        assignment.revoke().expect("revoke should succeed");

        assert_eq!(
            assignment.status().expect("status should exist"),
            UserRoleAssignmentStatus::Revoked
        );
        assert_eq!(
            assignment.uncommitted_events()[1].payload(),
            &UserRoleAssignmentEventPayload::Revoked {
                id: assignment
                    .aggregate_id()
                    .expect("assignment id should exist"),
                role_id,
                user_id,
            }
        );
    }
}
