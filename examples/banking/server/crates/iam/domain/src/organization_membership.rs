mod organization_membership_activate_rejection_reason;
mod organization_membership_activate_result;
mod organization_membership_deactivate_rejection_reason;
mod organization_membership_deactivate_result;
mod organization_membership_error;
mod organization_membership_event_payload;
mod organization_membership_event_payload_error;
mod organization_membership_id;
mod organization_membership_remove_rejection_reason;
mod organization_membership_remove_result;
mod organization_membership_role_grant_rejection_reason;
mod organization_membership_role_grant_result;
mod organization_membership_role_revoke_rejection_reason;
mod organization_membership_role_revoke_result;
mod organization_membership_state;
mod organization_membership_state_error;
mod organization_membership_status;
mod organization_role;

pub use organization_membership_activate_rejection_reason::OrganizationMembershipActivateRejectionReason;
pub use organization_membership_activate_result::OrganizationMembershipActivateResult;
pub use organization_membership_deactivate_rejection_reason::OrganizationMembershipDeactivateRejectionReason;
pub use organization_membership_deactivate_result::OrganizationMembershipDeactivateResult;
pub use organization_membership_error::OrganizationMembershipError;
pub use organization_membership_event_payload::OrganizationMembershipEventPayload;
pub use organization_membership_event_payload_error::OrganizationMembershipEventPayloadError;
pub use organization_membership_id::OrganizationMembershipId;
pub use organization_membership_remove_rejection_reason::OrganizationMembershipRemoveRejectionReason;
pub use organization_membership_remove_result::OrganizationMembershipRemoveResult;
pub use organization_membership_role_grant_rejection_reason::OrganizationMembershipRoleGrantRejectionReason;
pub use organization_membership_role_grant_result::OrganizationMembershipRoleGrantResult;
pub use organization_membership_role_revoke_rejection_reason::OrganizationMembershipRoleRevokeRejectionReason;
pub use organization_membership_role_revoke_result::OrganizationMembershipRoleRevokeResult;
pub use organization_membership_state::OrganizationMembershipState;
pub use organization_membership_state_error::OrganizationMembershipStateError;
pub use organization_membership_status::OrganizationMembershipStatus;
pub use organization_role::OrganizationRole;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::{OrganizationId, UserId};

/// Represents the `OrganizationMembership` aggregate root.
#[aggregate(type = "organization_membership", error = OrganizationMembershipError)]
pub struct OrganizationMembership {
    core: AggregateCore<OrganizationMembershipState, OrganizationMembershipEventPayload>,
}

impl OrganizationMembership {
    /// Returns the organization this membership belongs to.
    pub fn organization_id(&self) -> Result<&OrganizationId, OrganizationMembershipError> {
        Ok(&self.state_required()?.organization_id)
    }

    /// Returns the user this membership belongs to.
    pub fn user_id(&self) -> Result<&UserId, OrganizationMembershipError> {
        Ok(&self.state_required()?.user_id)
    }

    /// Returns the elevated roles granted through this membership.
    pub fn roles(&self) -> Result<&[OrganizationRole], OrganizationMembershipError> {
        Ok(self.state_required()?.roles.as_slice())
    }

