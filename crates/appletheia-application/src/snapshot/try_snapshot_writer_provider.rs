use appletheia_domain::Aggregate;
use std::error::Error;

use super::SnapshotWriter;

pub trait TrySnapshotWriterProvider<A: Aggregate> {
    type Error: Error + Send + Sync + 'static;
    type SnapshotWriter<'c>: SnapshotWriter<A, Error = Self::Error>
    where
        Self: 'c;

    fn try_snapshot_writer(&mut self) -> Result<Self::SnapshotWriter<'_>, Self::Error>;
}
