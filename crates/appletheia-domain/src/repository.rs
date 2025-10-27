pub mod repository_error;

pub use crate::aggregate::AggregateId;
pub use crate::aggregate::AggregateState;
pub use crate::event::EventPayload;
pub use crate::event::event_history_reader::EventHistoryReader;
pub use crate::repository_error::RepositoryError;

use crate::aggregate::{Aggregate, AggregateVersion};

#[allow(async_fn_in_trait)]
pub trait Repository {
    type AggregateId: AggregateId;
    type Aggregate: Aggregate<Id = Self::AggregateId>;
    type Error: From<RepositoryError<Self::Aggregate>>;
    type EventHistoryReader: EventHistoryReader<Self::Aggregate>;

    fn event_history_reader(&self) -> &Self::EventHistoryReader;

    async fn find(&self, id: Self::AggregateId) -> Result<Option<Self::Aggregate>, Self::Error> {
        let (events, snapshot) = self
            .event_history_reader()
            .read(Self::Aggregate::AGGREGATE_TYPE, id)
            .await
            .map_err(RepositoryError::EventHistoryReaderError)?;
        let mut aggregate = Self::Aggregate::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::AggregateError)?;
        Ok(Some(aggregate))
    }

    async fn find_at_version(
        &self,
        id: Self::AggregateId,
        version: AggregateVersion,
    ) -> Result<Option<Self::Aggregate>, Self::Error> {
        let (events, snapshot) = self
            .event_history_reader()
            .read_at_version(Self::Aggregate::AGGREGATE_TYPE, id, version)
            .await
            .map_err(RepositoryError::EventHistoryReaderError)?;
        let mut aggregate = Self::Aggregate::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::AggregateError)?;
        Ok(Some(aggregate))
    }
}
