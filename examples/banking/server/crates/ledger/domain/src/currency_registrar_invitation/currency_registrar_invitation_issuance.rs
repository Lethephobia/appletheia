use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

use super::{CurrencyRegistrarInvitationExpiresAt, CurrencyRegistrarInvitationIssuer};

/// Describes an currency registrar invitation issuance request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyRegistrarInvitationIssuance {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub invitee_id: UserId,
    pub issuer: CurrencyRegistrarInvitationIssuer,
    pub expires_at: CurrencyRegistrarInvitationExpiresAt,
}

impl CurrencyRegistrarInvitationIssuance {
    /// Returns the expiration timestamp.
    pub fn expires_at(&self) -> &CurrencyRegistrarInvitationExpiresAt {
        &self.expires_at
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        CurrencyRegistrarId,
        UserId,
        CurrencyRegistrarInvitationIssuer,
        CurrencyRegistrarInvitationExpiresAt,
    ) {
        (
            self.currency_registrar_id,
            self.invitee_id,
            self.issuer,
            self.expires_at,
        )
    }
}
