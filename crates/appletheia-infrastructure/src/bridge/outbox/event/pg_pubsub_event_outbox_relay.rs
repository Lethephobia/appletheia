use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::event::EventOutbox;

use crate::google_cloud::pubsub::messaging::PubsubCloudEventPublisher;
use crate::postgresql::outbox::event::{PgEventOutboxFetcher, PgEventOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;
use appletheia_application::messaging::EventPublisher;

pub type PgPubsubEventOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    EventOutbox,
    PgEventOutboxFetcher,
    PgEventOutboxWriter,
    EventPublisher<PubsubCloudEventPublisher>,
>;