    /// Returns the current membership status.
    pub fn status(&self) -> Result<OrganizationMembershipStatus, OrganizationMembershipError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the membership is active.
    pub fn is_active(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_active())
    }

    /// Returns whether the membership is inactive.
    pub fn is_inactive(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_inactive())
    }

    /// Returns whether the membership is removed.
    pub fn is_removed(&self) -> Result<bool, OrganizationMembershipError> {
        Ok(self.state_required()?.status.is_removed())
    }

    /// Creates a new organization membership.
    pub fn create(
        &mut self,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipError> {
        if self.state().is_some() {
            return Err(OrganizationMembershipError::AlreadyCreated);
        }

        self.append_event(OrganizationMembershipEventPayload::Created {
            id: OrganizationMembershipId::new(),
            organization_id,
            user_id,
        })
    }

    /// Grants a role to an active membership.
    pub fn grant_role(
        &mut self,
        role: OrganizationRole,
    ) -> Result<OrganizationMembershipRoleGrantResult, OrganizationMembershipError> {
        match self.state_required()?.status {
            OrganizationMembershipStatus::Active => {}
            OrganizationMembershipStatus::Inactive => {
                let reason = OrganizationMembershipRoleGrantRejectionReason::Inactive;
                let state = self.state_required()?;
                self.append_event(OrganizationMembershipEventPayload::RoleGrantRejected {
                    organization_id: state.organization_id,
                    user_id: state.user_id,
                    role,
                    reason,
                })?;
                return Ok(OrganizationMembershipRoleGrantResult::Rejected { reason });
            }
            OrganizationMembershipStatus::Removed => {
                let reason = OrganizationMembershipRoleGrantRejectionReason::Removed;
                let state = self.state_required()?;
                self.append_event(OrganizationMembershipEventPayload::RoleGrantRejected {
                    organization_id: state.organization_id,
                    user_id: state.user_id,
                    role,
                    reason,
                })?;
                return Ok(OrganizationMembershipRoleGrantResult::Rejected { reason });
            }
        }

        let state = self.state_required()?;
        if state.roles.contains(&role) {
            let reason = OrganizationMembershipRoleGrantRejectionReason::AlreadyGranted;
            self.append_event(OrganizationMembershipEventPayload::RoleGrantRejected {
                organization_id: state.organization_id,
                user_id: state.user_id,
                role,
                reason,
            })?;
            return Ok(OrganizationMembershipRoleGrantResult::Rejected { reason });
        }

        self.append_event(OrganizationMembershipEventPayload::RoleGranted {
            organization_id: state.organization_id,
            user_id: state.user_id,
            role,
        })?;
        Ok(OrganizationMembershipRoleGrantResult::Granted)
    }

    /// Revokes a role from an active membership.
    pub fn revoke_role(
        &mut self,
        role: OrganizationRole,
    ) -> Result<OrganizationMembershipRoleRevokeResult, OrganizationMembershipError> {
        match self.state_required()?.status {
            OrganizationMembershipStatus::Active => {}
            OrganizationMembershipStatus::Inactive => {
                let reason = OrganizationMembershipRoleRevokeRejectionReason::Inactive;
                let state = self.state_required()?;
                self.append_event(OrganizationMembershipEventPayload::RoleRevokeRejected {
                    organization_id: state.organization_id,
                    user_id: state.user_id,
                    role,
                    reason,
                })?;
                return Ok(OrganizationMembershipRoleRevokeResult::Rejected { reason });
            }
            OrganizationMembershipStatus::Removed => {
                let reason = OrganizationMembershipRoleRevokeRejectionReason::Removed;
                let state = self.state_required()?;
                self.append_event(OrganizationMembershipEventPayload::RoleRevokeRejected {
                    organization_id: state.organization_id,
                    user_id: state.user_id,
                    role,
                    reason,
                })?;
                return Ok(OrganizationMembershipRoleRevokeResult::Rejected { reason });
            }
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::RoleRevoked {
            organization_id: state.organization_id,
            user_id: state.user_id,
            role,
        })?;
        Ok(OrganizationMembershipRoleRevokeResult::Revoked)
    }

    /// Activates an inactive membership.
    pub fn activate(
        &mut self,
    ) -> Result<OrganizationMembershipActivateResult, OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationMembershipActivateRejectionReason::Removed;
            let state = self.state_required()?;
            self.append_event(OrganizationMembershipEventPayload::ActivateRejected {
                organization_id: state.organization_id,
                user_id: state.user_id,
                reason,
            })?;
            return Ok(OrganizationMembershipActivateResult::Rejected { reason });
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Activated {
            organization_id: state.organization_id,
            user_id: state.user_id,
            roles: state.roles.clone(),
        })?;
        Ok(OrganizationMembershipActivateResult::Activated)
    }

    /// Deactivates an active membership.
    pub fn deactivate(
        &mut self,
    ) -> Result<OrganizationMembershipDeactivateResult, OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationMembershipDeactivateRejectionReason::Removed;
            let state = self.state_required()?;
            self.append_event(OrganizationMembershipEventPayload::DeactivateRejected {
                organization_id: state.organization_id,
                user_id: state.user_id,
                reason,
            })?;
            return Ok(OrganizationMembershipDeactivateResult::Rejected { reason });
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Inactivated {
            organization_id: state.organization_id,
            user_id: state.user_id,
        })?;
        Ok(OrganizationMembershipDeactivateResult::Deactivated)
    }

    /// Permanently removes the membership.
    pub fn remove(
        &mut self,
    ) -> Result<OrganizationMembershipRemoveResult, OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationMembershipRemoveRejectionReason::Removed;
            let state = self.state_required()?;
            self.append_event(OrganizationMembershipEventPayload::RemoveRejected {
                organization_id: state.organization_id,
                user_id: state.user_id,
                reason,
            })?;
            return Ok(OrganizationMembershipRemoveResult::Rejected { reason });
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Removed {
            organization_id: state.organization_id,
            user_id: state.user_id,
        })?;
        Ok(OrganizationMembershipRemoveResult::Removed)
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
                id,
                organization_id,
                user_id,
            } => {
                self.set_state(Some(OrganizationMembershipState::new(
                    *id,
                    *organization_id,
                    *user_id,
                    Vec::new(),
                )));
            }
            OrganizationMembershipEventPayload::RoleGranted { role, .. } => {
                self.state_required_mut()?.roles.push(*role);
            }
            OrganizationMembershipEventPayload::RoleGrantRejected { .. } => {}
            OrganizationMembershipEventPayload::RoleRevoked { role, .. } => {
                self.state_required_mut()?
                    .roles
                    .retain(|existing_role| existing_role != role);
            }
            OrganizationMembershipEventPayload::RoleRevokeRejected { .. } => {}
            OrganizationMembershipEventPayload::Activated { roles, .. } => {
                let state = self.state_required_mut()?;
                state.status = OrganizationMembershipStatus::Active;
                state.roles = deduplicated_roles(roles);
            }
            OrganizationMembershipEventPayload::ActivateRejected { .. } => {}
            OrganizationMembershipEventPayload::Inactivated { .. } => {
                self.state_required_mut()?.status = OrganizationMembershipStatus::Inactive;
            }
            OrganizationMembershipEventPayload::DeactivateRejected { .. } => {}
            OrganizationMembershipEventPayload::Removed { .. } => {
                self.state_required_mut()?.status = OrganizationMembershipStatus::Removed;
            }
            OrganizationMembershipEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}

