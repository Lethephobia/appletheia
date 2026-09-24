use std::io::{Error as IoError, ErrorKind};
use std::marker::PhantomData;

use appletheia_application::{Consumer, ConsumerError};
use google_cloud_pubsub::subscriber::MessageStream;
use serde::de::DeserializeOwned;

use super::pubsub_delivery::PubsubDelivery;

pub struct PubsubConsumer<M>
where
    M: DeserializeOwned + Send + Sync + 'static,
{
    stream: MessageStream,
    _marker: PhantomData<fn() -> M>,
}

impl<M> PubsubConsumer<M>
where
    M: DeserializeOwned + Send + Sync + 'static,
{
    pub(crate) fn new(stream: MessageStream) -> Self {
        Self {
            stream,
            _marker: PhantomData,
        }
    }
}

impl<M> Consumer<M> for PubsubConsumer<M>
where
    M: DeserializeOwned + Send + Sync + 'static,
{
    type Delivery = PubsubDelivery<M>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError> {
        let (pubsub_message, handler) = self
            .stream
            .next()
            .await
            .transpose()
            .map_err(|error| ConsumerError::Next(Box::new(error)))?
            .ok_or_else(|| {
                ConsumerError::Next(Box::new(IoError::new(
                    ErrorKind::UnexpectedEof,
                    "pubsub message stream ended",
                )))
            })?;

        let message: M = serde_json::from_slice(&pubsub_message.data)
            .map_err(|error| ConsumerError::Next(Box::new(error)))?;

        Ok(PubsubDelivery::new(handler, message))
    }
}
