use appletheia::command;
use banking_ledger_domain::CurrencyRegistrarInvitationId;
use serde::{Deserialize, Serialize};

/// Accepts an currency registrar invitation.
#[command(name = "currency_registrar_invitation_accept")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationAcceptCommand {
    pub currency_registrar_invitation_id: CurrencyRegistrarInvitationId,
}
