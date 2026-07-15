use super::{ObjectUploadSignRequest, ObjectUploadSignerError, SignedObjectUpload};

#[allow(async_fn_in_trait)]
pub trait ObjectUploadSigner: Send + Sync {
    async fn sign(
        &self,
        request: ObjectUploadSignRequest,
    ) -> Result<SignedObjectUpload, ObjectUploadSignerError>;
}
