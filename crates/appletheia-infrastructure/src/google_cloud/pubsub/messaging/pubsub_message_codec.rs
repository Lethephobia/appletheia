use appletheia_application::PublishableMessage;
use google_cloud_pubsub::model::Message;

use super::PubsubMessageCodecError;

/// Encodes application messages and selectors for a Pub/Sub wire format.
pub trait PubsubMessageCodec: Send + Sync {
    type Message: PublishableMessage;
    type Selector: Send + Sync;

    fn encode(&self, message: &Self::Message) -> Result<Message, PubsubMessageCodecError>;

    fn decode(&self, message: &Message) -> Result<Self::Message, PubsubMessageCodecError>;

    fn encode_selector(&self, selector: &Self::Selector)
    -> Result<String, PubsubMessageCodecError>;
}
