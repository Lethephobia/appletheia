use appletheia_application::ObjectChecksumAlgorithm;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudStorageObjectUploaderError {
    #[error("object storage upload failed")]
    Upload(#[source] google_cloud_storage::Error),
    #[error("object storage checksum algorithm is not supported: {algorithm:?}")]
    UnsupportedChecksumAlgorithm { algorithm: ObjectChecksumAlgorithm },
    #[error("object storage MD5 checksum is invalid")]
    InvalidMd5Checksum(#[source] base64::DecodeError),
    #[error("object storage CRC32C checksum is invalid")]
    InvalidCrc32cChecksum(#[source] base64::DecodeError),
    #[error("object storage CRC32C checksum length is invalid: {length}")]
    InvalidCrc32cChecksumLength { length: usize },
}
