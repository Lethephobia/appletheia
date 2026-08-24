use appletheia::command;
use banking_ledger_domain::CurrencyRegistrarInvitationId;
use serde::{Deserialize, Serialize};

/// Declines an currency registrar invitation.
#[command(name = "currency_registrar_invitation_decline")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationDeclineCommand {
    pub currency_registrar_invitation_id: CurrencyRegistrarInvitationId,
}
