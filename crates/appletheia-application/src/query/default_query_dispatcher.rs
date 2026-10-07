use crate::authorization::Authorizer;
use crate::request_context::RequestContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{QueryDispatcher, QueryDispatcherError, QueryHandler};

pub struct DefaultQueryDispatcher<U, AZ>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
{
    uow_factory: U,
    authorizer: AZ,
}

impl<U, AZ> DefaultQueryDispatcher<U, AZ>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
{
    pub fn new(uow_factory: U, authorizer: AZ) -> Self {
        Self {
            uow_factory,
            authorizer,
        }
    }
}

impl<U, AZ> QueryDispatcher for DefaultQueryDispatcher<U, AZ>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
{
    type Uow = U::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
    ) -> Result<H::Output, QueryDispatcherError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>,
    {
        let authorization_plan = handler
            .authorization_plan(&query)
            .map_err(QueryDispatcherError::Handler)?;
        self.authorizer
            .authorize(&request_context.principal, &authorization_plan)
            .await?;

        let mut uow = self.uow_factory.begin().await?;

        let result = handler.handle(&mut uow, request_context, query).await;
        match result {
            Ok(output) => {
                uow.commit().await?;
                Ok(output)
            }
            Err(operation_error) => {
                let operation_error = uow
                    .rollback_with_operation_error(operation_error)
                    .await
                    .map_err(QueryDispatcherError::UnitOfWork)?;
                Err(QueryDispatcherError::Handler(operation_error))
            }
        }
    }
}
