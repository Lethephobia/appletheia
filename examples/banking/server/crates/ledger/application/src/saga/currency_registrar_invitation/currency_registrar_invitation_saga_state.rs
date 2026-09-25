use appletheia::application::saga::SagaState;
use banking_ledger_domain::CurrencyRegistrarInvitationId;
use serde::{Deserialize, Serialize};

/// Stores data needed by the currency registrar invitation saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyRegistrarInvitationSagaState {
    pub currency_registrar_invitation_id: CurrencyRegistrarInvitationId,
}

impl CurrencyRegistrarInvitationSagaState {
    pub fn new(currency_registrar_invitation_id: CurrencyRegistrarInvitationId) -> Self {
        Self {
            currency_registrar_invitation_id,
        }
    }
}

impl SagaState for CurrencyRegistrarInvitationSagaState {}
