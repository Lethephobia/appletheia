use appletheia_application::repository::{DefaultRepository, NoopEventSaveHook};

use crate::postgresql::event::{PgEventReader, PgEventWriter};
use crate::postgresql::repository::{
    PgReferenceIndexStore, PgUniqueKeyReservationStore, PgUniqueValueOwnerLookup,
};
use crate::postgresql::snapshot::{PgSnapshotReader, PgSnapshotWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub type PgRepository<A, ESH = NoopEventSaveHook<PgUnitOfWork>> = DefaultRepository<
    A,
    PgEventReader<A>,
    PgEventWriter<A>,
    PgSnapshotReader<A>,
    PgSnapshotWriter<A>,
    PgUniqueValueOwnerLookup,
    PgUniqueKeyReservationStore,
    PgReferenceIndexStore,
    ESH,
    PgUnitOfWork,
>;
