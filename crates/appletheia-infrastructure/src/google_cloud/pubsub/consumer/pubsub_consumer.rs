use std::marker::PhantomData;

use appletheia_application::{Consumer, ConsumerError};
use futures_util::StreamExt;
use google_cloud_pubsub::subscription::MessageStream;
use serde::de::DeserializeOwned;

use super::pubsub_delivery::PubsubDelivery;

pub struct PubsubConsumer<M> {
    stream: MessageStream,
    _marker: PhantomData<fn() -> M>,
}

impl<M> PubsubConsumer<M> {
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
        let received_message = self.stream.next().await.ok_or_else(|| {
            ConsumerError::Next(Box::new(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "pubsub message stream ended",
            )))
        })?;

        let message: M = serde_json::from_slice(&received_message.message.data)
            .map_err(|error| ConsumerError::Next(Box::new(error)))?;

        Ok(PubsubDelivery::new(received_message, message))
    }
}

