use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::query::QueryHandler;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use crate::authorization::OrganizationAdminRelation;
use crate::projection::OrganizationManagementInfoProjectorSpec;
use crate::read_model::{OrganizationManagementInfo, OrganizationManagementInfoReader};

use super::{OrganizationManagementInfoQuery, OrganizationManagementInfoQueryHandlerError};

/// Handles organization-management information queries.
pub struct OrganizationManagementInfoQueryHandler<R>
where
    R: OrganizationManagementInfoReader,
{
    reader: R,
}

impl<R> OrganizationManagementInfoQueryHandler<R>
where
    R: OrganizationManagementInfoReader,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> QueryHandler for OrganizationManagementInfoQueryHandler<R>
where
    R: OrganizationManagementInfoReader,
{
    type Query = OrganizationManagementInfoQuery;
    type Output = Option<OrganizationManagementInfo>;
    type Error = OrganizationManagementInfoQueryHandlerError;
    type Uow = R::Uow;

    const PROJECTOR_DEPENDENCIES: ProjectorDependencies<'static> =
        ProjectorDependencies::Some(&[OrganizationManagementInfoProjectorSpec::DESCRIPTOR]);

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

    use crate::authorization::OrganizationAdminRelation;
    use crate::read_model::{
        OrganizationManagementInfo, OrganizationManagementInfoReader,
        OrganizationManagementInfoReaderError,
    };

    use super::{OrganizationManagementInfoQuery, OrganizationManagementInfoQueryHandler};

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

    struct TestOrganizationManagementInfoReader;

    impl OrganizationManagementInfoReader for TestOrganizationManagementInfoReader {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _organization_id: OrganizationId,
        ) -> Result<Option<OrganizationManagementInfo>, OrganizationManagementInfoReaderError>
        {
            Ok(None)
        }
    }

    #[test]
    fn authorization_plan_requires_organization_admin_relationship() {
        let handler =
            OrganizationManagementInfoQueryHandler::new(TestOrganizationManagementInfoReader);
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&OrganizationManagementInfoQuery { organization_id })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<Organization>(
                        organization_id,
                        OrganizationAdminRelation::REF,
                    )
                ),
            ])
        );
    }
}
