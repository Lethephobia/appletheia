use super::{ObjectDeleteRequest, ObjectDeleterError};

#[allow(async_fn_in_trait)]
pub trait ObjectDeleter: Send + Sync {
    async fn delete(&self, request: ObjectDeleteRequest) -> Result<(), ObjectDeleterError>;
}
