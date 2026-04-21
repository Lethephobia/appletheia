use appletheia_application::{
    ObjectBucketName, ObjectChecksum, ObjectChecksumAlgorithm, ObjectUploadHeader,
    ObjectUploadHeaderName, ObjectUploadHeaderValue, ObjectUploadMethod, ObjectUploadRequest,
    ObjectUploadSigner, ObjectUploadSignerError, SignedObjectUploadRequest, SignedObjectUploadUrl,
};
use google_cloud_auth::signer::Signer;
use google_cloud_storage::builder::storage::SignedUrlBuilder;
use google_cloud_storage::http::Method;

use super::CloudStorageObjectUploadSignerError;

#[derive(Clone, Debug)]
pub struct CloudStorageObjectUploadSigner {
    signer: Signer,
}

impl CloudStorageObjectUploadSigner {
    pub fn new(signer: Signer) -> Self {
        Self { signer }
    }

    fn bucket_resource_name(bucket_name: &ObjectBucketName) -> String {
        format!("projects/_/buckets/{}", bucket_name.as_str())
    }

    fn method(method: ObjectUploadMethod) -> Result<Method, CloudStorageObjectUploadSignerError> {
        match method {
            ObjectUploadMethod::Put => Ok(Method::PUT),
            _ => Err(CloudStorageObjectUploadSignerError::UnsupportedUploadMethod { method }),
        }
    }

    fn headers_for_request(
        request: &ObjectUploadRequest,
    ) -> Result<Vec<ObjectUploadHeader>, CloudStorageObjectUploadSignerError> {
        let mut headers = vec![ObjectUploadHeader::new(
            ObjectUploadHeaderName::content_type(),
            ObjectUploadHeaderValue::from_content_type(request.content_type()),
        )];

        if let Some(content_length) = request.content_length() {
            headers.push(ObjectUploadHeader::new(
                ObjectUploadHeaderName::content_length(),
                ObjectUploadHeaderValue::from_content_length(content_length),
            ));
        }

        if let Some(checksum) = request.checksum() {
            headers.push(Self::checksum_header(checksum)?);
        }

        Ok(headers)
    }

    fn checksum_header(
        checksum: &ObjectChecksum,
    ) -> Result<ObjectUploadHeader, CloudStorageObjectUploadSignerError> {
        match checksum.algorithm() {
            ObjectChecksumAlgorithm::Md5 => Ok(ObjectUploadHeader::new(
                ObjectUploadHeaderName::content_md5(),
                ObjectUploadHeaderValue::from_checksum(checksum),
            )),
            algorithm => {
                Err(CloudStorageObjectUploadSignerError::UnsupportedChecksumAlgorithm { algorithm })
            }
        }
    }

    fn signer_error(error: CloudStorageObjectUploadSignerError) -> ObjectUploadSignerError {
        ObjectUploadSignerError::Backend(Box::new(error))
    }
}

impl ObjectUploadSigner for CloudStorageObjectUploadSigner {
    async fn sign(
        &self,
        request: ObjectUploadRequest,
    ) -> Result<SignedObjectUploadRequest, ObjectUploadSignerError> {
        let expires_in = request
            .expires_in()
            .value()
            .to_std()
            .map_err(CloudStorageObjectUploadSignerError::InvalidExpiration)
            .map_err(Self::signer_error)?;
        let headers = Self::headers_for_request(&request).map_err(Self::signer_error)?;
        let mut builder = SignedUrlBuilder::for_object(
            Self::bucket_resource_name(request.bucket_name()),
            request.object_name().as_str(),
        )
        .with_method(Self::method(request.method()).map_err(Self::signer_error)?)
        .with_expiration(expires_in);

        for header in &headers {
            builder = builder.with_header(header.name().as_str(), header.value().as_str());
        }

        let url = builder
            .sign_with(&self.signer)
            .await
            .map_err(CloudStorageObjectUploadSignerError::Sign)
            .map_err(Self::signer_error)?;
        let url = url
            .parse::<SignedObjectUploadUrl>()
            .map_err(CloudStorageObjectUploadSignerError::InvalidSignedUrl)
            .map_err(Self::signer_error)?;

        Ok(SignedObjectUploadRequest::new(
            request.method(),
            url,
            request.expires_in(),
            headers,
        ))
    }
}
