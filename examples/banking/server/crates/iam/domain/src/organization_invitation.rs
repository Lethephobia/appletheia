mod organization_invitation_accept_rejection_reason;
mod organization_invitation_accept_result;
mod organization_invitation_cancel_rejection_reason;
mod organization_invitation_cancel_result;
mod organization_invitation_decline_rejection_reason;
mod organization_invitation_decline_result;
mod organization_invitation_error;
mod organization_invitation_event_payload;
mod organization_invitation_event_payload_error;
mod organization_invitation_expires_at;
mod organization_invitation_id;
mod organization_invitation_issuance;
mod organization_invitation_issue_rejection_reason;
mod organization_invitation_issue_result;
mod organization_invitation_issuer;
mod organization_invitation_state;
mod organization_invitation_state_error;
mod organization_invitation_status;

pub use organization_invitation_accept_rejection_reason::OrganizationInvitationAcceptRejectionReason;
pub use organization_invitation_accept_result::OrganizationInvitationAcceptResult;
pub use organization_invitation_cancel_rejection_reason::OrganizationInvitationCancelRejectionReason;
pub use organization_invitation_cancel_result::OrganizationInvitationCancelResult;
pub use organization_invitation_decline_rejection_reason::OrganizationInvitationDeclineRejectionReason;
pub use organization_invitation_decline_result::OrganizationInvitationDeclineResult;
pub use organization_invitation_error::OrganizationInvitationError;
pub use organization_invitation_event_payload::OrganizationInvitationEventPayload;
pub use organization_invitation_event_payload_error::OrganizationInvitationEventPayloadError;
pub use organization_invitation_expires_at::OrganizationInvitationExpiresAt;
pub use organization_invitation_id::OrganizationInvitationId;
pub use organization_invitation_issuance::OrganizationInvitationIssuance;
pub use organization_invitation_issue_rejection_reason::OrganizationInvitationIssueRejectionReason;
pub use organization_invitation_issue_result::OrganizationInvitationIssueResult;
pub use organization_invitation_issuer::OrganizationInvitationIssuer;
pub use organization_invitation_state::OrganizationInvitationState;
pub use organization_invitation_state_error::OrganizationInvitationStateError;
pub use organization_invitation_status::OrganizationInvitationStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use crate::{OrganizationId, OrganizationRoles, UserId};

/// Represents the `OrganizationInvitation` aggregate root.
#[aggregate(type = "organization_invitation", error = OrganizationInvitationError)]
pub struct OrganizationInvitation {
    core: AggregateCore<
        OrganizationInvitationId,
        OrganizationInvitationState,
        OrganizationInvitationEventPayload,
    >,
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

