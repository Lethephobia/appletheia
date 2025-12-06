use appletheia_domain::Aggregate;

use super::SnapshotWriter;

pub trait SnapshotWriterAccess<A: Aggregate> {
    type Writer: SnapshotWriter<A>;

    fn snapshot_writer(&self) -> &Self::Writer;
}
