mod organization_membership_activate_rejection_reason;
mod organization_membership_activate_result;
mod organization_membership_create_rejection_reason;
mod organization_membership_create_result;
mod organization_membership_deactivate_rejection_reason;
mod organization_membership_deactivate_result;
mod organization_membership_error;
mod organization_membership_event_payload;
mod organization_membership_event_payload_error;
mod organization_membership_id;
mod organization_membership_remove_rejection_reason;
mod organization_membership_remove_result;
mod organization_membership_roles;
mod organization_membership_roles_change_rejection_reason;
mod organization_membership_roles_change_result;
mod organization_membership_state;
mod organization_membership_state_error;
mod organization_membership_status;
mod organization_role;

pub use organization_membership_activate_rejection_reason::OrganizationMembershipActivateRejectionReason;
pub use organization_membership_activate_result::OrganizationMembershipActivateResult;
pub use organization_membership_create_rejection_reason::OrganizationMembershipCreateRejectionReason;
pub use organization_membership_create_result::OrganizationMembershipCreateResult;
pub use organization_membership_deactivate_rejection_reason::OrganizationMembershipDeactivateRejectionReason;
pub use organization_membership_deactivate_result::OrganizationMembershipDeactivateResult;
pub use organization_membership_error::OrganizationMembershipError;
pub use organization_membership_event_payload::OrganizationMembershipEventPayload;
pub use organization_membership_event_payload_error::OrganizationMembershipEventPayloadError;
pub use organization_membership_id::OrganizationMembershipId;
pub use organization_membership_remove_rejection_reason::OrganizationMembershipRemoveRejectionReason;
pub use organization_membership_remove_result::OrganizationMembershipRemoveResult;
pub use organization_membership_roles::OrganizationMembershipRoles;
pub use organization_membership_roles_change_rejection_reason::OrganizationMembershipRolesChangeRejectionReason;
pub use organization_membership_roles_change_result::OrganizationMembershipRolesChangeResult;
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
    pub fn roles(&self) -> Result<&OrganizationMembershipRoles, OrganizationMembershipError> {
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
        roles: OrganizationMembershipRoles,
    ) -> Result<OrganizationMembershipCreateResult, OrganizationMembershipError> {
        if self.state().is_some() {
            return Err(OrganizationMembershipError::AlreadyCreated);
        }

        let id = OrganizationMembershipId::new();
        self.append_event(OrganizationMembershipEventPayload::Created {
            id,
            organization_id,
            user_id,
            roles,
        })?;
        Ok(OrganizationMembershipCreateResult::Created {
            organization_membership_id: id,
        })
    }

    /// Rejects an organization membership creation attempt.
    pub fn reject_create(
        &mut self,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationMembershipRoles,
        reason: OrganizationMembershipCreateRejectionReason,
    ) -> Result<OrganizationMembershipCreateResult, OrganizationMembershipError> {
        let id = OrganizationMembershipId::new();
        self.append_event(OrganizationMembershipEventPayload::CreateRejected {
            id,
            organization_id,
            user_id,
            roles,
            reason,
        })?;
        Ok(OrganizationMembershipCreateResult::Rejected { reason })
    }

    /// Changes the roles of an active membership.
    pub fn change_roles(
        &mut self,
        roles: OrganizationMembershipRoles,
    ) -> Result<OrganizationMembershipRolesChangeResult, OrganizationMembershipError> {
        match self.state_required()?.status {
            OrganizationMembershipStatus::Active => {}
            OrganizationMembershipStatus::Inactive => {
                let reason = OrganizationMembershipRolesChangeRejectionReason::Inactive;
                return self.reject_change_roles(roles, reason);
            }
            OrganizationMembershipStatus::Removed => {
                let reason = OrganizationMembershipRolesChangeRejectionReason::Removed;
                return self.reject_change_roles(roles, reason);
            }
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::RolesChanged {
            organization_id: state.organization_id,
            user_id: state.user_id,
            roles,
        })?;
        Ok(OrganizationMembershipRolesChangeResult::Changed)
    }

    /// Rejects a membership role change attempt.
    pub fn reject_change_roles(
        &mut self,
        roles: OrganizationMembershipRoles,
        reason: OrganizationMembershipRolesChangeRejectionReason,
    ) -> Result<OrganizationMembershipRolesChangeResult, OrganizationMembershipError> {
        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::RolesChangeRejected {
            organization_id: state.organization_id,
            user_id: state.user_id,
            roles,
            reason,
        })?;
        Ok(OrganizationMembershipRolesChangeResult::Rejected { reason })
    }

    /// Activates an inactive membership.
    pub fn activate(
        &mut self,
    ) -> Result<OrganizationMembershipActivateResult, OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationMembershipActivateRejectionReason::Removed;
            return self.reject_activate(reason);
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Activated {
            organization_id: state.organization_id,
            user_id: state.user_id,
            roles: state.roles.clone(),
        })?;
        Ok(OrganizationMembershipActivateResult::Activated)
    }

    /// Rejects a membership activation attempt.
    pub fn reject_activate(
        &mut self,
        reason: OrganizationMembershipActivateRejectionReason,
    ) -> Result<OrganizationMembershipActivateResult, OrganizationMembershipError> {
        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::ActivateRejected {
            organization_id: state.organization_id,
            user_id: state.user_id,
            reason,
        })?;
        Ok(OrganizationMembershipActivateResult::Rejected { reason })
    }

    /// Deactivates an active membership.
    pub fn deactivate(
        &mut self,
    ) -> Result<OrganizationMembershipDeactivateResult, OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationMembershipDeactivateRejectionReason::Removed;
            return self.reject_deactivate(reason);
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Deactivated {
            organization_id: state.organization_id,
            user_id: state.user_id,
        })?;
        Ok(OrganizationMembershipDeactivateResult::Deactivated)
    }

    /// Rejects a membership deactivation attempt.
    pub fn reject_deactivate(
        &mut self,
        reason: OrganizationMembershipDeactivateRejectionReason,
    ) -> Result<OrganizationMembershipDeactivateResult, OrganizationMembershipError> {
        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::DeactivateRejected {
            organization_id: state.organization_id,
            user_id: state.user_id,
            reason,
        })?;
        Ok(OrganizationMembershipDeactivateResult::Rejected { reason })
    }

    /// Permanently removes the membership.
    pub fn remove(
        &mut self,
    ) -> Result<OrganizationMembershipRemoveResult, OrganizationMembershipError> {
        if self.state_required()?.status.is_removed() {
            let reason = OrganizationMembershipRemoveRejectionReason::Removed;
            return self.reject_remove(reason);
        }

        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::Removed {
            organization_id: state.organization_id,
            user_id: state.user_id,
        })?;
        Ok(OrganizationMembershipRemoveResult::Removed)
    }

    /// Rejects a membership removal attempt.
    pub fn reject_remove(
        &mut self,
        reason: OrganizationMembershipRemoveRejectionReason,
    ) -> Result<OrganizationMembershipRemoveResult, OrganizationMembershipError> {
        let state = self.state_required()?;
        self.append_event(OrganizationMembershipEventPayload::RemoveRejected {
            organization_id: state.organization_id,
            user_id: state.user_id,
            reason,
        })?;
        Ok(OrganizationMembershipRemoveResult::Rejected { reason })
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
                roles,
            } => {
                self.set_state(Some(OrganizationMembershipState {
                    id: *id,
                    organization_id: *organization_id,
                    user_id: *user_id,
                    roles: roles.clone(),
                    status: OrganizationMembershipStatus::Active,
                }));
            }
            OrganizationMembershipEventPayload::CreateRejected { .. } => {}
            OrganizationMembershipEventPayload::RolesChanged { roles, .. } => {
                self.state_required_mut()?.roles = roles.clone();
            }
            OrganizationMembershipEventPayload::RolesChangeRejected { .. } => {}
            OrganizationMembershipEventPayload::Activated { roles, .. } => {
                let state = self.state_required_mut()?;
                state.status = OrganizationMembershipStatus::Active;
                state.roles = roles.clone();
            }
            OrganizationMembershipEventPayload::ActivateRejected { .. } => {}
            OrganizationMembershipEventPayload::Deactivated { .. } => {
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

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateId};

    use super::{
        OrganizationMembership, OrganizationMembershipEventPayload, OrganizationMembershipRoles,
        OrganizationMembershipStatus,
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
        let roles = OrganizationMembershipRoles::new([OrganizationRole::Admin]);
        let mut membership = OrganizationMembership::default();

        membership
            .create(organization_id, user_id, roles.clone())
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
        assert_eq!(membership.roles().expect("roles should exist"), &roles);
        assert_eq!(membership.uncommitted_events().len(), 1);
        assert_eq!(
            membership.uncommitted_events()[0].payload(),
            &OrganizationMembershipEventPayload::Created {
                id: aggregate_id,
                organization_id,
                user_id,
                roles,
            }
        );
    }

    #[test]
    fn activate_and_deactivate_update_status_and_record_events() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let roles = OrganizationMembershipRoles::new([OrganizationRole::Treasurer]);
        let mut membership = OrganizationMembership::default();
        membership
            .create(
                organization_id_value,
                user_id_value,
                OrganizationMembershipRoles::default(),
            )
            .expect("creation should succeed");
        membership
            .change_roles(roles.clone())
            .expect("role change should succeed");

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
            &OrganizationMembershipEventPayload::Deactivated {
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
    fn change_roles_updates_state_and_records_event() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let roles = OrganizationMembershipRoles::new([OrganizationRole::FinanceManager]);
        let mut membership = OrganizationMembership::default();
        membership
            .create(
                organization_id_value,
                user_id_value,
                OrganizationMembershipRoles::default(),
            )
            .expect("creation should succeed");

        membership
            .change_roles(roles.clone())
            .expect("role change should succeed");
        assert!(
            membership
                .roles()
                .expect("roles should exist")
                .contains(&OrganizationRole::FinanceManager)
        );

        membership
            .change_roles(OrganizationMembershipRoles::default())
            .expect("role change should succeed");
        assert!(
            !membership
                .roles()
                .expect("roles should exist")
                .contains(&OrganizationRole::FinanceManager)
        );
        assert_eq!(membership.uncommitted_events().len(), 3);
        assert_eq!(
            membership.uncommitted_events()[1].payload(),
            &OrganizationMembershipEventPayload::RolesChanged {
                organization_id: organization_id_value,
                user_id: user_id_value,
                roles,
            }
        );
        assert_eq!(
            membership.uncommitted_events()[2].payload(),
            &OrganizationMembershipEventPayload::RolesChanged {
                organization_id: organization_id_value,
                user_id: user_id_value,
                roles: OrganizationMembershipRoles::default(),
            }
        );
    }

    #[test]
    fn remove_updates_status_to_removed() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let mut membership = OrganizationMembership::default();
        membership
            .create(
                organization_id_value,
                user_id_value,
                OrganizationMembershipRoles::default(),
            )
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
            .create(
                organization_id(),
                user_id(),
                OrganizationMembershipRoles::default(),
            )
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

        let change_roles_result = membership
            .change_roles(OrganizationMembershipRoles::new([OrganizationRole::Admin]))
            .expect("role change should complete with a rejection event");
        assert!(matches!(
            change_roles_result,
            super::OrganizationMembershipRolesChangeResult::Rejected {
                reason: super::OrganizationMembershipRolesChangeRejectionReason::Removed
            }
        ));
    }

    #[test]
    fn inactive_membership_rejects_role_changes() {
        let mut membership = OrganizationMembership::default();
        membership
            .create(
                organization_id(),
                user_id(),
                OrganizationMembershipRoles::default(),
            )
            .expect("creation should succeed");
        membership.deactivate().expect("deactivate should succeed");

        let change_roles_result = membership
            .change_roles(OrganizationMembershipRoles::new([OrganizationRole::Admin]))
            .expect("role change should complete with a rejection event");
        assert!(matches!(
            change_roles_result,
            super::OrganizationMembershipRolesChangeResult::Rejected {
                reason: super::OrganizationMembershipRolesChangeRejectionReason::Inactive
            }
        ));
    }

    #[test]
    fn change_roles_normalizes_duplicates_and_sorts_roles() {
        let mut membership = OrganizationMembership::default();
        membership
            .create(
                organization_id(),
                user_id(),
                OrganizationMembershipRoles::default(),
            )
            .expect("creation should succeed");
        membership
            .change_roles(OrganizationMembershipRoles::new([
                OrganizationRole::Treasurer,
                OrganizationRole::Admin,
                OrganizationRole::Treasurer,
            ]))
            .expect("role change should succeed");
        assert_eq!(
            membership.roles().expect("roles should exist"),
            &OrganizationMembershipRoles::new([
                OrganizationRole::Admin,
                OrganizationRole::Treasurer,
            ])
        );
    }

    #[test]
    fn activated_roles_preserve_normalized_order_through_reactivation() {
        let organization_id_value = organization_id();
        let user_id_value = user_id();
        let roles = OrganizationMembershipRoles::new([
            OrganizationRole::Admin,
            OrganizationRole::FinanceManager,
        ]);
        let mut membership = OrganizationMembership::default();
        membership
            .create(
                organization_id_value,
                user_id_value,
                OrganizationMembershipRoles::default(),
            )
            .expect("creation should succeed");
        membership
            .change_roles(OrganizationMembershipRoles::new([
                OrganizationRole::FinanceManager,
                OrganizationRole::Admin,
            ]))
            .expect("role change should succeed");

        assert_eq!(membership.roles().expect("roles should exist"), &roles);

        membership.deactivate().expect("deactivate should succeed");
        membership.activate().expect("activate should succeed");

        assert_eq!(
            membership.uncommitted_events()[3].payload(),
            &OrganizationMembershipEventPayload::Activated {
                organization_id: organization_id_value,
                user_id: user_id_value,
                roles,
            }
        );
    }
}
