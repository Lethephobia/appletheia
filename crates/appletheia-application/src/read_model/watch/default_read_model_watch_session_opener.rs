use crate::read_model::ReadModel;
use crate::read_model::list::ReadModelListMatcher;

use super::{
    ReadModelTypedListWatch, ReadModelWatchDelivery, ReadModelWatchSession,
    ReadModelWatchSessionId, ReadModelWatchSessionOpener, ReadModelWatchSessionRegistry,
};

/// Opens typed read-model watch sessions and registers their runtime delivery state.
#[derive(Clone)]
pub struct DefaultReadModelWatchSessionOpener<G> {
    registry: G,
}

impl<G> DefaultReadModelWatchSessionOpener<G>
where
    G: ReadModelWatchSessionRegistry,
{
    pub fn new(registry: G) -> Self {
        Self { registry }
    }
}

impl<G> ReadModelWatchSessionOpener for DefaultReadModelWatchSessionOpener<G>
where
    G: ReadModelWatchSessionRegistry,
{
    async fn open_snapshot<R, D>(&self, delivery: D) -> ReadModelWatchSessionId
    where
        R: ReadModel + 'static,
        D: ReadModelWatchDelivery,
    {
        let session_id = ReadModelWatchSessionId::new();
        let session = ReadModelWatchSession::snapshot::<R, D>(session_id, delivery);
        self.registry.register(session_id, session).await;
        session_id
    }

    async fn open_list<R, M, D>(
        &self,
        matcher: M,
        watched_list: ReadModelTypedListWatch<M::Query, M::Cursor>,
        delivery: D,
    ) -> ReadModelWatchSessionId
    where
        R: ReadModel + 'static,
        M: ReadModelListMatcher + 'static,
        D: ReadModelWatchDelivery,
    {
        let session_id = ReadModelWatchSessionId::new();
        let session =
            ReadModelWatchSession::list::<R, M, D>(session_id, matcher, watched_list, delivery);
        self.registry.register(session_id, session).await;
        session_id
    }
}
