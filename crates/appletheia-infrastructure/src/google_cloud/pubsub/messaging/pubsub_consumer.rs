use std::io::{Error as IoError, ErrorKind};
use std::sync::Arc;

use super::PubsubMessageCodec;
use appletheia_application::{Consumer, ConsumerError};
use google_cloud_pubsub::subscriber::MessageStream;

use super::pubsub_delivery::PubsubDelivery;

pub struct PubsubConsumer<C>
where
    C: PubsubMessageCodec,
{
    stream: MessageStream,
    codec: Arc<C>,
}

impl<C> PubsubConsumer<C>
where
    C: PubsubMessageCodec,
{
    pub(crate) fn new(stream: MessageStream, codec: Arc<C>) -> Self {
        Self { stream, codec }
    }
}

impl<C> Consumer<C::Message> for PubsubConsumer<C>
where
    C: PubsubMessageCodec,
{
    type Delivery = PubsubDelivery<C::Message>;

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

        let message = self
            .codec
            .decode(&pubsub_message)
            .map_err(|error| ConsumerError::Next(Box::new(error)))?;

        Ok(PubsubDelivery::new(handler, message))
    }
}
