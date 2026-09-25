use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;

use crate::projection::OrganizationFragmentProjectorSpec;
use crate::read_model::{PublicOrganizationList, PublicOrganizationListReader};

use super::{PublicOrganizationListQuery, PublicOrganizationListQueryHandlerError};

/// Handles public organization list queries.
pub struct PublicOrganizationListQueryHandler<S>
where
    S: PublicOrganizationListReader,
{
    reader: S,
}

impl<S> PublicOrganizationListQueryHandler<S>
where
    S: PublicOrganizationListReader,
{
    pub fn new(reader: S) -> Self {
        Self { reader }
    }
}

impl<S> QueryHandler for PublicOrganizationListQueryHandler<S>
where
    S: PublicOrganizationListReader,
{
    type Query = PublicOrganizationListQuery;
    type Output = PublicOrganizationList;
    type Error = PublicOrganizationListQueryHandlerError;
    type Uow = S::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[OrganizationFragmentProjectorSpec::DESCRIPTOR]);

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
        PublicOrganizationList, PublicOrganizationListCriteria, PublicOrganizationListCursor,
        PublicOrganizationListReader, PublicOrganizationListReaderError,
        PublicOrganizationListSortKey,
    };

    use super::{PublicOrganizationListQuery, PublicOrganizationListQueryHandler};

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
    struct TestPublicOrganizationListReader;

    impl PublicOrganizationListReader for TestPublicOrganizationListReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _criteria: PublicOrganizationListCriteria,
            _sort: Sort<PublicOrganizationListSortKey>,
            _page: CursorWindow<PublicOrganizationListCursor>,
        ) -> Result<PublicOrganizationList, PublicOrganizationListReaderError> {
            Ok(PublicOrganizationList {
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
        let handler = PublicOrganizationListQueryHandler::new(TestPublicOrganizationListReader);
        let query = PublicOrganizationListQuery {
            criteria: PublicOrganizationListCriteria::default(),
            sort: Sort {
                key: PublicOrganizationListSortKey::CreatedAt,
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
