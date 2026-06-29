use super::{MintProvisionReceipt, MintProvisionRequest, MintProvisionerError};

/// Provisions an on-chain mint for application use.
#[allow(async_fn_in_trait)]
pub trait MintProvisioner: Send + Sync {
    async fn provision(
        &self,
        request: MintProvisionRequest,
    ) -> Result<MintProvisionReceipt, MintProvisionerError>;
}
