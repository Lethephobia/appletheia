use appletheia_application::{
    ObjectBucketName, ObjectChecksum, ObjectChecksumAlgorithm, ObjectUploadRequest, ObjectUploader,
    ObjectUploaderError,
};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use google_cloud_storage::client::Storage;

use super::CloudStorageObjectUploaderError;

#[derive(Clone, Debug)]
pub struct CloudStorageObjectUploader {
    client: Storage,
}

impl CloudStorageObjectUploader {
    pub fn new(client: Storage) -> Self {
        Self { client }
    }

    fn bucket_resource_name(bucket_name: &ObjectBucketName) -> String {
        format!("projects/_/buckets/{}", bucket_name.as_str())
    }

    fn apply_checksum<T, S>(
        upload: google_cloud_storage::builder::storage::WriteObject<T, S>,
        checksum: &ObjectChecksum,
    ) -> Result<
        google_cloud_storage::builder::storage::WriteObject<T, S>,
        CloudStorageObjectUploaderError,
    >
    where
        S: google_cloud_storage::stub::Storage + 'static,
    {
        match checksum.algorithm() {
            ObjectChecksumAlgorithm::Md5 => {
                let md5 = BASE64_STANDARD
                    .decode(checksum.value().as_str())
                    .map_err(CloudStorageObjectUploaderError::InvalidMd5Checksum)?;
                Ok(upload.with_known_md5_hash(md5))
            }
            ObjectChecksumAlgorithm::Crc32c => {
                let crc32c = BASE64_STANDARD
                    .decode(checksum.value().as_str())
                    .map_err(CloudStorageObjectUploaderError::InvalidCrc32cChecksum)?;
                let crc32c =
                    crc32c
                        .try_into()
                        .map(u32::from_be_bytes)
                        .map_err(|value: Vec<u8>| {
                            CloudStorageObjectUploaderError::InvalidCrc32cChecksumLength {
                                length: value.len(),
                            }
                        })?;
                Ok(upload.with_known_crc32c(crc32c))
            }
            algorithm => {
                Err(CloudStorageObjectUploaderError::UnsupportedChecksumAlgorithm { algorithm })
            }
        }
    }
}

impl ObjectUploader for CloudStorageObjectUploader {
    async fn upload(&self, request: ObjectUploadRequest) -> Result<(), ObjectUploaderError> {
        let (bucket_name, object_name, content_type, body, checksum) = request.into_parts();
        let body: Vec<u8> = body.into();
        let mut upload = self
            .client
            .write_object(
                Self::bucket_resource_name(&bucket_name),
                object_name.as_str(),
                bytes::Bytes::from(body),
            )
            .set_content_type(content_type.as_str());

        if let Some(checksum) = checksum.as_ref() {
            upload = Self::apply_checksum(upload, checksum)
                .map_err(|error| ObjectUploaderError::Backend(Box::new(error)))?;
        }

        upload
            .send_unbuffered()
            .await
            .map_err(CloudStorageObjectUploaderError::Upload)
            .map_err(|error| ObjectUploaderError::Backend(Box::new(error)))?;

        Ok(())
    }
}
