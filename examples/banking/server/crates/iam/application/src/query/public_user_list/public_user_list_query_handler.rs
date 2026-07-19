use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::projection::PublicUserListProjectorSpec;
use crate::read_model::{PublicUserList, PublicUserListReader};

use super::{PublicUserListQuery, PublicUserListQueryHandlerError};

/// Handles public user list queries.
pub struct PublicUserListQueryHandler<S>
where
    S: PublicUserListReader,
{
    reader: S,
}

impl<S> PublicUserListQueryHandler<S>
where
    S: PublicUserListReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for PublicUserListQueryHandler<S>
where
    S: PublicUserListReader,
{
    type Query = PublicUserListQuery;
    type Output = PublicUserList;
    type Error = PublicUserListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[PublicUserListProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, _query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut S::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self
            .reader
            .list(uow, query.criteria, query.cursor_options, query.limit)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
    use appletheia::application::query::QueryHandler;
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

    use crate::read_model::{
        PublicUserList, PublicUserListCriteria, PublicUserListCursor, PublicUserListReader,
        PublicUserListReaderError, PublicUserListSortKey,
    };

    use super::{PublicUserListQuery, PublicUserListQueryHandler};

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct TestPublicUserListReader;

    impl PublicUserListReader for TestPublicUserListReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _criteria: PublicUserListCriteria,
            _cursor_options: Option<CursorOptions<PublicUserListSortKey, PublicUserListCursor>>,
            _limit: PageSize,
        ) -> Result<PublicUserList, PublicUserListReaderError> {
            Ok(PublicUserList {
                items: Vec::new(),
                next_cursor: None,
            })
        }
    }

    #[test]
    fn authorization_plan_requires_authentication() {
        let handler = PublicUserListQueryHandler::new(TestPublicUserListReader);
        let query = PublicUserListQuery {
            criteria: PublicUserListCriteria::default(),
            cursor_options: None,
            limit: PageSize::new(20).expect("page size should be valid"),
        };

        let plan = handler
            .authorization_plan(&query)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![PrincipalRequirement::Authenticated])
        );
    }
}
