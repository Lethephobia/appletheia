use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::{OrganizationId, OrganizationRoles, UserId};

use super::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationInvitationIssuer,
    OrganizationInvitationStateError, OrganizationInvitationStatus,
};

/// Stores the materialized state of an `OrganizationInvitation` aggregate.
#[aggregate_state(error = OrganizationInvitationStateError)]
#[unique_constraints(
    entry(key = "organization_invitee", value = organization_invitee_unique_value)
)]
#[reference_indexes(
    entry(key = "organization", value = organization_ref_value),
    entry(key = "invitee", value = invitee_ref_value),
    entry(key = "issuer_user", value = issuer_user_ref_value)
)]
pub struct OrganizationInvitationState {
    pub(super) id: OrganizationInvitationId,
    pub(super) organization_id: OrganizationId,
    pub(super) invitee_id: UserId,
    pub(super) roles: OrganizationRoles,
    pub(super) issuer: OrganizationInvitationIssuer,
    pub(super) expires_at: OrganizationInvitationExpiresAt,
    pub(super) status: OrganizationInvitationStatus,
}

fn organization_invitee_unique_value(
    state: &OrganizationInvitationState,
) -> Result<Option<UniqueValue>, OrganizationInvitationStateError> {
    if !state.status.is_pending() {
        return Ok(None);
    }

    let organization_id = state.organization_id.value().to_string();
    let invitee_id = state.invitee_id.value().to_string();
    let value = UniqueValue::from_strings([organization_id.as_str(), invitee_id.as_str()])?;

    Ok(Some(value))
}

fn organization_ref_value(
    state: &OrganizationInvitationState,
) -> Result<Option<OrganizationId>, OrganizationInvitationStateError> {
    Ok(Some(state.organization_id))
}

fn invitee_ref_value(
    state: &OrganizationInvitationState,
) -> Result<Option<UserId>, OrganizationInvitationStateError> {
    Ok(Some(state.invitee_id))
}

fn issuer_user_ref_value(
    state: &OrganizationInvitationState,
) -> Result<Option<UserId>, OrganizationInvitationStateError> {
    let user_id = match state.issuer {
        OrganizationInvitationIssuer::User(user_id) => Some(user_id),
        OrganizationInvitationIssuer::System => None,
    };

    Ok(user_id)
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{
        AggregateState, ReferenceIndexes, ReferenceValues, UniqueConstraints, UniqueValues,
    };
    use chrono::{Duration, Utc};

    use crate::{OrganizationId, OrganizationRoles, UserId};

    use super::{
        OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationInvitationIssuer,
        OrganizationInvitationState, OrganizationInvitationStatus,
    };

    fn expires_at() -> OrganizationInvitationExpiresAt {
        OrganizationInvitationExpiresAt::from(Utc::now() + Duration::minutes(10))
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationInvitationId::new();
        let state = OrganizationInvitationState {
            id,
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
            roles: OrganizationRoles::default(),
            issuer: OrganizationInvitationIssuer::User(UserId::new()),
            expires_at: expires_at(),
            status: OrganizationInvitationStatus::Pending,
        };

        assert_eq!(state.id(), id);
    }

    #[test]
    fn pending_state_returns_unique_entries_for_organization_and_invitee() {
        let state = OrganizationInvitationState {
            id: OrganizationInvitationId::new(),
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
            roles: OrganizationRoles::default(),
            issuer: OrganizationInvitationIssuer::User(UserId::new()),
            expires_at: expires_at(),
            status: OrganizationInvitationStatus::Pending,
        };

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationInvitationState::ORGANIZATION_INVITEE_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn non_pending_state_has_no_unique_entry() {
        let mut state = OrganizationInvitationState {
            id: OrganizationInvitationId::new(),
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
            roles: OrganizationRoles::default(),
            issuer: OrganizationInvitationIssuer::User(UserId::new()),
            expires_at: expires_at(),
            status: OrganizationInvitationStatus::Pending,
        };
        state.status = OrganizationInvitationStatus::Accepted;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationInvitationState::ORGANIZATION_INVITEE_KEY)
                .map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn returns_reference_entries_for_organization_invitee_and_issuer() {
        let organization_id = OrganizationId::new();
        let invitee_id = UserId::new();
        let issuer_id = UserId::new();
        let state = OrganizationInvitationState {
            id: OrganizationInvitationId::new(),
            organization_id,
            invitee_id,
            roles: OrganizationRoles::default(),
            issuer: OrganizationInvitationIssuer::User(issuer_id),
            expires_at: expires_at(),
            status: OrganizationInvitationStatus::Pending,
        };

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(OrganizationInvitationState::ORGANIZATION_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(OrganizationInvitationState::INVITEE_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(OrganizationInvitationState::ISSUER_USER_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }

    #[test]
    fn system_issued_invitation_has_no_issuer_reference_entry() {
        let state = OrganizationInvitationState {
            id: OrganizationInvitationId::new(),
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
            roles: OrganizationRoles::default(),
            issuer: OrganizationInvitationIssuer::System,
            expires_at: expires_at(),
            status: OrganizationInvitationStatus::Pending,
        };

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(OrganizationInvitationState::ISSUER_USER_REF)
                .map(ReferenceValues::len),
            None
        );
    }
}
