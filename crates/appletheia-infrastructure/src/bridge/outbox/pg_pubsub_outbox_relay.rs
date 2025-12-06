use std::sync::atomic::{AtomicBool, Ordering};

use appletheia_application::outbox::{
    OutboxFetcherAccess, OutboxPublisherAccess, OutboxRelay, OutboxRelayConfig,
    OutboxRelayConfigAccess, OutboxWriterAccess,
};

use crate::google_cloud::pubsub::PubsubOutboxPublisher;
use crate::postgresql::outbox::{PgOutboxFetcher, PgOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgPubsubOutboxRelay {
    config: OutboxRelayConfig,
    publisher: PubsubOutboxPublisher,
    fetcher: PgOutboxFetcher,
    writer: PgOutboxWriter,
    stop_requested: AtomicBool,
}

impl PgPubsubOutboxRelay {
    pub fn new(config: OutboxRelayConfig, publisher: PubsubOutboxPublisher) -> Self {
        Self {
            config,
            publisher,
            fetcher: PgOutboxFetcher::new(),
            writer: PgOutboxWriter::new(),
            stop_requested: AtomicBool::new(false),
        }
    }

    pub fn with_components(
        config: OutboxRelayConfig,
        publisher: PubsubOutboxPublisher,
        fetcher: PgOutboxFetcher,
        writer: PgOutboxWriter,
    ) -> Self {
        Self {
            config,
            publisher,
            fetcher,
            writer,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl OutboxRelayConfigAccess for PgPubsubOutboxRelay {
    fn config(&self) -> &OutboxRelayConfig {
        &self.config
    }
}

impl OutboxPublisherAccess for PgPubsubOutboxRelay {
    type OutboxPublisher = PubsubOutboxPublisher;

    fn outbox_publisher(&self) -> &Self::OutboxPublisher {
        &self.publisher
    }
}

impl OutboxFetcherAccess for PgPubsubOutboxRelay {
    type Fetcher = PgOutboxFetcher;

    fn outbox_fetcher(&self) -> &Self::Fetcher {
        &self.fetcher
    }
}

impl OutboxWriterAccess for PgPubsubOutboxRelay {
    type Writer = PgOutboxWriter;

    fn outbox_writer(&self) -> &Self::Writer {
        &self.writer
    }
}

impl OutboxRelay for PgPubsubOutboxRelay {
    type Uow = PgUnitOfWork;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }
}
