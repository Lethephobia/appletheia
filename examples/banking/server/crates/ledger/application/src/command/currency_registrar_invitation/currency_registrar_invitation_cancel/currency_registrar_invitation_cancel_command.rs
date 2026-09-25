use appletheia::command;
use banking_ledger_domain::CurrencyRegistrarInvitationId;
use serde::{Deserialize, Serialize};

/// Cancels an currency registrar invitation.
#[command(name = "currency_registrar_invitation_cancel")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationCancelCommand {
    pub currency_registrar_invitation_id: CurrencyRegistrarInvitationId,
}
