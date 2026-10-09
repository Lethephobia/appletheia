use std::collections::BTreeSet;

mod organization_membership_error;
mod organization_membership_event_payload;
mod organization_membership_event_payload_error;
mod organization_membership_id;
mod organization_membership_state;
mod organization_membership_state_error;
mod organization_membership_status;
mod organization_role;

pub use organization_membership_error::OrganizationMembershipError;
pub use organization_membership_event_payload::OrganizationMembershipEventPayload;
pub use organization_membership_event_payload_error::OrganizationMembershipEventPayloadError;
pub use organization_membership_id::OrganizationMembershipId;
pub use organization_membership_state::OrganizationMembershipState;
pub use organization_membership_state_error::OrganizationMembershipStateError;
pub use organization_membership_status::OrganizationMembershipStatus;
pub use organization_role::OrganizationRole;

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
    pub fn roles(&self) -> Result<&BTreeSet<OrganizationRole>, OrganizationMembershipError> {
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
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipError> {
        if self.state().is_some() {
            return Err(OrganizationMembershipError::AlreadyCreated);
        }

        self.append_event(OrganizationMembershipEventPayload::Created {
            organization_id,
            user_id,
        })?;
        Ok(())
    }

    /// Grants a role to the membership.
    pub fn grant_role(
        &mut self,
        role: OrganizationRole,
    ) -> Result<(), OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationMembershipError::Removed);
        }

        let state = self.state_required()?;
        let organization_id = state.organization_id;
        let user_id = state.user_id;
        self.append_event(OrganizationMembershipEventPayload::RoleGranted {
            organization_id,
            user_id,
            role,
        })?;
        Ok(())
    }

    /// Revokes a role from the membership.
    pub fn revoke_role(
        &mut self,
        role: OrganizationRole,
    ) -> Result<(), OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            return Err(OrganizationMembershipError::Removed);
        }

        let state = self.state_required()?;
        let organization_id = state.organization_id;
        let user_id = state.user_id;
        self.append_event(OrganizationMembershipEventPayload::RoleRevoked {
            organization_id,
            user_id,
            role,
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
            } => {
                self.set_state(Some(OrganizationMembershipState {
                    organization_id: *organization_id,
                    user_id: *user_id,
                    roles: BTreeSet::new(),
                    status: OrganizationMembershipStatus::Active,
                }));
            }
            OrganizationMembershipEventPayload::RoleGranted { role, .. } => {
                let state = self.state_required_mut()?;
                state.roles.insert(*role);
            }
            OrganizationMembershipEventPayload::RoleRevoked { role, .. } => {
                let state = self.state_required_mut()?;
                state.roles.remove(role);
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
        OrganizationMembership, OrganizationMembershipError, OrganizationMembershipEventPayload,
        OrganizationMembershipStatus, OrganizationRole,
    };
    use crate::{OrganizationId, UserId};
    use std::collections::BTreeSet;

    fn created_membership() -> OrganizationMembership {
        let mut membership = OrganizationMembership::new();
        membership
            .create(OrganizationId::new(), UserId::new())
            .expect("create should succeed");
        membership
    }

    #[test]
    fn create_initializes_state_and_records_event() {
        let organization_id = OrganizationId::new();
        let user_id = UserId::new();
        let mut membership = OrganizationMembership::new();

        membership
            .create(organization_id, user_id)
            .expect("create should succeed");

        assert!(!membership.aggregate_id().value().is_nil());
        assert_eq!(
            membership
                .organization_id()
                .expect("organization id should exist"),
            &organization_id
        );
        assert_eq!(
            membership.user_id().expect("user id should exist"),
            &user_id
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
        let mut membership = created_membership();

        let error = membership
            .create(OrganizationId::new(), UserId::new())
            .expect_err("second create should fail");

        assert!(matches!(
            error,
            super::OrganizationMembershipError::AlreadyCreated
        ));
    }

    #[test]
    fn granting_role_updates_state_and_records_event() {
        let mut membership = created_membership();
        let roles = BTreeSet::from([OrganizationRole::Admin]);

        membership
            .grant_role(OrganizationRole::Admin)
            .expect("role grant should succeed");

        assert_eq!(membership.roles().expect("roles should exist"), &roles);
        assert_eq!(membership.uncommitted_events().len(), 2);
        assert_eq!(
            membership.uncommitted_events()[1].payload().name(),
            OrganizationMembershipEventPayload::ROLE_GRANTED
        );
    }

    #[test]
    fn initial_roles_are_recorded_as_individual_grants() {
        let mut membership = created_membership();
        assert!(membership.roles().unwrap().is_empty());

        for role in [OrganizationRole::Admin, OrganizationRole::Treasurer] {
            membership.grant_role(role).unwrap();
        }

        let names: Vec<_> = membership
            .uncommitted_events()
            .iter()
            .map(|event| event.payload().name())
            .collect();
        assert_eq!(
            names,
            vec![
                OrganizationMembershipEventPayload::CREATED,
                OrganizationMembershipEventPayload::ROLE_GRANTED,
                OrganizationMembershipEventPayload::ROLE_GRANTED,
            ]
        );
        assert_eq!(membership.roles().unwrap().len(), 2);
    }

    #[test]
    fn role_operations_preserve_other_roles_and_record_repeated_requests() {
        let mut membership = created_membership();
        membership.grant_role(OrganizationRole::Admin).unwrap();
        membership.grant_role(OrganizationRole::Treasurer).unwrap();
        membership.grant_role(OrganizationRole::Admin).unwrap();
        membership.revoke_role(OrganizationRole::Admin).unwrap();
        membership.revoke_role(OrganizationRole::Admin).unwrap();

        assert_eq!(
            membership.roles().unwrap(),
            &BTreeSet::from([OrganizationRole::Treasurer]),
        );
        assert_eq!(membership.uncommitted_events().len(), 6);
        assert_eq!(
            membership.uncommitted_events()[5].payload().name(),
            OrganizationMembershipEventPayload::ROLE_REVOKED,
        );
    }

    #[test]
    fn revoking_role_of_removed_membership_is_rejected() {
        let mut membership = created_membership();
        membership.remove().unwrap();

        assert!(matches!(
            membership.revoke_role(OrganizationRole::Admin),
            Err(OrganizationMembershipError::Removed),
        ));
        assert_eq!(membership.uncommitted_events().len(), 2);
    }

    #[test]
    fn removing_membership_updates_status_and_records_event() {
        let mut membership = created_membership();

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
        let mut membership = created_membership();
        membership.remove().expect("remove should succeed");

        let error = membership.remove().expect_err("second remove should fail");

        assert!(matches!(error, OrganizationMembershipError::Removed));
        assert_eq!(membership.uncommitted_events().len(), 2);
    }

    #[test]
    fn granting_role_of_removed_membership_is_rejected() {
        let mut membership = created_membership();
        membership.remove().expect("remove should succeed");

        let error = membership
            .grant_role(OrganizationRole::Admin)
            .expect_err("role grant should fail");

        assert!(matches!(error, OrganizationMembershipError::Removed));
        assert_eq!(membership.uncommitted_events().len(), 2);
    }
}
