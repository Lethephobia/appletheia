use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudStorageObjectDeleterError {
    #[error("google cloud storage object delete failed")]
    Delete(#[source] google_cloud_storage::Error),
}
