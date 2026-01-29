use appletheia_application::repository::DefaultRepository;

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
