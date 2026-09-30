mod currency_registrar_invitation_error;
mod currency_registrar_invitation_event_payload;
mod currency_registrar_invitation_event_payload_error;
mod currency_registrar_invitation_expires_at;
mod currency_registrar_invitation_id;
mod currency_registrar_invitation_issuance;
mod currency_registrar_invitation_issuer;
mod currency_registrar_invitation_state;
mod currency_registrar_invitation_state_error;
mod currency_registrar_invitation_status;

pub use currency_registrar_invitation_error::CurrencyRegistrarInvitationError;
pub use currency_registrar_invitation_event_payload::CurrencyRegistrarInvitationEventPayload;
pub use currency_registrar_invitation_event_payload_error::CurrencyRegistrarInvitationEventPayloadError;
pub use currency_registrar_invitation_expires_at::CurrencyRegistrarInvitationExpiresAt;
pub use currency_registrar_invitation_id::CurrencyRegistrarInvitationId;
pub use currency_registrar_invitation_issuance::CurrencyRegistrarInvitationIssuance;
pub use currency_registrar_invitation_issuer::CurrencyRegistrarInvitationIssuer;
pub use currency_registrar_invitation_state::CurrencyRegistrarInvitationState;
pub use currency_registrar_invitation_state_error::CurrencyRegistrarInvitationStateError;
pub use currency_registrar_invitation_status::CurrencyRegistrarInvitationStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

/// Represents the `CurrencyRegistrarInvitation` aggregate root.
#[aggregate(type = "currency_registrar_invitation", error = CurrencyRegistrarInvitationError)]
pub struct CurrencyRegistrarInvitation {
    core: AggregateCore<
        CurrencyRegistrarInvitationId,
        CurrencyRegistrarInvitationState,
        CurrencyRegistrarInvitationEventPayload,
    >,
}

impl CurrencyRegistrarInvitation {
    /// Returns the registrar that issued the invitation.
    pub fn currency_registrar_id(
        &self,
    ) -> Result<&CurrencyRegistrarId, CurrencyRegistrarInvitationError> {
        Ok(&self.state_required()?.currency_registrar_id)
    }

    /// Returns the invited user.
    pub fn invitee_id(&self) -> Result<&UserId, CurrencyRegistrarInvitationError> {
        Ok(&self.state_required()?.invitee_id)
    }

    /// Returns who issued the invitation.
    pub fn issuer(
        &self,
    ) -> Result<&CurrencyRegistrarInvitationIssuer, CurrencyRegistrarInvitationError> {
        Ok(&self.state_required()?.issuer)
    }

    /// Returns the invitation expiration timestamp.
    pub fn expires_at(
        &self,
    ) -> Result<&CurrencyRegistrarInvitationExpiresAt, CurrencyRegistrarInvitationError> {
        Ok(&self.state_required()?.expires_at)
    }

    /// Returns the current invitation status.
    pub fn status(
        &self,
    ) -> Result<CurrencyRegistrarInvitationStatus, CurrencyRegistrarInvitationError> {
        Ok(self.state_required()?.status)
    }

    /// Returns whether the invitation is pending.
    pub fn is_pending(&self) -> Result<bool, CurrencyRegistrarInvitationError> {
        Ok(self.state_required()?.status.is_pending())
    }

    /// Returns whether the invitation is accepted.
    pub fn is_accepted(&self) -> Result<bool, CurrencyRegistrarInvitationError> {
        Ok(self.state_required()?.status.is_accepted())
    }

    /// Returns whether the invitation is declined.
    pub fn is_declined(&self) -> Result<bool, CurrencyRegistrarInvitationError> {
        Ok(self.state_required()?.status.is_declined())
    }

    /// Returns whether the invitation is canceled.
    pub fn is_canceled(&self) -> Result<bool, CurrencyRegistrarInvitationError> {
        Ok(self.state_required()?.status.is_canceled())
    }

    /// Returns whether the invitation is expired.
    pub fn is_expired(
        &self,
        now: CurrentDateTime,
    ) -> Result<bool, CurrencyRegistrarInvitationError> {
        Ok(self.state_required()?.expires_at.is_expired(now))
    }

    /// Issues a new currency registrar invitation.
    pub fn issue(
        &mut self,
        issuance: CurrencyRegistrarInvitationIssuance,
        now: CurrentDateTime,
    ) -> Result<(), CurrencyRegistrarInvitationError> {
        if self.state().is_some() {
            return Err(CurrencyRegistrarInvitationError::AlreadyIssued);
        }

        if issuance.expires_at().is_expired(now) {
            return Err(CurrencyRegistrarInvitationError::Expired);
        }

        let (currency_registrar_id, invitee_id, issuer, expires_at) = issuance.into_parts();
        self.append_event(CurrencyRegistrarInvitationEventPayload::Issued {
            currency_registrar_id,
            invitee_id,
            issuer,
            expires_at,
        })?;
        Ok(())
    }

