use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationInvitationExpiresAt, OrganizationInvitationId, OrganizationInvitationIssuer,
    OrganizationInvitationStateError, OrganizationInvitationStatus,
};

/// Stores the materialized state of an `OrganizationInvitation` aggregate.
#[aggregate_state(error = OrganizationInvitationStateError)]
#[unique_constraints(
    entry(key = "organization_invitee", values = organization_invitee_values)
)]
pub struct OrganizationInvitationState {
    pub(super) id: OrganizationInvitationId,
    pub(super) status: OrganizationInvitationStatus,
    pub(super) organization_id: OrganizationId,
    pub(super) invitee_id: UserId,
    pub(super) issuer: OrganizationInvitationIssuer,
    pub(super) expires_at: OrganizationInvitationExpiresAt,
}

impl OrganizationInvitationState {
    /// Creates a new organization invitation state.
    pub(super) fn new(
        id: OrganizationInvitationId,
        organization_id: OrganizationId,
        invitee_id: UserId,
        issuer: OrganizationInvitationIssuer,
        expires_at: OrganizationInvitationExpiresAt,
    ) -> Self {
        Self {
            id,
            status: OrganizationInvitationStatus::Pending,
            organization_id,
            invitee_id,
            issuer,
            expires_at,
        }
    }
}

fn organization_invitee_values(
    state: &OrganizationInvitationState,
) -> Result<Option<UniqueValues>, OrganizationInvitationStateError> {
    if !state.status.is_pending() {
        return Ok(None);
    }

    let organization_id = state.organization_id.value().to_string();
    let invitee_id = state.invitee_id.value().to_string();
    let organization_part = UniqueValuePart::try_from(organization_id.as_str())?;
    let invitee_part = UniqueValuePart::try_from(invitee_id.as_str())?;
    let value = UniqueValue::new(vec![organization_part, invitee_part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueValues};
    use chrono::{Duration, Utc};

    use crate::{OrganizationId, UserId};

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
        let state = OrganizationInvitationState::new(
            id,
            OrganizationId::new(),
            UserId::new(),
            OrganizationInvitationIssuer::User(UserId::new()),
            expires_at(),
        );

        assert_eq!(state.id(), id);
    }

    #[test]
    fn pending_state_returns_unique_entries_for_organization_and_invitee() {
        let state = OrganizationInvitationState::new(
            OrganizationInvitationId::new(),
            OrganizationId::new(),
            UserId::new(),
            OrganizationInvitationIssuer::User(UserId::new()),
            expires_at(),
        );

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
        let mut state = OrganizationInvitationState::new(
            OrganizationInvitationId::new(),
            OrganizationId::new(),
            UserId::new(),
            OrganizationInvitationIssuer::User(UserId::new()),
            expires_at(),
        );
        state.status = OrganizationInvitationStatus::Accepted;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationInvitationState::ORGANIZATION_INVITEE_KEY)
                .map(UniqueValues::len),
            None
        );
    }
}
