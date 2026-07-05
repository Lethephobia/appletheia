use std::error::Error;

use appletheia::application::object_storage::{
    ObjectContentType, ObjectName, ObjectUploadBody, ObjectUploadRequest, ObjectUploader,
};
use appletheia::domain::AggregateId;
use banking_ledger_domain::currency::CurrencyImageRef;
use serde_json::{Map, Value};

use super::ObjectStorageMintMetadataPublisherConfig;
use crate::solana::{
    MintMetadataPublishRequest, MintMetadataPublisher, MintMetadataPublisherError,
};

/// Object storage implementation of `MintMetadataPublisher`.
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

    fn backend<E>(error: E) -> MintMetadataPublisherError
    where
        E: Error + Send + Sync + 'static,
    {
        MintMetadataPublisherError::Backend(Box::new(error))
    }

    fn metadata_object_name(request: &MintMetadataPublishRequest) -> String {
        format!(
            "currencies/{}/mint/metadata.json",
            request.currency_id().value()
        )
    }

    fn public_url(
        base_url: &url::Url,
        object_name: &str,
    ) -> Result<String, MintMetadataPublisherError> {
        base_url
            .join(object_name)
            .map(|url| url.to_string())
            .map_err(Self::backend)
    }

    fn image_url(&self, image: &CurrencyImageRef) -> Result<String, MintMetadataPublisherError> {
        match image {
            CurrencyImageRef::ObjectName(object_name) => {
                Self::public_url(self.config.image_public_base_url(), object_name.value())
            }
            CurrencyImageRef::ExternalUrl(url) => Ok(url.value().to_string()),
        }
    }

    fn metadata_body(
        &self,
        request: &MintMetadataPublishRequest,
    ) -> Result<Vec<u8>, MintMetadataPublisherError> {
        let mut body = Map::new();
        body.insert(
            "name".to_owned(),
            Value::String(request.name().value().to_owned()),
        );
        body.insert(
            "symbol".to_owned(),
            Value::String(request.symbol().value().to_owned()),
        );

        if let Some(description) = request.description() {
            body.insert(
                "description".to_owned(),
                Value::String(description.value().to_owned()),
            );
        }

        if let Some(image) = request.image() {
            body.insert("image".to_owned(), Value::String(self.image_url(image)?));
        }

        serde_json::to_vec(&body).map_err(Self::backend)
    }
}

impl<U> MintMetadataPublisher for ObjectStorageMintMetadataPublisher<U>
where
    U: ObjectUploader,
{
    async fn publish(
        &self,
        request: MintMetadataPublishRequest,
    ) -> Result<String, MintMetadataPublisherError> {
        let object_name = Self::metadata_object_name(&request);
        let metadata_uri = Self::public_url(self.config.metadata_public_base_url(), &object_name)?;
        let upload_request = ObjectUploadRequest::new(
            self.config.bucket_name().clone(),
            ObjectName::new(object_name).map_err(Self::backend)?,
            ObjectContentType::json(),
            ObjectUploadBody::new(self.metadata_body(&request)?),
        );

        self.object_uploader
            .upload(upload_request)
            .await
            .map_err(Self::backend)?;

        Ok(metadata_uri)
    }
}
