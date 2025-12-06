use std::error::Error;

use appletheia_domain::Aggregate;

use super::SnapshotReader;

pub trait TrySnapshotReaderProvider<A: Aggregate> {
    type Error: Error + Send + Sync + 'static;
    type SnapshotReader<'c>: SnapshotReader<A>
    where
        Self: 'c;

    fn try_snapshot_reader(&mut self) -> Result<Self::SnapshotReader<'_>, Self::Error>;
}
