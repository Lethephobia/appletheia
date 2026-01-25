use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::event::EventOutbox;

use crate::google_cloud::pubsub::outbox::event::PubsubEventOutboxPublisher;
use crate::postgresql::outbox::event::{PgEventOutboxFetcher, PgEventOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;

pub type PgPubsubEventOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    EventOutbox,
    PgEventOutboxFetcher,
    PgEventOutboxWriter,
    PubsubEventOutboxPublisher,
>;
