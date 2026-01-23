use appletheia_application::outbox::event::EventOutbox;
use appletheia_application::outbox::{DefaultOutboxRelay, OutboxRelayConfig};
use appletheia_domain::AggregateType;

use crate::google_cloud::pubsub::outbox::event::PubsubEventOutboxPublisher;
use crate::postgresql::outbox::event::{PgEventOutboxFetcher, PgEventOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub type PgPubsubEventOutboxRelay<AT> = DefaultOutboxRelay<
    PgUnitOfWork,
    EventOutbox<AT>,
    PgEventOutboxFetcher<AT>,
    PgEventOutboxWriter<AT>,
    PubsubEventOutboxPublisher<AT>,
>;

pub fn pg_pubsub_event_outbox_relay<AT: AggregateType>(
    config: OutboxRelayConfig,
    publisher: PubsubEventOutboxPublisher<AT>,
) -> PgPubsubEventOutboxRelay<AT> {
    DefaultOutboxRelay::new(
        config,
        publisher,
        PgEventOutboxFetcher::<AT>::new(),
        PgEventOutboxWriter::<AT>::new(),
    )
}

pub fn pg_pubsub_event_outbox_relay_with_components<AT: AggregateType>(
    config: OutboxRelayConfig,
    publisher: PubsubEventOutboxPublisher<AT>,
    fetcher: PgEventOutboxFetcher<AT>,
    writer: PgEventOutboxWriter<AT>,
) -> PgPubsubEventOutboxRelay<AT> {
    DefaultOutboxRelay::new(config, publisher, fetcher, writer)
}
