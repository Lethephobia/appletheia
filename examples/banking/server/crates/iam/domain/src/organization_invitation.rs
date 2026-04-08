mod organization_invitation_error;
mod organization_invitation_event_payload;
mod organization_invitation_event_payload_error;
mod organization_invitation_expires_at;
mod organization_invitation_id;
mod organization_invitation_issuer;
mod organization_invitation_state;
mod organization_invitation_state_error;
mod organization_invitation_status;

pub use organization_invitation_error::OrganizationInvitationError;
pub use organization_invitation_event_payload::OrganizationInvitationEventPayload;
pub use organization_invitation_event_payload_error::OrganizationInvitationEventPayloadError;
pub use organization_invitation_expires_at::OrganizationInvitationExpiresAt;
pub use organization_invitation_id::OrganizationInvitationId;
pub use organization_invitation_issuer::OrganizationInvitationIssuer;
pub use organization_invitation_state::OrganizationInvitationState;
pub use organization_invitation_state_error::OrganizationInvitationStateError;
pub use organization_invitation_status::OrganizationInvitationStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};
use chrono::Utc;

use crate::{OrganizationId, UserId};

/// Represents the `OrganizationInvitation` aggregate root.
#[aggregate(type = "organization_invitation", error = OrganizationInvitationError)]
pub struct OrganizationInvitation {
    core: AggregateCore<OrganizationInvitationState, OrganizationInvitationEventPayload>,
}

impl OrganizationInvitation {
    /// Returns the organization that issued the invitation.
    pub fn organization_id(&self) -> Result<&OrganizationId, OrganizationInvitationError> {
        Ok(&self.state_required()?.organization_id)
    }

    /// Returns the invited user.
    pub fn invitee_id(&self) -> Result<&UserId, OrganizationInvitationError> {
        Ok(&self.state_required()?.invitee_id)
    }

    /// Returns who issued the invitation.
    pub fn issuer(&self) -> Result<&OrganizationInvitationIssuer, OrganizationInvitationError> {
        Ok(&self.state_required()?.issuer)
    }

    /// Returns the invitation expiration timestamp.
    pub fn expires_at(
        &self,
    ) -> Result<&OrganizationInvitationExpiresAt, OrganizationInvitationError> {
        Ok(&self.state_required()?.expires_at)
    }

    /// Returns the current invitation status.
    pub fn status(&self) -> Result<OrganizationInvitationStatus, OrganizationInvitationError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the invitation is pending.
    pub fn is_pending(&self) -> Result<bool, OrganizationInvitationError> {
        Ok(self.state_required()?.status.is_pending())
    }

    /// Returns whether the invitation is accepted.
    pub fn is_accepted(&self) -> Result<bool, OrganizationInvitationError> {
        Ok(self.state_required()?.status.is_accepted())
    }

    /// Returns whether the invitation is declined.
    pub fn is_declined(&self) -> Result<bool, OrganizationInvitationError> {
        Ok(self.state_required()?.status.is_declined())
    }

    /// Returns whether the invitation is canceled.
    pub fn is_canceled(&self) -> Result<bool, OrganizationInvitationError> {
        Ok(self.state_required()?.status.is_canceled())
    }

    /// Returns whether the invitation is expired.
    pub fn is_expired(&self) -> Result<bool, OrganizationInvitationError> {
        Ok(self.state_required()?.expires_at.value() <= Utc::now())
    }

    /// Issues a new organization invitation.
    pub fn issue(
        &mut self,
        organization_id: OrganizationId,
        invitee_id: UserId,
        issuer: OrganizationInvitationIssuer,
        expires_at: OrganizationInvitationExpiresAt,
    ) -> Result<(), OrganizationInvitationError> {
        if self.state().is_some() {
            return Err(OrganizationInvitationError::AlreadyIssued);
        }

        if expires_at.value() <= Utc::now() {
            return Err(OrganizationInvitationError::Expired);
        }

        self.append_event(OrganizationInvitationEventPayload::Issued {
            id: OrganizationInvitationId::new(),
            organization_id,
            invitee_id,
            issuer,
            expires_at,
        })
    }

