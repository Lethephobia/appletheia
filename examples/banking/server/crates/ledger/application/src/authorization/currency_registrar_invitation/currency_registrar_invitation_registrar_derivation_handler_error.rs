use appletheia::application::aggregate::SerializedAggregateError;
use banking_ledger_domain::currency_registrar_invitation::CurrencyRegistrarInvitationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationRegistrarDerivationHandlerError {
    #[error(transparent)]
    SerializedAggregate(#[from] SerializedAggregateError),

    #[error(transparent)]
    CurrencyRegistrarInvitation(#[from] CurrencyRegistrarInvitationError),
}
