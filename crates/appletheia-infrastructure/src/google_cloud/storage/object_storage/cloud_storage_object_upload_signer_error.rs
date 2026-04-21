use appletheia_application::{
    ObjectChecksumAlgorithm, ObjectUploadMethod, SignedObjectUploadUrlError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudStorageObjectUploadSignerError {
    #[error("object storage upload expiration is out of range")]
    InvalidExpiration(#[source] chrono::OutOfRangeError),
    #[error("object storage checksum algorithm is not supported: {algorithm:?}")]
    UnsupportedChecksumAlgorithm { algorithm: ObjectChecksumAlgorithm },
    #[error("object storage upload method is not supported: {method:?}")]
    UnsupportedUploadMethod { method: ObjectUploadMethod },
    #[error("google cloud storage signed url is invalid")]
    InvalidSignedUrl(#[source] SignedObjectUploadUrlError),
    #[error("google cloud storage signed url signing failed")]
    Sign(#[source] google_cloud_storage::error::SigningError),
}
