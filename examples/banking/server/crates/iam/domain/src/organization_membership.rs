mod organization_membership_creation;
mod organization_membership_error;
mod organization_membership_event_payload;
mod organization_membership_event_payload_error;
mod organization_membership_id;
mod organization_membership_state;
mod organization_membership_state_error;
mod organization_membership_status;
mod organization_role;
mod organization_roles;

pub use organization_membership_creation::OrganizationMembershipCreation;
pub use organization_membership_error::OrganizationMembershipError;
pub use organization_membership_event_payload::OrganizationMembershipEventPayload;
pub use organization_membership_event_payload_error::OrganizationMembershipEventPayloadError;
pub use organization_membership_id::OrganizationMembershipId;
pub use organization_membership_state::OrganizationMembershipState;
pub use organization_membership_state_error::OrganizationMembershipStateError;
pub use organization_membership_status::OrganizationMembershipStatus;
pub use organization_role::OrganizationRole;
pub use organization_roles::OrganizationRoles;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::{OrganizationId, UserId};

/// Represents the `OrganizationMembership` aggregate root.
///
/// Membership is modeled independently of both `Organization` and `User` so
/// that the persisted aggregate reference graph stays acyclic: membership
/// references an organization and a user, while neither root references a
/// membership.
#[aggregate(type = "organization_membership", error = OrganizationMembershipError)]
pub struct OrganizationMembership {
    core: AggregateCore<
        OrganizationMembershipId,
        OrganizationMembershipState,
        OrganizationMembershipEventPayload,
    >,
}

impl OrganizationMembership {
    /// Returns the organization the membership belongs to.
    pub fn organization_id(&self) -> Result<&OrganizationId, OrganizationMembershipError> {
        Ok(&self.state_required()?.organization_id)
    }

    /// Returns the member user.
    pub fn user_id(&self) -> Result<&UserId, OrganizationMembershipError> {
        Ok(&self.state_required()?.user_id)
    }

    /// Returns the roles granted by the membership.
    pub fn roles(&self) -> Result<&OrganizationRoles, OrganizationMembershipError> {
        Ok(&self.state_required()?.roles)
    }

    /// Returns the current membership status.
    pub fn status(&self) -> Result<OrganizationMembershipStatus, OrganizationMembershipError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the membership is active.
    pub fn is_active(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Returns whether the membership is removed.
    pub fn is_removed(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_removed())
    }

    /// Creates the membership.
    pub fn create(
        &mut self,
        creation: OrganizationMembershipCreation,
    ) -> Result<(), OrganizationMembershipError> {
        if self.state().is_some() {
            return Err(OrganizationMembershipError::AlreadyCreated);
        }

        let (organization_id, user_id, roles) = creation.into_parts();
        self.append_event(OrganizationMembershipEventPayload::Created {
            organization_id,
            user_id,
            roles,
        })?;
        Ok(())
    }

    /// Changes the roles granted by the membership.
    pub fn change_roles(
        &mut self,
        roles: OrganizationRoles,
    ) -> Result<(), OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationMembershipError::Removed);
        }

        let state = self.state_required()?;
        let organization_id = state.organization_id;
        let user_id = state.user_id;
        self.append_event(OrganizationMembershipEventPayload::RolesChanged {
            organization_id,
            user_id,
            roles,
        })?;
        Ok(())
    }

    /// Removes the membership.
    ///
    /// Removal is terminal. Rejoining creates a new membership aggregate.
    pub fn remove(&mut self) -> Result<(), OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationMembershipError::Removed);
        }

        let state = self.state_required()?;
        let organization_id = state.organization_id;
        let user_id = state.user_id;
        self.append_event(OrganizationMembershipEventPayload::Removed {
            organization_id,
            user_id,
        })?;
        Ok(())
    }
}

