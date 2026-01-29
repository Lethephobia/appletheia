use appletheia_domain::{Aggregate, AggregateVersion, Snapshot};

use super::snapshot_reader_error::SnapshotReaderError;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait SnapshotReader<A: Aggregate>: Send + Sync {
    type Uow: UnitOfWork;

    async fn read_latest_snapshot(
        &self,
        uow: &mut Self::Uow,
        aggregate_id: A::Id,
        as_of: Option<AggregateVersion>,
    ) -> Result<Option<Snapshot<A::State>>, SnapshotReaderError>;
}