    /// Accepts the invitation.
    pub fn accept(&mut self, now: CurrentDateTime) -> Result<(), CurrencyRegistrarInvitationError> {
        if self.is_expired(now)? {
            return Err(CurrencyRegistrarInvitationError::Expired);
        }
        if !self.state_required()?.status.is_pending() {
            return Err(CurrencyRegistrarInvitationError::NotPending);
        }
        let state = self.state_required()?;
        self.append_event(CurrencyRegistrarInvitationEventPayload::Accepted {
            currency_registrar_id: state.currency_registrar_id,
            invitee_id: state.invitee_id,
        })?;
        Ok(())
    }

    /// Declines the invitation.
    pub fn decline(
        &mut self,
        now: CurrentDateTime,
    ) -> Result<(), CurrencyRegistrarInvitationError> {
        if self.is_expired(now)? {
            return Err(CurrencyRegistrarInvitationError::Expired);
        }
        if !self.state_required()?.status.is_pending() {
            return Err(CurrencyRegistrarInvitationError::NotPending);
        }
        let state = self.state_required()?;
        self.append_event(CurrencyRegistrarInvitationEventPayload::Declined {
            currency_registrar_id: state.currency_registrar_id,
            invitee_id: state.invitee_id,
        })?;
        Ok(())
    }

    /// Cancels the invitation.
    pub fn cancel(&mut self, now: CurrentDateTime) -> Result<(), CurrencyRegistrarInvitationError> {
        if self.is_expired(now)? {
            return Err(CurrencyRegistrarInvitationError::Expired);
        }
        if !self.state_required()?.status.is_pending() {
            return Err(CurrencyRegistrarInvitationError::NotPending);
        }
        let state = self.state_required()?;
        self.append_event(CurrencyRegistrarInvitationEventPayload::Canceled {
            currency_registrar_id: state.currency_registrar_id,
            invitee_id: state.invitee_id,
        })?;
        Ok(())
    }
}

impl AggregateApply<CurrencyRegistrarInvitationEventPayload, CurrencyRegistrarInvitationError>
    for CurrencyRegistrarInvitation
{
    fn apply(
        &mut self,
        payload: &CurrencyRegistrarInvitationEventPayload,
    ) -> Result<(), CurrencyRegistrarInvitationError> {
        match payload {
            CurrencyRegistrarInvitationEventPayload::Issued {
                currency_registrar_id,
                invitee_id,
                issuer,
                expires_at,
            } => {
                self.set_state(Some(CurrencyRegistrarInvitationState {
                    currency_registrar_id: *currency_registrar_id,
                    invitee_id: *invitee_id,
                    issuer: *issuer,
                    expires_at: *expires_at,
                    status: CurrencyRegistrarInvitationStatus::Pending,
                }));
            }
            CurrencyRegistrarInvitationEventPayload::Accepted { .. } => {
                self.state_required_mut()?.status = CurrencyRegistrarInvitationStatus::Accepted;
            }
            CurrencyRegistrarInvitationEventPayload::Declined { .. } => {
                self.state_required_mut()?.status = CurrencyRegistrarInvitationStatus::Declined;
            }
            CurrencyRegistrarInvitationEventPayload::Canceled { .. } => {
                self.state_required_mut()?.status = CurrencyRegistrarInvitationStatus::Canceled;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::Aggregate;
    use banking_iam_domain::UserId;
    use banking_shared_kernel_domain::timestamps::CurrentDateTime;
    use chrono::{Duration, Utc};

    use super::{
        CurrencyRegistrarInvitation, CurrencyRegistrarInvitationExpiresAt,
        CurrencyRegistrarInvitationIssuance, CurrencyRegistrarInvitationIssuer,
    };
    use crate::currency_registrar::CurrencyRegistrarId;

    fn pending_invitation() -> CurrencyRegistrarInvitation {
        let mut invitation = CurrencyRegistrarInvitation::new();
        invitation
            .issue(
                CurrencyRegistrarInvitationIssuance {
                    currency_registrar_id: CurrencyRegistrarId::new(),
                    invitee_id: UserId::new(),
                    issuer: CurrencyRegistrarInvitationIssuer::System,
                    expires_at: CurrencyRegistrarInvitationExpiresAt::new(
                        Utc::now() + Duration::hours(1),
                    ),
                },
                CurrentDateTime::new(),
            )
            .expect("invitation should be issued");

        invitation
    }

    #[test]
    fn accepted_invitation_is_terminal_and_repeated_accept_returns_error() {
        let mut invitation = pending_invitation();

        invitation
            .accept(CurrentDateTime::new())
            .expect("first accept should succeed");
        invitation
            .accept(CurrentDateTime::new())
            .expect_err("repeated accept should fail");
        assert_eq!(invitation.uncommitted_events().len(), 2);
    }
}
