use appletheia_domain::Aggregate;

use super::SnapshotReader;

pub trait SnapshotReaderAccess<A: Aggregate> {
    type Reader: SnapshotReader<A>;

    fn snapshot_reader(&self) -> &Self::Reader;
}
