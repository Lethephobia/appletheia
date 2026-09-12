use appletheia_application::repository::DefaultRepository;

use crate::postgresql::PgRelationshipStore;

use crate::postgresql::event::{PgEventReader, PgEventWriter};
use crate::postgresql::outbox::event::PgEventOutboxEnqueuer;
use crate::postgresql::repository::{
    PgReferenceIndexStore, PgUniqueKeyReservationStore, PgUniqueValueOwnerLookup,
};
use crate::postgresql::snapshot::{PgSnapshotReader, PgSnapshotWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub type PgRepository<A, RD> = DefaultRepository<
    A,
    PgEventReader<A>,
    PgEventWriter<A>,
    PgEventOutboxEnqueuer,
    PgSnapshotReader<A>,
    PgSnapshotWriter<A>,
    PgUniqueValueOwnerLookup,
    PgUniqueKeyReservationStore,
    PgReferenceIndexStore,
    PgRelationshipStore,
    RD,
    PgUnitOfWork,
>;
