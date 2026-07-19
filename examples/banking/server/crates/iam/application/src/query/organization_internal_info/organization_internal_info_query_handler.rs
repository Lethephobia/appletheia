use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use crate::authorization::OrganizationMemberRelation;
use crate::projection::OrganizationInternalInfoProjectorSpec;
use crate::read_model::{OrganizationInternalInfo, OrganizationInternalInfoReader};

use super::{OrganizationInternalInfoQuery, OrganizationInternalInfoQueryHandlerError};

/// Handles organization-internal information queries.
pub struct OrganizationInternalInfoQueryHandler<R>
where
    R: OrganizationInternalInfoReader,
{
    reader: R,
}

impl<R> OrganizationInternalInfoQueryHandler<R>
where
    R: OrganizationInternalInfoReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for OrganizationInternalInfoQueryHandler<R>
where
    R: OrganizationInternalInfoReader,
{
    type Query = OrganizationInternalInfoQuery;
    type Output = Option<OrganizationInternalInfo>;
    type Error = OrganizationInternalInfoQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[OrganizationInternalInfoProjectorSpec::DESCRIPTOR]);

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
        Ok(self.reader.find(uow, query.organization_id).await?)
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

    use crate::authorization::OrganizationMemberRelation;
    use crate::read_model::{
        OrganizationInternalInfo, OrganizationInternalInfoReader,
        OrganizationInternalInfoReaderError,
    };

    use super::{OrganizationInternalInfoQuery, OrganizationInternalInfoQueryHandler};

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

    struct TestOrganizationInternalInfoReader;

    impl OrganizationInternalInfoReader for TestOrganizationInternalInfoReader {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _organization_id: OrganizationId,
        ) -> Result<Option<OrganizationInternalInfo>, OrganizationInternalInfoReaderError> {
            Ok(None)
        }
    }

    #[test]
    fn authorization_plan_requires_organization_member_relationship() {
        let handler = OrganizationInternalInfoQueryHandler::new(TestOrganizationInternalInfoReader);
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&OrganizationInternalInfoQuery { organization_id })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationMemberRelation::REF,
                    )
                ),
            ])
        );
    }
}
