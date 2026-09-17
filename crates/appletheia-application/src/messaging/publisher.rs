use super::{PublishResult, PublishableMessage, PublisherError};

#[allow(async_fn_in_trait)]
pub trait Publisher: Send + Sync {
    type Message: PublishableMessage;

    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a Self::Message>,
        Self::Message: 'a;
}
