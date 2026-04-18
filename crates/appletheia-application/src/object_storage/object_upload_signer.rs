use super::{ObjectUploadRequest, ObjectUploadSignerError, SignedObjectUploadRequest};

#[allow(async_fn_in_trait)]
pub trait ObjectUploadSigner: Send + Sync {
    async fn sign(
        &self,
        request: ObjectUploadRequest,
    ) -> Result<SignedObjectUploadRequest, ObjectUploadSignerError>;
}
