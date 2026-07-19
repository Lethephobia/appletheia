use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use crate::authorization::OrganizationInviterRelation;
use crate::projection::OrganizationInvitationListProjectorSpec;
use crate::read_model::{OrganizationInvitationList, OrganizationInvitationListReader};

use super::{OrganizationInvitationListQuery, OrganizationInvitationListQueryHandlerError};

/// Handles organization invitation list queries.
pub struct OrganizationInvitationListQueryHandler<R>
where
    R: OrganizationInvitationListReader,
{
    reader: R,
}

impl<R> OrganizationInvitationListQueryHandler<R>
where
    R: OrganizationInvitationListReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for OrganizationInvitationListQueryHandler<R>
where
    R: OrganizationInvitationListReader,
{
    type Query = OrganizationInvitationListQuery;
    type Output = OrganizationInvitationList;
    type Error = OrganizationInvitationListQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[OrganizationInvitationListProjectorSpec::DESCRIPTOR]);

    fn authorization_plan(&self, query: &Self::Query) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                query.organization_id,
                OrganizationInviterRelation::REF,
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

    use crate::authorization::OrganizationInviterRelation;
    use crate::read_model::{
        OrganizationInvitationList, OrganizationInvitationListCriteria,
        OrganizationInvitationListCursor, OrganizationInvitationListReader,
        OrganizationInvitationListReaderError, OrganizationInvitationListSortKey,
    };

    use super::{OrganizationInvitationListQuery, OrganizationInvitationListQueryHandler};

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

    impl OrganizationInvitationListReader for TestReader {
        type Uow = TestUow;

        async fn list(
            &self,
            _uow: &mut Self::Uow,
            _scope_id: OrganizationId,
            _criteria: OrganizationInvitationListCriteria,
            _cursor_options: Option<
                CursorOptions<OrganizationInvitationListSortKey, OrganizationInvitationListCursor>,
            >,
            _limit: PageSize,
        ) -> Result<OrganizationInvitationList, OrganizationInvitationListReaderError> {
            panic!("reader is not exercised by this test")
        }
    }

    #[test]
    fn authorization_plan_requires_expected_relationship() {
        let handler = OrganizationInvitationListQueryHandler::new(TestReader);
        let scope_id = OrganizationId::new();
        let query = OrganizationInvitationListQuery {
            organization_id: scope_id,
            criteria: OrganizationInvitationListCriteria::default(),
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
                        scope_id,
                        OrganizationInviterRelation::REF
                    )
                ),
            ])
        );
    }
}
