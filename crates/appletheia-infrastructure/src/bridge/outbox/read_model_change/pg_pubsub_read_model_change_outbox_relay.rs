use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::read_model_change::ReadModelChangeOutbox;

use crate::google_cloud::pubsub::messaging::PubsubPublisher;
use crate::postgresql::outbox::read_model_change::{
    PgReadModelChangeOutboxFetcher, PgReadModelChangeOutboxWriter,
};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;

pub type PgPubsubReadModelChangeOutboxRelay<C> = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    ReadModelChangeOutbox,
    PgReadModelChangeOutboxFetcher,
    PgReadModelChangeOutboxWriter,
    PubsubPublisher<C>,
>;
