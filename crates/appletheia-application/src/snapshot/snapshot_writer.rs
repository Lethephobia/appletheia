use appletheia_domain::{Aggregate, Snapshot};
use std::error::Error;

#[allow(async_fn_in_trait)]
pub trait SnapshotWriter<A: Aggregate> {
    type Error: Error + Send + Sync + 'static;

    async fn write_snapshot(&mut self, snapshot: &Snapshot<A::State>) -> Result<(), Self::Error>;
}
