use std::error::Error;

#[allow(async_fn_in_trait)]
pub trait EventStoreMigrator {
    type Error: Error + Send + Sync + 'static;

    async fn run(&self) -> Result<(), Self::Error>;
}
