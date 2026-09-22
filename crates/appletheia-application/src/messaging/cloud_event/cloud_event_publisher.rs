use crate::messaging::PublishResult;

use super::{CloudEvent, CloudEventPublisherError};

#[allow(async_fn_in_trait)]
pub trait CloudEventPublisher: Send + Sync {
    /// Borrows events and associates each dispatch result with its input index.
    async fn publish<'a, I>(
        &self,
        messages: I,
    ) -> Result<Vec<PublishResult>, CloudEventPublisherError>
    where
        I: IntoIterator<Item = &'a CloudEvent>;
}
