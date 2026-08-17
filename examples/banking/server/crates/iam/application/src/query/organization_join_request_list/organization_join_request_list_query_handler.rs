use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use crate::authorization::OrganizationAdminRelation;
use crate::projection::{
    OrganizationFragmentProjectorSpec, OrganizationJoinRequestFragmentProjectorSpec,
    UserFragmentProjectorSpec,
};
use crate::read_model::{OrganizationJoinRequestList, OrganizationJoinRequestListReader};

use super::{OrganizationJoinRequestListQuery, OrganizationJoinRequestListQueryHandlerError};

/// Handles organization join request list queries.
pub struct OrganizationJoinRequestListQueryHandler<R>
where
    R: OrganizationJoinRequestListReader,
{
    reader: R,
}

impl<R> OrganizationJoinRequestListQueryHandler<R>
where
    R: OrganizationJoinRequestListReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for OrganizationJoinRequestListQueryHandler<R>
where
    R: OrganizationJoinRequestListReader,
{
    type Query = OrganizationJoinRequestListQuery;
    type Output = OrganizationJoinRequestList;
    type Error = OrganizationJoinRequestListQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> = ProjectorDependencies::Some(&[
        OrganizationJoinRequestFragmentProjectorSpec::DESCRIPTOR,
        OrganizationFragmentProjectorSpec::DESCRIPTOR,
        UserFragmentProjectorSpec::DESCRIPTOR,
    ]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                query.organization_id,
                OrganizationAdminRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        query: Self::Query,
    ) -> Result<Self::Output, Self::Error> {
        Ok(self
            .reader
            .list(
                uow,
                query.organization_id,
                query.criteria,
                query.sort,
                query.page,
            )
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::query::QueryHandler;
    use appletheia::application::read_model::pagination::{
        CursorPage, PageSize, Sort, SortDirection,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_iam_domain::{Organization, OrganizationId};

    use crate::authorization::OrganizationAdminRelation;
    use crate::read_model::{
        OrganizationJoinRequestList, OrganizationJoinRequestListCriteria,
        OrganizationJoinRequestListCursor, OrganizationJoinRequestListReader,
        OrganizationJoinRequestListReaderError, OrganizationJoinRequestListSortKey,
    };

    use super::{OrganizationJoinRequestListQuery, OrganizationJoinRequestListQueryHandler};

    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    struct TestReader;

    impl OrganizationJoinRequestListReader for TestReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _scope_id: OrganizationId,
            _criteria: OrganizationJoinRequestListCriteria,
            _sort: Sort<OrganizationJoinRequestListSortKey>,
            _page: CursorPage<OrganizationJoinRequestListCursor>,
        ) -> Result<OrganizationJoinRequestList, OrganizationJoinRequestListReaderError> {
            panic!("reader is not exercised by this test")
        }
    }

    #[test]
    fn authorization_plan_requires_expected_relationship() {
        let handler = OrganizationJoinRequestListQueryHandler::new(TestReader);
        let scope_id = OrganizationId::new();
        let query = OrganizationJoinRequestListQuery {
            organization_id: scope_id,
            criteria: OrganizationJoinRequestListCriteria::default(),
            sort: Sort {
                key: OrganizationJoinRequestListSortKey::CreatedAt,
                direction: SortDirection::Desc,
            },
            page: CursorPage {
                after: None,
                limit: PageSize::new(20).expect("page size should be valid"),
            },
        };

        let plan = handler
            .authorization_plan(&query)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        scope_id,
                        OrganizationAdminRelation::REF
                    )
                ),
            ])
        );
    }
}
