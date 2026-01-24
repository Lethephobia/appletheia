use appletheia_application::outbox::event::EventOutbox;
use appletheia_application::outbox::DefaultOutboxRelay;

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