impl AggregateApply<OrganizationMembershipEventPayload, OrganizationMembershipError>
    for OrganizationMembership
{
    fn apply(
        &mut self,
        payload: &OrganizationMembershipEventPayload,
    ) -> Result<(), OrganizationMembershipError> {
        match payload {
            OrganizationMembershipEventPayload::Created {
                organization_id,
                user_id,
                roles,
            } => {
                self.set_state(Some(OrganizationMembershipState {
                    organization_id: *organization_id,
                    user_id: *user_id,
                    roles: roles.clone(),
                    status: OrganizationMembershipStatus::Active,
                }));
            }
            OrganizationMembershipEventPayload::RolesChanged { roles, .. } => {
                self.state_required_mut()?.roles = roles.clone();
            }
            OrganizationMembershipEventPayload::Removed { .. } => {
                self.state_required_mut()?.status = OrganizationMembershipStatus::Removed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{
        OrganizationMembership, OrganizationMembershipCreation, OrganizationMembershipError,
        OrganizationMembershipEventPayload, OrganizationMembershipStatus, OrganizationRole,
        OrganizationRoles,
    };
    use crate::{OrganizationId, UserId};

    fn creation() -> OrganizationMembershipCreation {
        OrganizationMembershipCreation {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
        }
    }

    fn created_membership() -> (OrganizationMembership, OrganizationMembershipCreation) {
        let creation = creation();
        let mut membership = OrganizationMembership::new();
        membership
            .create(creation.clone())
            .expect("create should succeed");
        (membership, creation)
    }

    #[test]
    fn create_initializes_state_and_records_event() {
        let creation = creation();
        let mut membership = OrganizationMembership::new();

        membership
            .create(creation.clone())
            .expect("create should succeed");

        assert!(!membership.aggregate_id().value().is_nil());
        assert_eq!(
            membership
                .organization_id()
                .expect("organization id should exist"),
            &creation.organization_id
        );
        assert_eq!(
            membership.user_id().expect("user id should exist"),
            &creation.user_id
        );
        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Active
        );
        assert_eq!(membership.uncommitted_events().len(), 1);
        assert_eq!(
            membership.uncommitted_events()[0].payload().name(),
            OrganizationMembershipEventPayload::CREATED
        );
    }

    #[test]
    fn creating_twice_fails() {
        let (mut membership, creation) = created_membership();

        let error = membership
            .create(creation)
            .expect_err("second create should fail");

        assert!(matches!(
            error,
            super::OrganizationMembershipError::AlreadyCreated
        ));
    }

    #[test]
    fn changing_roles_updates_state_and_records_event() {
        let (mut membership, _) = created_membership();
        let roles = OrganizationRoles::new([OrganizationRole::Admin]);

        membership
            .change_roles(roles.clone())
            .expect("roles change should succeed");

        assert_eq!(membership.roles().expect("roles should exist"), &roles);
        assert_eq!(membership.uncommitted_events().len(), 2);
        assert_eq!(
            membership.uncommitted_events()[1].payload().name(),
            OrganizationMembershipEventPayload::ROLES_CHANGED
        );
    }

    #[test]
    fn removing_membership_updates_status_and_records_event() {
        let (mut membership, _) = created_membership();

        membership.remove().expect("remove should succeed");

        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Removed
        );
        assert_eq!(membership.uncommitted_events().len(), 2);
        assert_eq!(
            membership.uncommitted_events()[1].payload().name(),
            OrganizationMembershipEventPayload::REMOVED
        );
    }

    #[test]
    fn removing_twice_is_rejected() {
        let (mut membership, _) = created_membership();
        membership.remove().expect("remove should succeed");

        let error = membership.remove().expect_err("second remove should fail");

        assert!(matches!(error, OrganizationMembershipError::Removed));
        assert_eq!(membership.uncommitted_events().len(), 2);
    }

    #[test]
    fn changing_roles_of_removed_membership_is_rejected() {
        let (mut membership, _) = created_membership();
        membership.remove().expect("remove should succeed");

        let error = membership
            .change_roles(OrganizationRoles::new([OrganizationRole::Admin]))
            .expect_err("roles change should fail");

        assert!(matches!(error, OrganizationMembershipError::Removed));
        assert_eq!(membership.uncommitted_events().len(), 2);
    }
}