    /// Returns the membership roles granted by accepting the invitation.
    pub fn roles(&self) -> Result<&OrganizationRoles, OrganizationInvitationError> {
        Ok(&self.state_required()?.roles)
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
    pub fn is_expired(&self, now: CurrentDateTime) -> Result<bool, OrganizationInvitationError> {
        Ok(self.state_required()?.expires_at.is_expired(now))
    }

    /// Issues a new organization invitation.
    pub fn issue(
        &mut self,
        issuance: OrganizationInvitationIssuance,
        now: CurrentDateTime,
    ) -> Result<OrganizationInvitationIssueResult, OrganizationInvitationError> {
        if self.state().is_some() {
            return Err(OrganizationInvitationError::AlreadyIssued);
        }

        if issuance.expires_at().is_expired(now) {
            let reason = OrganizationInvitationIssueRejectionReason::Expired;
            self.reject_issue(issuance, reason)?;
            return Ok(OrganizationInvitationIssueResult::Rejected { reason });
        }

        let (organization_id, invitee_id, roles, issuer, expires_at) = issuance.into_parts();
        self.append_event(OrganizationInvitationEventPayload::Issued {
            organization_id,
            invitee_id,
            roles,
            issuer,
            expires_at,
        })?;
        Ok(OrganizationInvitationIssueResult::Issued)
    }

    /// Rejects an invitation issue attempt.
    pub fn reject_issue(
        &mut self,
        _issuance: OrganizationInvitationIssuance,
        reason: OrganizationInvitationIssueRejectionReason,
    ) -> Result<(), OrganizationInvitationError> {
        Err(OrganizationInvitationError::IssueRejected(reason))
    }

    /// Accepts the invitation.
    pub fn accept(
        &mut self,
        now: CurrentDateTime,
    ) -> Result<OrganizationInvitationAcceptResult, OrganizationInvitationError> {
        if self.is_expired(now)? {
            let reason = OrganizationInvitationAcceptRejectionReason::Expired;
            self.reject_accept(reason)?;
            return Ok(OrganizationInvitationAcceptResult::Rejected { reason });
        }
        if !self.state_required()?.status.is_pending() {
            let reason = OrganizationInvitationAcceptRejectionReason::NotPending;
            self.reject_accept(reason)?;
            return Ok(OrganizationInvitationAcceptResult::Rejected { reason });
        }
        let state = self.state_required()?;
        self.append_event(OrganizationInvitationEventPayload::Accepted {
            organization_id: state.organization_id,
            invitee_id: state.invitee_id,
            roles: state.roles.clone(),
        })?;
        Ok(OrganizationInvitationAcceptResult::Accepted)
    }

    /// Rejects an invitation accept attempt.
    pub fn reject_accept(
        &mut self,
        reason: OrganizationInvitationAcceptRejectionReason,
    ) -> Result<(), OrganizationInvitationError> {
        Err(OrganizationInvitationError::AcceptRejected(reason))
    }

    /// Declines the invitation.
    pub fn decline(
        &mut self,
        now: CurrentDateTime,
    ) -> Result<OrganizationInvitationDeclineResult, OrganizationInvitationError> {
        if self.is_expired(now)? {
            let reason = OrganizationInvitationDeclineRejectionReason::Expired;
            self.reject_decline(reason)?;
            return Ok(OrganizationInvitationDeclineResult::Rejected { reason });
        }
        if !self.state_required()?.status.is_pending() {
            let reason = OrganizationInvitationDeclineRejectionReason::NotPending;
            self.reject_decline(reason)?;
            return Ok(OrganizationInvitationDeclineResult::Rejected { reason });
        }
        let state = self.state_required()?;
        self.append_event(OrganizationInvitationEventPayload::Declined {
            organization_id: state.organization_id,
            invitee_id: state.invitee_id,
        })?;
        Ok(OrganizationInvitationDeclineResult::Declined)
    }

    /// Rejects an invitation decline attempt.
    pub fn reject_decline(
        &mut self,
        reason: OrganizationInvitationDeclineRejectionReason,
    ) -> Result<(), OrganizationInvitationError> {
        Err(OrganizationInvitationError::DeclineRejected(reason))
    }

    /// Cancels the invitation.
    pub fn cancel(
        &mut self,
        now: CurrentDateTime,
    ) -> Result<OrganizationInvitationCancelResult, OrganizationInvitationError> {
        if self.is_expired(now)? {
            let reason = OrganizationInvitationCancelRejectionReason::Expired;
            self.reject_cancel(reason)?;
            return Ok(OrganizationInvitationCancelResult::Rejected { reason });
        }
        if !self.state_required()?.status.is_pending() {
            let reason = OrganizationInvitationCancelRejectionReason::NotPending;
            self.reject_cancel(reason)?;
            return Ok(OrganizationInvitationCancelResult::Rejected { reason });
        }
        let state = self.state_required()?;
        self.append_event(OrganizationInvitationEventPayload::Canceled {
            organization_id: state.organization_id,
            invitee_id: state.invitee_id,
        })?;
        Ok(OrganizationInvitationCancelResult::Canceled)
    }

    /// Rejects an invitation cancel attempt.
    pub fn reject_cancel(
        &mut self,
        reason: OrganizationInvitationCancelRejectionReason,
    ) -> Result<(), OrganizationInvitationError> {
        Err(OrganizationInvitationError::CancelRejected(reason))
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
                organization_id,
                invitee_id,
                roles,
                issuer,
                expires_at,
            } => {
                self.set_state(Some(OrganizationInvitationState {
                    organization_id: *organization_id,
                    invitee_id: *invitee_id,
                    roles: roles.clone(),
                    issuer: *issuer,
                    expires_at: *expires_at,
                    status: OrganizationInvitationStatus::Pending,
                }));
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
        OrganizationInvitationExpiresAt, OrganizationInvitationIssuance,
        OrganizationInvitationIssuer, OrganizationInvitationStatus,
    };
    use crate::{OrganizationId, OrganizationRoles, UserId};
    use banking_shared_kernel_domain::timestamps::CurrentDateTime;

    fn organization_id() -> OrganizationId {
        OrganizationId::new()
    }

    fn user_id() -> UserId {
        UserId::new()
    }

    fn roles() -> OrganizationRoles {
        OrganizationRoles::default()
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
        let mut invitation = OrganizationInvitation::new();

        invitation
            .issue(
                OrganizationInvitationIssuance {
                    organization_id,
                    invitee_id,
                    roles: roles(),
                    issuer,
                    expires_at,
                },
                CurrentDateTime::new(),
            )
            .expect("issue should succeed");

        let aggregate_id = invitation.aggregate_id();
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
        assert_eq!(invitation.roles().expect("roles should exist"), &roles());
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
        let mut invitation = OrganizationInvitation::new();
        invitation
            .issue(
                OrganizationInvitationIssuance {
                    organization_id,
                    invitee_id,
                    roles: roles(),
                    issuer,
                    expires_at,
                },
                CurrentDateTime::new(),
            )
            .expect("issue should succeed");

        invitation
            .accept(CurrentDateTime::new())
            .expect("accept should succeed");

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
        let mut invitation = OrganizationInvitation::new();
        invitation
            .issue(
                OrganizationInvitationIssuance {
                    organization_id,
                    invitee_id,
                    roles: roles(),
                    issuer,
                    expires_at,
                },
                CurrentDateTime::new(),
            )
            .expect("issue should succeed");

        invitation
            .decline(CurrentDateTime::new())
            .expect("decline should succeed");

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
        let mut invitation = OrganizationInvitation::new();
        invitation
            .issue(
                OrganizationInvitationIssuance {
                    organization_id,
                    invitee_id,
                    roles: roles(),
                    issuer,
                    expires_at,
                },
                CurrentDateTime::new(),
            )
            .expect("issue should succeed");

        invitation
            .cancel(CurrentDateTime::new())
            .expect("cancel should succeed");

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
        let mut invitation = OrganizationInvitation::new();

        let error = invitation
            .issue(
                OrganizationInvitationIssuance {
                    organization_id: organization_id(),
                    invitee_id: user_id(),
                    roles: roles(),
                    issuer: OrganizationInvitationIssuer::User(user_id()),
                    expires_at: past_expires_at(),
                },
                CurrentDateTime::new(),
            )
            .expect_err("expired invitation should be rejected");

        assert!(matches!(
            error,
            super::OrganizationInvitationError::IssueRejected(
                super::OrganizationInvitationIssueRejectionReason::Expired
            )
        ));
    }

    #[test]
    fn expired_invitation_rejects_acceptance() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = past_expires_at();
        let mut invitation = OrganizationInvitation::new();

        invitation
            .append_event(OrganizationInvitationEventPayload::Issued {
                organization_id,
                invitee_id,
                roles: roles(),
                issuer,
                expires_at,
            })
            .expect("setup event should succeed");

        let error = invitation
            .accept(CurrentDateTime::new())
            .expect_err("expired invitation should reject acceptance");
        assert!(matches!(
            error,
            super::OrganizationInvitationError::AcceptRejected(
                super::OrganizationInvitationAcceptRejectionReason::Expired
            )
        ));
    }

    #[test]
    fn repeated_acceptance_is_rejected() {
        let organization_id = organization_id();
        let invitee_id = user_id();
        let issuer = OrganizationInvitationIssuer::User(user_id());
        let expires_at = future_expires_at();
        let mut invitation = OrganizationInvitation::new();
        invitation
            .issue(
                OrganizationInvitationIssuance {
                    organization_id,
                    invitee_id,
                    roles: roles(),
                    issuer,
                    expires_at,
                },
                CurrentDateTime::new(),
            )
            .expect("issue should succeed");
        invitation
            .accept(CurrentDateTime::new())
            .expect("accept should succeed");

        let error = invitation
            .accept(CurrentDateTime::new())
            .expect_err("second accept should fail");
        assert!(matches!(
            error,
            super::OrganizationInvitationError::AcceptRejected(
                super::OrganizationInvitationAcceptRejectionReason::NotPending
            )
        ));
    }
}
