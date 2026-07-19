use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use crate::authorization::OrganizationMemberRelation;
use crate::projection::OrganizationMemberListProjectorSpec;
use crate::read_model::{OrganizationMemberList, OrganizationMemberListReader};

use super::{OrganizationMemberListQuery, OrganizationMemberListQueryHandlerError};

/// Handles organization member list queries.
pub struct OrganizationMemberListQueryHandler<R>
where
    R: OrganizationMemberListReader,
{
    reader: R,
}

impl<R> OrganizationMemberListQueryHandler<R>
where
    R: OrganizationMemberListReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for OrganizationMemberListQueryHandler<R>
where
    R: OrganizationMemberListReader,
{
    type Query = OrganizationMemberListQuery;
    type Output = OrganizationMemberList;
    type Error = OrganizationMemberListQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[OrganizationMemberListProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                query.organization_id,
                OrganizationMemberRelation::REF,
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
                query.cursor_options,
                query.limit,
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
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_iam_domain::{Organization, OrganizationId};
    use banking_shared_kernel_application::read_model::{CursorOptions, PageSize};

    use crate::authorization::OrganizationMemberRelation;
    use crate::read_model::{
        OrganizationMemberList, OrganizationMemberListCriteria, OrganizationMemberListCursor,
        OrganizationMemberListReader, OrganizationMemberListReaderError,
        OrganizationMemberListSortKey,
    };

    use super::{OrganizationMemberListQuery, OrganizationMemberListQueryHandler};

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

    impl OrganizationMemberListReader for TestReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _organization_id: OrganizationId,
            _criteria: OrganizationMemberListCriteria,
            _cursor_options: Option<
                CursorOptions<OrganizationMemberListSortKey, OrganizationMemberListCursor>,
            >,
            _limit: PageSize,
        ) -> Result<OrganizationMemberList, OrganizationMemberListReaderError> {
            panic!("reader is not exercised by this test")
        }
    }

    #[test]
    fn authorization_plan_requires_organization_membership() {
        let handler = OrganizationMemberListQueryHandler::new(TestReader);
        let organization_id = OrganizationId::new();
        let query = OrganizationMemberListQuery {
            organization_id,
            criteria: OrganizationMemberListCriteria::default(),
            cursor_options: None,
            limit: PageSize::new(20).expect("page size should be valid"),
        };

        let plan = handler
            .authorization_plan(&query)
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationMemberRelation::REF
                    )
                ),
            ])
        );
    }
}
