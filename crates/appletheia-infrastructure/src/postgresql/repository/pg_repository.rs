use appletheia_application::repository::{DefaultRepository, RepositoryConfig};
use appletheia_domain::Aggregate;

use crate::postgresql::event::{PgEventReader, PgEventWriter};
use crate::postgresql::snapshot::{PgSnapshotReader, PgSnapshotWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub type PgRepository<A> = DefaultRepository<
    A,
    PgEventReader<A>,
    PgEventWriter<A>,
    PgSnapshotReader<A>,
    PgSnapshotWriter<A>,
    PgUnitOfWork,
>;

pub fn pg_repository<A: Aggregate>(config: RepositoryConfig) -> PgRepository<A> {
    DefaultRepository::new(
        config,
        PgEventReader::new(),
        PgSnapshotReader::new(),
        PgEventWriter::new(),
        PgSnapshotWriter::new(),
    )
}

pub fn pg_repository_with_components<A: Aggregate>(
    config: RepositoryConfig,
    event_reader: PgEventReader<A>,
    snapshot_reader: PgSnapshotReader<A>,
    event_writer: PgEventWriter<A>,
    snapshot_writer: PgSnapshotWriter<A>,
) -> PgRepository<A> {
    DefaultRepository::new(
        config,
        event_reader,
        snapshot_reader,
        event_writer,
        snapshot_writer,
    )
}
