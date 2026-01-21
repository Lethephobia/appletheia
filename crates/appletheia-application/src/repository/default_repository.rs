use std::marker::PhantomData;
use std::ops::Bound;

use appletheia_domain::{Aggregate, AggregateError, AggregateVersion, AggregateVersionRange};

use crate::event::{EventReader, EventReaderAccess, EventWriter, EventWriterAccess};
use crate::request_context::RequestContext;
use crate::snapshot::{SnapshotReader, SnapshotReaderAccess, SnapshotWriter, SnapshotWriterAccess};
use crate::unit_of_work::{SnapshotPolicy, UnitOfWork};

use super::{Repository, RepositoryConfig, RepositoryConfigAccess, RepositoryError};

pub struct DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    config: RepositoryConfig,
    event_reader: ER,
    snapshot_reader: SR,
    event_writer: EW,
    snapshot_writer: SW,
    _marker: PhantomData<fn() -> A>,
}

impl<A, ER, EW, SR, SW, Uow> DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    pub fn new(
        config: RepositoryConfig,
        event_reader: ER,
        snapshot_reader: SR,
        event_writer: EW,
        snapshot_writer: SW,
    ) -> Self {
        Self {
            config,
            event_reader,
            snapshot_reader,
            event_writer,
            snapshot_writer,
            _marker: PhantomData,
        }
    }
}

impl<A, ER, EW, SR, SW, Uow> RepositoryConfigAccess for DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    fn config(&self) -> &RepositoryConfig {
        &self.config
    }
}

impl<A, ER, EW, SR, SW, Uow> EventReaderAccess<A> for DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    type Reader = ER;

    fn event_reader(&self) -> &Self::Reader {
        &self.event_reader
    }
}

impl<A, ER, EW, SR, SW, Uow> SnapshotReaderAccess<A> for DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    type Reader = SR;

    fn snapshot_reader(&self) -> &Self::Reader {
        &self.snapshot_reader
    }
}

impl<A, ER, EW, SR, SW, Uow> EventWriterAccess<A> for DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    type Writer = EW;

    fn event_writer(&self) -> &Self::Writer {
        &self.event_writer
    }
}

impl<A, ER, EW, SR, SW, Uow> SnapshotWriterAccess<A> for DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    type Writer = SW;

    fn snapshot_writer(&self) -> &Self::Writer {
        &self.snapshot_writer
    }
}

impl<A, ER, EW, SR, SW, Uow> Repository<A> for DefaultRepository<A, ER, EW, SR, SW, Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
    ER: EventReader<A, Uow = Uow>,
    EW: EventWriter<A, Uow = Uow>,
    SR: SnapshotReader<A, Uow = Uow>,
    SW: SnapshotWriter<A, Uow = Uow>,
{
    type Uow = Uow;

    async fn find(&self, uow: &mut Self::Uow, id: A::Id) -> Result<Option<A>, RepositoryError<A>> {
        self.find_at_version(uow, id, None).await
    }

    async fn find_at_version(
        &self,
        uow: &mut Self::Uow,
        id: A::Id,
        at: Option<AggregateVersion>,
    ) -> Result<Option<A>, RepositoryError<A>> {
        let snapshot = self
            .snapshot_reader
            .read_latest_snapshot(uow, id, at)
            .await?;
        let events = {
            let start = snapshot
                .as_ref()
                .map(|s| Bound::Excluded(s.aggregate_version()))
                .unwrap_or(Bound::Unbounded);
            let end = at.map(Bound::Included).unwrap_or(Bound::Unbounded);
            let range = AggregateVersionRange::new(start, end);
            self.event_reader.read_events(uow, id, range).await?
        };

        if events.is_empty() && snapshot.is_none() {
            return Ok(None);
        }

        let mut aggregate = A::default();
        aggregate
            .replay_events(events, snapshot)
            .map_err(RepositoryError::Aggregate)?;

        Ok(Some(aggregate))
    }

    async fn save(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        aggregate: &mut A,
    ) -> Result<(), RepositoryError<A>> {
        let events = aggregate.uncommitted_events();
        self.event_writer
            .write_events_and_outbox(uow, request_context, events)
            .await?;

        match self.config.snapshot_policy {
            SnapshotPolicy::Disabled => {}
            SnapshotPolicy::AtLeast { minimum_interval } => {
                let aggregate_id = aggregate.aggregate_id().ok_or_else(|| {
                    let err: AggregateError<A::Id> = AggregateError::NoState;
                    RepositoryError::Aggregate(err.into())
                })?;

                let current_version = aggregate.version().as_u64();
                let latest_snapshot_version = self
                    .snapshot_reader
                    .read_latest_snapshot(uow, aggregate_id, None)
                    .await?
                    .as_ref()
                    .map(|snapshot| snapshot.aggregate_version().as_u64())
                    .unwrap_or(0);

                if current_version.saturating_sub(latest_snapshot_version)
                    >= minimum_interval.as_u64()
                {
                    let snapshot = aggregate
                        .to_snapshot()
                        .map_err(RepositoryError::Aggregate)?;
                    self.snapshot_writer.write_snapshot(uow, &snapshot).await?;
                }
            }
        }

        aggregate.clear_uncommitted_events();
        Ok(())
    }
}
