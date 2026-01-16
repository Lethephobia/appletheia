use std::sync::atomic::{AtomicBool, Ordering};

use appletheia_application::event::{
    EventOutboxFetcherAccess, EventOutboxPublisherAccess, EventOutboxRelay, EventOutboxRelayConfig,
    EventOutboxRelayConfigAccess, EventOutboxWriterAccess,
};

use crate::google_cloud::pubsub::PubsubEventOutboxPublisher;
use crate::postgresql::event_outbox::{PgEventOutboxFetcher, PgEventOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgPubsubEventOutboxRelay {
    config: EventOutboxRelayConfig,
    publisher: PubsubEventOutboxPublisher,
    fetcher: PgEventOutboxFetcher,
    writer: PgEventOutboxWriter,
    stop_requested: AtomicBool,
}

impl PgPubsubEventOutboxRelay {
    pub fn new(config: EventOutboxRelayConfig, publisher: PubsubEventOutboxPublisher) -> Self {
        Self {
            config,
            publisher,
            fetcher: PgEventOutboxFetcher::new(),
            writer: PgEventOutboxWriter::new(),
            stop_requested: AtomicBool::new(false),
        }
    }

    pub fn with_components(
        config: EventOutboxRelayConfig,
        publisher: PubsubEventOutboxPublisher,
        fetcher: PgEventOutboxFetcher,
        writer: PgEventOutboxWriter,
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

impl EventOutboxRelayConfigAccess for PgPubsubEventOutboxRelay {
    fn config(&self) -> &EventOutboxRelayConfig {
        &self.config
    }
}

impl EventOutboxPublisherAccess for PgPubsubEventOutboxRelay {
    type EventOutboxPublisher = PubsubEventOutboxPublisher;

    fn outbox_publisher(&self) -> &Self::EventOutboxPublisher {
        &self.publisher
    }
}

impl EventOutboxFetcherAccess for PgPubsubEventOutboxRelay {
    type Fetcher = PgEventOutboxFetcher;

    fn outbox_fetcher(&self) -> &Self::Fetcher {
        &self.fetcher
    }
}

impl EventOutboxWriterAccess for PgPubsubEventOutboxRelay {
    type Writer = PgEventOutboxWriter;

    fn outbox_writer(&self) -> &Self::Writer {
        &self.writer
    }
}

impl EventOutboxRelay for PgPubsubEventOutboxRelay {
    type Uow = PgUnitOfWork;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }
}
