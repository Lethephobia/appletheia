use crate::authorization::{Authorizer, RelationshipRequirement};
use crate::projection::ReadYourWritesWaiter;
use crate::request_context::RequestContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{QueryConsistency, QueryDispatchError, QueryDispatcher, QueryHandler, QueryOptions};

pub struct DefaultQueryDispatcher<W, U, AZ>
where
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
{
    read_your_writes_waiter: W,
    uow_factory: U,
    authorizer: AZ,
}

impl<W, U, AZ> DefaultQueryDispatcher<W, U, AZ>
where
    W: ReadYourWritesWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
{
    pub fn new(read_your_writes_waiter: W, uow_factory: U, authorizer: AZ) -> Self {
        Self {
            read_your_writes_waiter,
            uow_factory,
            authorizer,
        }
    }
}

impl<W, U, AZ> QueryDispatcher for DefaultQueryDispatcher<W, U, AZ>
where
    W: ReadYourWritesWaiter,
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
        options: QueryOptions,
    ) -> Result<H::Output, QueryDispatchError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>,
    {
        let authorization_plan = handler.authorization_plan(&query);
        let requirement = authorization_plan.requirement;
        let authorization_dependencies = authorization_plan.dependencies;
        match options.consistency {
            QueryConsistency::Eventual => {}
            QueryConsistency::ReadYourWrites {
                after,
                timeout,
                poll_interval,
            } => {
                if !matches!(requirement, RelationshipRequirement::None) {
                    let projectors = authorization_dependencies.owned_names();
                    self.read_your_writes_waiter
                        .wait(after, timeout, poll_interval, &projectors)
                        .await?;
                }
            }
        }

        self.authorizer
            .authorize(&request_context.principal, &requirement)
            .await?;

        match options.consistency {
            QueryConsistency::Eventual => {}
            QueryConsistency::ReadYourWrites {
                after,
                timeout,
                poll_interval,
            } => {
                let projectors = H::DEPENDENCIES.owned_names();
                self.read_your_writes_waiter
                    .wait(after, timeout, poll_interval, &projectors)
                    .await?;
            }
        }

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
                    .map_err(QueryDispatchError::UnitOfWork)?;
                Err(QueryDispatchError::Handler(operation_error))
            }
        }
    }
}
