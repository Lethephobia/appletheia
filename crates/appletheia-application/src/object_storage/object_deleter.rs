use super::{ObjectDeleterError, ObjectDeletionRequest};

#[allow(async_fn_in_trait)]
pub trait ObjectDeleter: Send + Sync {
    async fn delete(&self, request: ObjectDeletionRequest) -> Result<(), ObjectDeleterError>;
}
