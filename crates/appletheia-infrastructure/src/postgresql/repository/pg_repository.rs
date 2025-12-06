use std::marker::PhantomData;

use sqlx::{Postgres, Transaction};

use appletheia_application::event::EventReaderProvider;
use appletheia_application::repository::Repository;
use appletheia_application::snapshot::SnapshotReaderProvider;
use appletheia_domain::Aggregate;

use crate::postgresql::event::PgEventReader;
use crate::postgresql::snapshot::PgSnapshotReader;

pub struct PgRepository<'c, A: Aggregate> {
    transaction: &'c mut Transaction<'static, Postgres>,
    _phantom: PhantomData<A>,
}

impl<'c, A: Aggregate> PgRepository<'c, A> {
    pub fn new(transaction: &'c mut Transaction<'static, Postgres>) -> Self {
        Self {
            transaction,
            _phantom: PhantomData,
        }
    }
}

impl<'c, A: Aggregate> EventReaderProvider<A> for PgRepository<'c, A> {
    type EventReader<'r>
        = PgEventReader<'r, A>
    where
        Self: 'r;

    fn event_reader(&mut self) -> Self::EventReader<'_> {
        PgEventReader::new(self.transaction)
    }
}

impl<'c, A: Aggregate> SnapshotReaderProvider<A> for PgRepository<'c, A> {
    type SnapshotReader<'r>
        = PgSnapshotReader<'r, A>
    where
        Self: 'r;

    fn snapshot_reader(&mut self) -> Self::SnapshotReader<'_> {
        PgSnapshotReader::new(self.transaction)
    }
}

impl<'c, A: Aggregate> Repository<A> for PgRepository<'c, A> {}