fn deduplicated_roles(roles: &[OrganizationRole]) -> Vec<OrganizationRole> {
    let mut deduplicated = Vec::with_capacity(roles.len());

    for role in roles {
        if deduplicated.contains(role) {
            continue;
        }

        deduplicated.push(*role);
    }

    deduplicated
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{
        OrganizationMembership, OrganizationMembershipEventPayload, OrganizationMembershipStatus,
    };
    use crate::{OrganizationId, OrganizationRole, UserId};

    fn organization_id() -> OrganizationId {
        OrganizationId::new()
    }

    fn user_id() -> UserId {
        UserId::new()
    }

    #[test]
    fn create_initializes_state_and_records_event() {
        let organization_id = organization_id();
        let user_id = user_id();
        let mut membership = OrganizationMembership::default();

        membership
            .create(organization_id, user_id)
            .expect("creation should succeed");

        let aggregate_id = membership
            .aggregate_id()
            .expect("aggregate id should exist");
        assert!(!aggregate_id.value().is_nil());
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
        assert_eq!(membership.roles().expect("roles should exist"), &[]);
        assert_eq!(membership.uncommitted_events().len(), 1);
        assert_eq!(
            membership.uncommitted_events()[0].payload().name(),
            OrganizationMembershipEventPayload::CREATED
        );
    }

    #[test]
    fn activate_and_deactivate_update_status_and_record_events() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let roles = vec![OrganizationRole::Treasurer];
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id_value, user_id_value)
            .expect("creation should succeed");
        membership
            .grant_role(OrganizationRole::Treasurer)
            .expect("grant should succeed");

        membership.deactivate().expect("deactivate should succeed");
        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Inactive
        );

        membership.activate().expect("activate should succeed");
        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Active
        );
        assert_eq!(membership.uncommitted_events().len(), 4);
        assert_eq!(
            membership.uncommitted_events()[2].payload(),
            &OrganizationMembershipEventPayload::Inactivated {
                organization_id: organization_id_value,
                user_id: user_id_value,
            }
        );
        assert_eq!(
            membership.uncommitted_events()[3].payload(),
            &OrganizationMembershipEventPayload::Activated {
                organization_id: organization_id_value,
                user_id: user_id_value,
                roles,
            }
        );
    }

    #[test]
    fn grant_and_revoke_role_update_state_and_record_events() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id_value, user_id_value)
            .expect("creation should succeed");

        membership
            .grant_role(OrganizationRole::FinanceManager)
            .expect("grant should succeed");
        assert!(
            membership
                .roles()
                .expect("roles should exist")
                .contains(&OrganizationRole::FinanceManager)
        );

        membership
            .revoke_role(OrganizationRole::FinanceManager)
            .expect("revoke should succeed");
        assert!(
            !membership
                .roles()
                .expect("roles should exist")
                .contains(&OrganizationRole::FinanceManager)
        );
        assert_eq!(membership.uncommitted_events().len(), 3);
        assert_eq!(
            membership.uncommitted_events()[1].payload(),
            &OrganizationMembershipEventPayload::RoleGranted {
                organization_id: organization_id_value,
                user_id: user_id_value,
                role: OrganizationRole::FinanceManager,
            }
        );
        assert_eq!(
            membership.uncommitted_events()[2].payload(),
            &OrganizationMembershipEventPayload::RoleRevoked {
                organization_id: organization_id_value,
                user_id: user_id_value,
                role: OrganizationRole::FinanceManager,
            }
        );
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id_value, user_id_value)
            .expect("creation should succeed");

        membership.remove().expect("remove should succeed");
        let duplicate_remove_result = membership
            .remove()
            .expect("duplicate remove should complete with a rejection event");

        assert_eq!(
            membership.status().expect("status should exist"),
            OrganizationMembershipStatus::Removed
        );
        assert_eq!(membership.uncommitted_events().len(), 3);
        assert!(matches!(
            duplicate_remove_result,
            super::OrganizationMembershipRemoveResult::Rejected {
                reason: super::OrganizationMembershipRemoveRejectionReason::Removed
            }
        ));
        assert_eq!(
            membership.uncommitted_events()[1].payload(),
            &OrganizationMembershipEventPayload::Removed {
                organization_id: organization_id_value,
                user_id: user_id_value,
            }
        );
    }

    #[test]
    fn removed_membership_rejects_status_changes() {
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id(), user_id())
            .expect("creation should succeed");
        membership.remove().expect("remove should succeed");

        let activate_result = membership
            .activate()
            .expect("activate should complete with a rejection event");
        assert!(matches!(
            activate_result,
            super::OrganizationMembershipActivateResult::Rejected {
                reason: super::OrganizationMembershipActivateRejectionReason::Removed
            }
        ));

        let deactivate_result = membership
            .deactivate()
            .expect("deactivate should complete with a rejection event");
        assert!(matches!(
            deactivate_result,
            super::OrganizationMembershipDeactivateResult::Rejected {
                reason: super::OrganizationMembershipDeactivateRejectionReason::Removed
            }
        ));

        let remove_result = membership
            .remove()
            .expect("remove should complete with a rejection event");
        assert!(matches!(
            remove_result,
            super::OrganizationMembershipRemoveResult::Rejected {
                reason: super::OrganizationMembershipRemoveRejectionReason::Removed
            }
        ));

        let grant_result = membership
            .grant_role(OrganizationRole::Admin)
            .expect("grant should complete with a rejection event");
        assert!(matches!(
            grant_result,
            super::OrganizationMembershipRoleGrantResult::Rejected {
                reason: super::OrganizationMembershipRoleGrantRejectionReason::Removed
            }
        ));
    }

    #[test]
    fn inactive_membership_rejects_role_changes() {
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id(), user_id())
            .expect("creation should succeed");
        membership.deactivate().expect("deactivate should succeed");

        let grant_result = membership
            .grant_role(OrganizationRole::Admin)
            .expect("grant should complete with a rejection event");
        assert!(matches!(
            grant_result,
            super::OrganizationMembershipRoleGrantResult::Rejected {
                reason: super::OrganizationMembershipRoleGrantRejectionReason::Inactive
            }
        ));

        let revoke_result = membership
            .revoke_role(OrganizationRole::Admin)
            .expect("revoke should complete with a rejection event");
        assert!(matches!(
            revoke_result,
            super::OrganizationMembershipRoleRevokeResult::Rejected {
                reason: super::OrganizationMembershipRoleRevokeRejectionReason::Inactive
            }
        ));
    }

    #[test]
    fn duplicate_grant_is_rejected_and_missing_revoke_appends_success_event() {
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id(), user_id())
            .expect("creation should succeed");
        membership
            .grant_role(OrganizationRole::Admin)
            .expect("grant should succeed");

        let duplicate_grant_result = membership
            .grant_role(OrganizationRole::Admin)
            .expect("duplicate grant rejection should be recorded");
        assert!(matches!(
            duplicate_grant_result,
            super::OrganizationMembershipRoleGrantResult::Rejected {
                reason: super::OrganizationMembershipRoleGrantRejectionReason::AlreadyGranted
            }
        ));
        assert_eq!(membership.uncommitted_events().len(), 3);
        assert_eq!(
            membership.roles().expect("roles should exist"),
            &[OrganizationRole::Admin]
        );

        membership
            .revoke_role(OrganizationRole::Admin)
            .expect("revoke should succeed");
        membership
            .revoke_role(OrganizationRole::Admin)
            .expect("missing revoke should succeed");
        assert_eq!(membership.uncommitted_events().len(), 5);
    }

    #[test]
    fn granted_roles_preserve_grant_order_through_reactivation() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let mut membership = OrganizationMembership::default();
        membership
            .create(organization_id_value, user_id_value)
            .expect("creation should succeed");
        membership
            .grant_role(OrganizationRole::Treasurer)
            .expect("grant should succeed");
        membership
            .grant_role(OrganizationRole::FinanceManager)
            .expect("grant should succeed");

        assert_eq!(
            membership.roles().expect("roles should exist"),
            &[
                OrganizationRole::Treasurer,
                OrganizationRole::FinanceManager
            ]
        );

        membership.deactivate().expect("deactivate should succeed");
        membership.activate().expect("activate should succeed");

        assert_eq!(
            membership.uncommitted_events()[4].payload(),
            &OrganizationMembershipEventPayload::Activated {
                organization_id: organization_id_value,
                user_id: user_id_value,
                roles: vec![
                    OrganizationRole::Treasurer,
                    OrganizationRole::FinanceManager,
                ],
            }
        );
    }
}
