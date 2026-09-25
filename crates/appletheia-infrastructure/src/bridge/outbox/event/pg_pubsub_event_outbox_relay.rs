use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::event::EventOutbox;

use crate::google_cloud::pubsub::messaging::PubsubPublisher;
use crate::postgresql::outbox::event::{PgEventOutboxFetcher, PgEventOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;

pub type PgPubsubEventOutboxRelay<C> = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    EventOutbox,
    PgEventOutboxFetcher,
    PgEventOutboxWriter,
    PubsubPublisher<C>,
>;
