use appletheia::application::object_storage::{
    ObjectContentType, ObjectName, ObjectUploadBody, ObjectUploadRequest, ObjectUploader,
};

use crate::banking_ledger::{
    MintMetadataPublishRequest, MintMetadataPublisher, MintMetadataPublisherError, MintMetadataUri,
};

use super::{
    MintMetadataObjectName, ObjectStorageMintMetadataPublisherConfig,
    mint_metadata_body::MintMetadataBody,
};

#[derive(Clone, Debug)]
pub struct ObjectStorageMintMetadataPublisher<U>
where
    U: ObjectUploader,
{
    object_uploader: U,
    config: ObjectStorageMintMetadataPublisherConfig,
}

impl<U> ObjectStorageMintMetadataPublisher<U>
where
    U: ObjectUploader,
{
    pub fn new(object_uploader: U, config: ObjectStorageMintMetadataPublisherConfig) -> Self {
        Self {
            object_uploader,
            config,
        }
    }
}

impl<U> MintMetadataPublisher for ObjectStorageMintMetadataPublisher<U>
where
    U: ObjectUploader,
{
    async fn publish(
        &self,
        request: MintMetadataPublishRequest,
    ) -> Result<MintMetadataUri, MintMetadataPublisherError> {
        let object_name = MintMetadataObjectName::new(request.mint_id());
        let metadata_uri = self
            .config
            .metadata_public_base_url()
            .resolve(&object_name)
            .map_err(|error| MintMetadataPublisherError::Backend(Box::new(error)))?;
        let body = serde_json::to_vec(&MintMetadataBody::from(request.document()))
            .map_err(|error| MintMetadataPublisherError::Backend(Box::new(error)))?;
        let request = ObjectUploadRequest::new(
            self.config.bucket_name().clone(),
            ObjectName::new(object_name.value().to_owned())
                .map_err(|error| MintMetadataPublisherError::Backend(Box::new(error)))?,
            ObjectContentType::json(),
            ObjectUploadBody::new(body),
        );

        self.object_uploader
            .upload(request)
            .await
            .map_err(|error| MintMetadataPublisherError::Backend(Box::new(error)))?;

        Ok(metadata_uri)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::object_storage::{
        ObjectBucketName, ObjectUploadRequest, ObjectUploader, ObjectUploaderError,
    };
    use banking_ledger_domain::currency::CurrencyImageUrl;
    use serde_json::json;

    use crate::banking_ledger::{
        MintId, MintMetadataDescription, MintMetadataDocument, MintMetadataImageUri,
        MintMetadataName, MintMetadataPublishRequest, MintMetadataPublisher, MintMetadataSymbol,
    };
    use crate::banking_ledger::{
        MintMetadataPublicBaseUrl, ObjectStorageMintMetadataPublisher,
        ObjectStorageMintMetadataPublisherConfig,
    };

    #[derive(Clone, Default)]
    struct TestObjectUploader {
        request: Arc<Mutex<Option<ObjectUploadRequest>>>,
    }

    impl ObjectUploader for TestObjectUploader {
        async fn upload(&self, request: ObjectUploadRequest) -> Result<(), ObjectUploaderError> {
            *self.request.lock().expect("lock") = Some(request);
            Ok(())
        }
    }

    fn publisher() -> (
        ObjectStorageMintMetadataPublisher<TestObjectUploader>,
        Arc<Mutex<Option<ObjectUploadRequest>>>,
    ) {
        let object_uploader = TestObjectUploader::default();
        let request = object_uploader.request.clone();
        let publisher = ObjectStorageMintMetadataPublisher::new(
            object_uploader,
            ObjectStorageMintMetadataPublisherConfig::new(
                ObjectBucketName::new("metadata".to_owned()).expect("bucket name should be valid"),
                MintMetadataPublicBaseUrl::try_from("https://storage.example.com/metadata/")
                    .expect("base URL should be valid"),
            ),
        );

        (publisher, request)
    }

    #[tokio::test]
    async fn publish_uploads_metadata_json_and_returns_uri() {
        let (publisher, uploaded_request) = publisher();
        let mint_id =
            MintId::try_from("00000000000000000000000000000000").expect("mint ID should be valid");
        let document = MintMetadataDocument::new(
            MintMetadataName::try_from("USD Coin").expect("name should be valid"),
            MintMetadataSymbol::try_from("USDC").expect("symbol should be valid"),
            Some(
                MintMetadataDescription::try_from("Stablecoin backed by USD")
                    .expect("description should be valid"),
            ),
            Some(
                MintMetadataImageUri::try_from(
                    CurrencyImageUrl::try_from(
                        "https://assets.example.com/currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
                    )
                    .expect("image URL should be valid"),
                )
                .expect("image URI should be valid"),
            ),
        );

        let uri = publisher
            .publish(MintMetadataPublishRequest::new(mint_id, document))
            .await
            .expect("metadata should be published");

        assert_eq!(
            uri.value().as_str(),
            "https://storage.example.com/metadata/currencies/00000000000000000000000000000000/mint/metadata.json"
        );

        let request = uploaded_request
            .lock()
            .expect("lock")
            .clone()
            .expect("object should be uploaded");
        assert_eq!(request.bucket_name().as_str(), "metadata");
        assert_eq!(
            request.object_name().as_str(),
            "currencies/00000000000000000000000000000000/mint/metadata.json"
        );
        assert_eq!(request.content_type().as_str(), "application/json");

        let json: serde_json::Value =
            serde_json::from_slice(request.body().as_slice()).expect("body should be JSON");
        assert_eq!(
            json,
            json!({
                "name": "USD Coin",
                "symbol": "USDC",
                "description": "Stablecoin backed by USD",
                "image": "https://assets.example.com/currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002"
            })
        );
    }

    #[tokio::test]
    async fn publish_omits_absent_optional_metadata_fields() {
        let (publisher, uploaded_request) = publisher();
        let mint_id =
            MintId::try_from("00000000000000000000000000000000").expect("mint ID should be valid");
        let document = MintMetadataDocument::new(
            MintMetadataName::try_from("USD Coin").expect("name should be valid"),
            MintMetadataSymbol::try_from("USDC").expect("symbol should be valid"),
            None,
            None,
        );

        publisher
            .publish(MintMetadataPublishRequest::new(mint_id, document))
            .await
            .expect("metadata should be published");

        let request = uploaded_request
            .lock()
            .expect("lock")
            .clone()
            .expect("object should be uploaded");
        let json: serde_json::Value =
            serde_json::from_slice(request.body().as_slice()).expect("body should be JSON");
        assert_eq!(
            json,
            json!({
                "name": "USD Coin",
                "symbol": "USDC"
            })
        );
    }
}
