use banking_ledger_domain::deposit::DepositId;

use super::TokenDepositVerifierError;

#[allow(async_fn_in_trait)]
pub trait TokenDepositVerifier: Send + Sync {
    async fn verify(&self, deposit_id: DepositId) -> Result<(), TokenDepositVerifierError>;
}
