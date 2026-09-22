use std::io::{Error as IoError, ErrorKind};

use appletheia_application::{CloudEventConsumer, CloudEventConsumerError};
use google_cloud_pubsub::subscriber::MessageStream;

use super::{PubsubCloudEventCodec, PubsubCloudEventDelivery};

/// Receives binary CloudEvents with opaque byte payloads and string extensions.
/// Empty payloads become `None`, as Pub/Sub does not distinguish absence from
/// empty bytes. Unsupported or invalid events are nacked and returned as errors.
pub struct PubsubCloudEventConsumer {
    stream: MessageStream,
}

impl PubsubCloudEventConsumer {
    pub(crate) fn new(stream: MessageStream) -> Self {
        Self { stream }
    }
}

impl CloudEventConsumer for PubsubCloudEventConsumer {
    type Delivery = PubsubCloudEventDelivery;

    async fn next(&mut self) -> Result<Self::Delivery, CloudEventConsumerError> {
        let (message, handler) = self
            .stream
            .next()
            .await
            .transpose()
            .map_err(|error| CloudEventConsumerError::Next(Box::new(error)))?
            .ok_or_else(|| {
                CloudEventConsumerError::Next(Box::new(IoError::new(
                    ErrorKind::UnexpectedEof,
                    "pubsub message stream ended",
                )))
            })?;

        // On decode failure the SDK handler is dropped, which nacks the message.
        let cloud_event = PubsubCloudEventCodec::decode(&message)
            .map_err(|error| CloudEventConsumerError::Next(Box::new(error)))?;

        Ok(PubsubCloudEventDelivery::new(handler, cloud_event))
    }
}