    /// Accepts the invitation.
    pub fn accept(&mut self) -> Result<(), OrganizationInvitationError> {
        self.ensure_not_expired()?;
        self.ensure_pending()?;

        let state = self.state_required()?;
        self.append_event(OrganizationInvitationEventPayload::Accepted {
            organization_id: state.organization_id,
            invitee_id: state.invitee_id,
        })
    }

    /// Declines the invitation.
    pub fn decline(&mut self) -> Result<(), OrganizationInvitationError> {
        self.ensure_not_expired()?;
        self.ensure_pending()?;

        let state = self.state_required()?;
        self.append_event(OrganizationInvitationEventPayload::Declined {
            organization_id: state.organization_id,
            invitee_id: state.invitee_id,
        })
    }

    /// Cancels the invitation.
    pub fn cancel(&mut self) -> Result<(), OrganizationInvitationError> {
        self.ensure_not_expired()?;
        self.ensure_pending()?;

        let state = self.state_required()?;
        self.append_event(OrganizationInvitationEventPayload::Canceled {
            organization_id: state.organization_id,
            invitee_id: state.invitee_id,
        })
    }

    fn ensure_pending(&self) -> Result<(), OrganizationInvitationError> {
        if !self.state_required()?.status.is_pending() {
            return Err(OrganizationInvitationError::NotPending);
        }

        Ok(())
    }

    fn ensure_not_expired(&self) -> Result<(), OrganizationInvitationError> {
        if self.is_expired()? {
            return Err(OrganizationInvitationError::Expired);
        }

        Ok(())
    }
}

