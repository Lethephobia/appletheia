use appletheia_application::event::{EventReaderAccess, EventWriterAccess};
use appletheia_application::repository::{Repository, RepositoryConfig, RepositoryConfigAccess};
use appletheia_application::snapshot::{SnapshotReaderAccess, SnapshotWriterAccess};
use appletheia_domain::Aggregate;

use crate::postgresql::event::{PgEventReader, PgEventWriter};
use crate::postgresql::snapshot::{PgSnapshotReader, PgSnapshotWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgRepository<A: Aggregate> {
    event_reader: PgEventReader<A>,
    snapshot_reader: PgSnapshotReader<A>,
    event_writer: PgEventWriter<A>,
    snapshot_writer: PgSnapshotWriter<A>,
    config: RepositoryConfig,
}

impl<A: Aggregate> PgRepository<A> {
    pub fn new(config: RepositoryConfig) -> Self {
        Self {
            event_reader: PgEventReader::new(),
            snapshot_reader: PgSnapshotReader::new(),
            event_writer: PgEventWriter::new(),
            snapshot_writer: PgSnapshotWriter::new(),
            config,
        }
    }
}

impl<A: Aggregate> RepositoryConfigAccess for PgRepository<A> {
    fn config(&self) -> &RepositoryConfig {
        &self.config
    }
}

impl<A: Aggregate> EventReaderAccess<A> for PgRepository<A> {
    type Reader = PgEventReader<A>;

    fn event_reader(&self) -> &Self::Reader {
        &self.event_reader
    }
}

impl<A: Aggregate> SnapshotReaderAccess<A> for PgRepository<A> {
    type Reader = PgSnapshotReader<A>;

    fn snapshot_reader(&self) -> &Self::Reader {
        &self.snapshot_reader
    }
}

impl<A: Aggregate> EventWriterAccess<A> for PgRepository<A> {
    type Writer = PgEventWriter<A>;

    fn event_writer(&self) -> &Self::Writer {
        &self.event_writer
    }
}

impl<A: Aggregate> SnapshotWriterAccess<A> for PgRepository<A> {
    type Writer = PgSnapshotWriter<A>;

    fn snapshot_writer(&self) -> &Self::Writer {
        &self.snapshot_writer
    }
}

impl<A: Aggregate> Repository<A> for PgRepository<A> {
    type Uow = PgUnitOfWork;
}
