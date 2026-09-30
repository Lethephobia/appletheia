use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationJoinRequestError;
use banking_iam_domain::{Organization, OrganizationJoinRequest};

use crate::authorization::OrganizationJoinRequestRejecterRelation;

use super::{
    OrganizationJoinRequestRejectCommand, OrganizationJoinRequestRejectCommandHandlerError,
    OrganizationJoinRequestRejectOutput,
};

pub struct OrganizationJoinRequestRejectCommandHandler<ORG, JR>
where
    ORG: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_join_request_repository: JR,
}

impl<ORG, JR> OrganizationJoinRequestRejectCommandHandler<ORG, JR>
where
    ORG: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = ORG::Uow>,
{
    pub fn new(organization_repository: ORG, organization_join_request_repository: JR) -> Self {
        Self {
            organization_repository,
            organization_join_request_repository,
        }
    }
}

impl<ORG, JR> CommandHandler for OrganizationJoinRequestRejectCommandHandler<ORG, JR>
where
    ORG: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = ORG::Uow>,
{
    type Command = OrganizationJoinRequestRejectCommand;
    type Output = OrganizationJoinRequestRejectOutput;
    type Error = OrganizationJoinRequestRejectCommandHandlerError;
    type Uow = JR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationJoinRequest,
            >(
                command.organization_join_request_id,
                OrganizationJoinRequestRejecterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut organization_join_request = self
            .organization_join_request_repository
            .read(uow, command.organization_join_request_id)
            .await?;

        let organization = self
            .organization_repository
            .read(uow, *organization_join_request.organization_id()?)
            .await?;

        if organization.is_removed()? {
            return Err(OrganizationJoinRequestError::OrganizationRemoved.into());
        }

        organization_join_request.reject()?;

        self.organization_join_request_repository
            .save(uow, _request_context, &mut organization_join_request)
            .await?;

        Ok(OrganizationJoinRequestRejectOutput {})
    }
}
