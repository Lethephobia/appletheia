use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::time::Duration as StdDuration;

use chrono::Duration;
use tokio::time::sleep;

use crate::massaging::{PublishResult, Publisher, Topic};
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::{
    Outbox, OutboxFetcher, OutboxRelay, OutboxRelayConfig, OutboxRelayError, OutboxRelayRunReport,
    OutboxState, OutboxWriter,
};

pub struct DefaultOutboxRelay<UowFactory, O, F, W, T>
where
    UowFactory: UnitOfWorkFactory,
    O: Outbox,
    F: OutboxFetcher<Uow = UowFactory::Uow, Outbox = O>,
    W: OutboxWriter<Uow = UowFactory::Uow, Outbox = O>,
    T: Topic<O::Message> + Sync,
{
    config: OutboxRelayConfig,
    topic: T,
    fetcher: F,
    writer: W,
    uow_factory: UowFactory,
    stop_requested: AtomicBool,
    _marker: PhantomData<fn() -> O>,
}

impl<UowFactory, O, F, W, T> DefaultOutboxRelay<UowFactory, O, F, W, T>
where
    UowFactory: UnitOfWorkFactory,
    O: Outbox,
    F: OutboxFetcher<Uow = UowFactory::Uow, Outbox = O>,
    W: OutboxWriter<Uow = UowFactory::Uow, Outbox = O>,
    T: Topic<O::Message> + Sync,
{
    pub fn new(
        config: OutboxRelayConfig,
        topic: T,
        fetcher: F,
        writer: W,
        uow_factory: UowFactory,
    ) -> Self {
        Self {
            config,
            topic,
            fetcher,
            writer,
            uow_factory,
            stop_requested: AtomicBool::new(false),
            _marker: PhantomData,
        }
    }
}

impl<UowFactory, O, F, W, T> OutboxRelay for DefaultOutboxRelay<UowFactory, O, F, W, T>
where
    UowFactory: UnitOfWorkFactory,
    O: Outbox,
    F: OutboxFetcher<Uow = UowFactory::Uow, Outbox = O>,
    W: OutboxWriter<Uow = UowFactory::Uow, Outbox = O>,
    T: Topic<O::Message> + Sync,
{
    type Outbox = O;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&self) -> Result<(), OutboxRelayError> {
        let polling_options = &self.config.polling_options;
        let mut poll_interval = polling_options.base;

        while !self.is_stop_requested() {
            let run_report = self.run_once().await?;

            match run_report {
                OutboxRelayRunReport::Progress { .. } => {
                    poll_interval = polling_options.base;
                }
                OutboxRelayRunReport::Idle | OutboxRelayRunReport::Throttled => {
                    let duration: Duration = poll_interval.into();
                    let sleep_duration = duration
                        .to_std()
                        .unwrap_or_else(|_| StdDuration::from_secs(0));

                    if sleep_duration > StdDuration::from_secs(0) {
                        sleep(sleep_duration).await;
                    }

                    poll_interval = poll_interval.next(
                        polling_options.multiplier,
                        polling_options.jitter,
                        polling_options.max,
                    );
                }
            }
        }

        Ok(())
    }

    async fn run_once(&self) -> Result<OutboxRelayRunReport, OutboxRelayError> {
        let relay_instance = &self.config.instance;
        let lease_duration = self.config.lease_duration;
        let batch_size = self.config.batch_size;
        let retry_options = self.config.retry_options;

        let mut uow = self.uow_factory.begin().await?;
        let outboxes = self.fetcher.fetch(&mut uow, batch_size).await;
        let mut outboxes = match outboxes {
            Ok(mut outboxes) => {
                if outboxes.is_empty() {
                    uow.commit().await?;
                    return Ok(OutboxRelayRunReport::Idle);
                }

                for outbox in &mut outboxes {
                    match outbox.state() {
                        OutboxState::Pending { .. } => {
                            outbox.acquire_lease(relay_instance, lease_duration)?;
                        }
                        other => {
                            return Err(uow
                                .rollback_with_operation_error(
                                    OutboxRelayError::NonPendingOutboxState(other.clone()),
                                )
                                .await?);
                        }
                    }
                }

                if let Err(operation_error) = self.writer.write_outbox(&mut uow, &outboxes).await {
                    return Err(uow
                        .rollback_with_operation_error(OutboxRelayError::Writer(operation_error))
                        .await?);
                }

                uow.commit().await?;
                outboxes
            }
            Err(operation_error) => {
                return Err(uow
                    .rollback_with_operation_error(OutboxRelayError::Fetcher(operation_error))
                    .await?);
            }
        };

        let publish_results = self
            .topic
            .publisher()
            .publish(outboxes.iter().map(Outbox::message))
            .await?;

        for publish_result in publish_results {
            match publish_result {
                PublishResult::Success { input_index, .. } => {
                    outboxes[input_index].ack()?;
                }
                PublishResult::Failed { input_index, cause } => {
                    outboxes[input_index].nack(&cause, &retry_options)?;
                }
            }
        }

        let proceeded_outbox_count = outboxes.len().min(u32::MAX as usize) as u32;

        let mut uow = self.uow_factory.begin().await?;
        let write_result = self.writer.write_outbox(&mut uow, &outboxes).await;
        match write_result {
            Ok(()) => {
                uow.commit().await?;
            }
            Err(operation_error) => {
                return Err(uow
                    .rollback_with_operation_error(OutboxRelayError::Writer(operation_error))
                    .await?);
            }
        }

        Ok(OutboxRelayRunReport::Progress {
            proceeded_outbox_count,
        })
    }
}
