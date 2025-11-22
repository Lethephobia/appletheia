use crate::aggregate::Aggregate;

use super::SnapshotReader;

pub trait SnapshotReaderProvider<A: Aggregate> {
    type SnapshotReader<'c>: SnapshotReader<A>
    where
        Self: 'c;

    fn snapshot_reader(&mut self) -> Self::SnapshotReader<'_>;
}
