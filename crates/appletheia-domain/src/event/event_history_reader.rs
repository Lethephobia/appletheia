use crate::aggregate::{Aggregate, AggregateVersion};
use crate::snapshot::Snapshot;

use super::{Event, EventHistoryReaderError};

#[allow(async_fn_in_trait)]
pub trait EventHistoryReader<A: Aggregate> {
    async fn read(
        &self,
        aggregate_type: &str,
        aggregate_id: A::Id,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        EventHistoryReaderError,
    >;

    async fn read_at_version(
        &self,
        aggregate_type: &str,
        aggregate_id: A::Id,
        version: AggregateVersion,
    ) -> Result<
        (
            Vec<Event<A::Id, A::EventPayload>>,
            Option<Snapshot<A::State>>,
        ),
        EventHistoryReaderError,
    >;
}
