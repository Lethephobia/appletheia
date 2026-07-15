use super::{MintProvisionReceipt, MintProvisionRequest, MintProvisionerError};

#[allow(async_fn_in_trait)]
pub trait MintProvisioner: Send + Sync {
    async fn provision(
        &self,
        request: MintProvisionRequest,
    ) -> Result<MintProvisionReceipt, MintProvisionerError>;
}
