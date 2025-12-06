use appletheia_domain::{Aggregate, AggregateVersion, Snapshot};

use super::snapshot_reader_error::SnapshotReaderError;

#[allow(async_fn_in_trait)]
pub trait SnapshotReader<A: Aggregate> {
    async fn read_latest_snapshot(
        &mut self,
        aggregate_id: A::Id,
        as_of: Option<AggregateVersion>,
    ) -> Result<Option<Snapshot<A::State>>, SnapshotReaderError>;
}
