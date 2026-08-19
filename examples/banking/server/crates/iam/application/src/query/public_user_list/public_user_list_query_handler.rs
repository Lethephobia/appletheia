use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::projection::UserFragmentProjectorSpec;
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
        ProjectorDependencies::Some(&[UserFragmentProjectorSpec::DESCRIPTOR]);

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
            .list(uow, query.criteria, query.sort, query.page)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
    use appletheia::application::query::QueryHandler;
    use appletheia::application::read_model::pagination::{
        CursorWindow, PageSize, Sort, SortDirection,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};

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
            _sort: Sort<PublicUserListSortKey>,
            _page: CursorWindow<PublicUserListCursor>,
        ) -> Result<PublicUserList, PublicUserListReaderError> {
            Ok(PublicUserList {
                items: Vec::new(),
                start_cursor: None,
                end_cursor: None,
                has_previous: false,
                has_next: false,
            })
        }
    }

    #[test]
    fn authorization_plan_requires_authentication() {
        let handler = PublicUserListQueryHandler::new(TestPublicUserListReader);
        let query = PublicUserListQuery {
            criteria: PublicUserListCriteria::default(),
            sort: Sort {
                key: PublicUserListSortKey::CreatedAt,
                direction: SortDirection::Desc,
            },
            page: CursorWindow::Forward {
                after: None,
                limit: PageSize::new(20).expect("page size should be valid"),
            },
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
