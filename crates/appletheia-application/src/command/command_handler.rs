use std::error::Error;

use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait CommandHandler: Send + Sync {
    type Command: Send + 'static;
    type Output: Send + 'static;
    type Error: Error + Send + Sync + 'static;
    type Uow: UnitOfWork + Send;

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<Self::Output, Self::Error>;
}
