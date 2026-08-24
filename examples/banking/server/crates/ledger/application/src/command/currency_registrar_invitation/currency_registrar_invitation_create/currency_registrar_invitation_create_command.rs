use appletheia::command;
use banking_ledger_domain::{
    CurrencyRegistrarId, CurrencyRegistrarInvitationExpiresAt, CurrencyRegistrarInvitationIssuer,
    UserId,
};
use serde::{Deserialize, Serialize};

/// Issues an currency registrar invitation.
#[command(name = "currency_registrar_invitation_issue")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationIssueCommand {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub invitee_id: UserId,
    pub issuer: CurrencyRegistrarInvitationIssuer,
    pub expires_at: CurrencyRegistrarInvitationExpiresAt,
}
