use appletheia_application::outbox::{DefaultOutboxRelay, OutboxRelayConfig};
use appletheia_application::outbox::event::EventOutbox;

use crate::google_cloud::pubsub::outbox::event::PubsubEventOutboxPublisher;
use crate::postgresql::outbox::event::{PgEventOutboxFetcher, PgEventOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub type PgPubsubEventOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWork,
    EventOutbox,
    PgEventOutboxFetcher,
    PgEventOutboxWriter,
    PubsubEventOutboxPublisher,
>;

pub fn pg_pubsub_event_outbox_relay(
    config: OutboxRelayConfig,
    publisher: PubsubEventOutboxPublisher,
) -> PgPubsubEventOutboxRelay {
    DefaultOutboxRelay::new(
        config,
        publisher,
        PgEventOutboxFetcher::new(),
        PgEventOutboxWriter::new(),
    )
}

pub fn pg_pubsub_event_outbox_relay_with_components(
    config: OutboxRelayConfig,
    publisher: PubsubEventOutboxPublisher,
    fetcher: PgEventOutboxFetcher,
    writer: PgEventOutboxWriter,
) -> PgPubsubEventOutboxRelay {
    DefaultOutboxRelay::new(config, publisher, fetcher, writer)
}
