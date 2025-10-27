use crate::aggregate::{AggregateId, AggregateState, AggregateVersion};
use crate::snapshot::Snapshot;

use super::{Event, EventHistoryReaderError, EventPayload};

#[allow(async_fn_in_trait)]
pub trait EventHistoryReader<A: AggregateId, P: EventPayload, S: AggregateState<Id = A>> {
    async fn read(
        &self,
        aggregate_type: &str,
        aggregate_id: A,
    ) -> Result<(Vec<Event<A, P>>, Option<Snapshot<S>>), EventHistoryReaderError>;

    async fn read_at_version(
        &self,
        aggregate_type: &str,
        aggregate_id: A,
        version: AggregateVersion,
    ) -> Result<(Vec<Event<A, P>>, Option<Snapshot<S>>), EventHistoryReaderError>;
}
