use appletheia_domain::{Aggregate, Snapshot};

use crate::unit_of_work::UnitOfWork;

use super::snapshot_writer_error::SnapshotWriterError;

#[allow(async_fn_in_trait)]
pub trait SnapshotWriter<A: Aggregate>: Send + Sync {
    type Uow: UnitOfWork;

    async fn write_snapshot(
        &self,
        uow: &mut Self::Uow,
        snapshot: &Snapshot<A::State>,
    ) -> Result<(), SnapshotWriterError>;
}
