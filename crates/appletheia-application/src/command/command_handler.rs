use std::error::Error;

use crate::command::Command;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[allow(async_fn_in_trait)]
pub trait CommandHandler: Send + Sync {
    type Command: Command;
    type Output: Serialize + DeserializeOwned + Send + 'static;
    type Error: Error + Send + Sync + 'static;
    type Uow: UnitOfWork;

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<Self::Output, Self::Error>;
}
