use super::{ObjectUploadRequest, ObjectUploaderError};

#[allow(async_fn_in_trait)]
pub trait ObjectUploader: Send + Sync {
    async fn upload(&self, request: ObjectUploadRequest) -> Result<(), ObjectUploaderError>;
}
