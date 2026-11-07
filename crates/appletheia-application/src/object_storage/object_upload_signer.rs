use super::{ObjectUploadSignerError, ObjectUploadSigningRequest, SignedObjectUpload};

#[allow(async_fn_in_trait)]
pub trait ObjectUploadSigner: Send + Sync {
    async fn sign(
        &self,
        request: ObjectUploadSigningRequest,
    ) -> Result<SignedObjectUpload, ObjectUploadSignerError>;
}