impl AggregateApply<OrganizationInvitationEventPayload, OrganizationInvitationError>
    for OrganizationInvitation
{
    fn apply(
        &mut self,
        payload: &OrganizationInvitationEventPayload,
    ) -> Result<(), OrganizationInvitationError> {
        match payload {
            OrganizationInvitationEventPayload::Issued {
                id,
                organization_id,
                invitee_id,
                issuer,
                expires_at,
            } => {
                self.set_state(Some(OrganizationInvitationState::new(
                    *id,
                    *organization_id,
                    *invitee_id,
                    *issuer,
                    *expires_at,
                )));
            }
            OrganizationInvitationEventPayload::Accepted { .. } => {
                self.state_required_mut()?.status = OrganizationInvitationStatus::Accepted;
            }
            OrganizationInvitationEventPayload::Declined { .. } => {
                self.state_required_mut()?.status = OrganizationInvitationStatus::Declined;
            }
            OrganizationInvitationEventPayload::Canceled { .. } => {
                self.state_required_mut()?.status = OrganizationInvitationStatus::Canceled;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use appletheia::domain::{Aggregate, AggregateId, EventPayload};

    use super::{
        OrganizationInvitation, OrganizationInvitationEventPayload,
        OrganizationInvitationExpiresAt, OrganizationInvitationIssuer,
        OrganizationInvitationStatus,
    };
    use crate::{OrganizationId, UserId};

    fn organization_id() -> OrganizationId {
        OrganizationId::new()
    }

    fn user_id() -> UserId {
        UserId::new()
    }

    fn future_expires_at() -> OrganizationInvitationExpiresAt {
        OrganizationInvitationExpiresAt::from(Utc::now() + Duration::minutes(10))
    }

    fn past_expires_at() -> OrganizationInvitationExpiresAt {
        OrganizationInvitationExpiresAt::from(Utc::now() - Duration::minutes(10))
    }

    #[test]
    fn issue_initializes_state_and_records_event() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = future_expires_at();
        let mut invitation = OrganizationInvitation::default();

        invitation
            .issue(organization_id, invitee_id, issuer, expires_at)
            .expect("issue should succeed");

        let aggregate_id = invitation
            .aggregate_id()
            .expect("aggregate id should exist");
        assert!(!aggregate_id.value().is_nil());
        assert_eq!(
            invitation
                .organization_id()
                .expect("organization id should exist"),
            &organization_id
        );
        assert_eq!(
            invitation.invitee_id().expect("invitee id should exist"),
            &invitee_id
        );
        assert_eq!(invitation.issuer().expect("issuer should exist"), &issuer);
        assert_eq!(
            invitation.expires_at().expect("expires at should exist"),
            &expires_at
        );
        assert_eq!(
            invitation.status().expect("status should exist"),
            OrganizationInvitationStatus::Pending
        );
        assert_eq!(invitation.uncommitted_events().len(), 1);
        assert_eq!(
            invitation.uncommitted_events()[0].payload().name(),
            OrganizationInvitationEventPayload::ISSUED
        );
    }

    #[test]
    fn accept_updates_status_and_records_event() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = future_expires_at();
        let mut invitation = OrganizationInvitation::default();
        invitation
            .issue(organization_id, invitee_id, issuer, expires_at)
            .expect("issue should succeed");

        invitation.accept().expect("accept should succeed");

        assert_eq!(
            invitation.status().expect("status should exist"),
            OrganizationInvitationStatus::Accepted
        );
        assert_eq!(invitation.uncommitted_events().len(), 2);
        assert_eq!(
            invitation.uncommitted_events()[1].payload().name(),
            OrganizationInvitationEventPayload::ACCEPTED
        );
    }

    #[test]
    fn decline_updates_status_and_records_event() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = future_expires_at();
        let mut invitation = OrganizationInvitation::default();
        invitation
            .issue(organization_id, invitee_id, issuer, expires_at)
            .expect("issue should succeed");

        invitation.decline().expect("decline should succeed");

        assert_eq!(
            invitation.status().expect("status should exist"),
            OrganizationInvitationStatus::Declined
        );
        assert_eq!(invitation.uncommitted_events().len(), 2);
        assert_eq!(
            invitation.uncommitted_events()[1].payload().name(),
            OrganizationInvitationEventPayload::DECLINED
        );
    }

    #[test]
    fn cancel_updates_status_and_records_event() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = future_expires_at();
        let mut invitation = OrganizationInvitation::default();
        invitation
            .issue(organization_id, invitee_id, issuer, expires_at)
            .expect("issue should succeed");

        invitation.cancel().expect("cancel should succeed");

        assert_eq!(
            invitation.status().expect("status should exist"),
            OrganizationInvitationStatus::Canceled
        );
        assert_eq!(invitation.uncommitted_events().len(), 2);
        assert_eq!(
            invitation.uncommitted_events()[1].payload().name(),
            OrganizationInvitationEventPayload::CANCELED
        );
    }

    #[test]
    fn expired_invitation_rejects_issue() {
        let mut invitation = OrganizationInvitation::default();

        let error = invitation
            .issue(
                organization_id(),
                user_id(),
                OrganizationInvitationIssuer::User(user_id()),
                past_expires_at(),
            )
            .expect_err("expired invitation should be rejected");

        assert!(matches!(error, super::OrganizationInvitationError::Expired));
    }

    #[test]
    fn expired_invitation_rejects_acceptance() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = past_expires_at();
        let mut invitation = OrganizationInvitation::default();

        invitation
            .append_event(OrganizationInvitationEventPayload::Issued {
                id: super::OrganizationInvitationId::new(),
                organization_id,
                invitee_id,
                issuer,
                expires_at,
            })
            .expect("setup event should succeed");

        let error = invitation
            .accept()
            .expect_err("expired invitation should be rejected");
        assert!(matches!(error, super::OrganizationInvitationError::Expired));
    }

    #[test]
    fn repeated_acceptance_is_rejected() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = future_expires_at();
        let mut invitation = OrganizationInvitation::default();
        invitation
            .issue(organization_id, invitee_id, issuer, expires_at)
            .expect("issue should succeed");
        invitation.accept().expect("accept should succeed");

        let error = invitation.accept().expect_err("second accept should fail");
        assert!(matches!(
            error,
            super::OrganizationInvitationError::NotPending
        ));
    }
}
