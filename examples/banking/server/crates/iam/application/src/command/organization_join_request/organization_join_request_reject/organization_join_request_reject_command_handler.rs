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

pub struct OrganizationJoinRequestRejectCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationJoinRequestRejectCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationJoinRequestRejectCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationJoinRequestRejectCommand;
    type Output = OrganizationJoinRequestRejectOutput;
    type Error = OrganizationJoinRequestRejectCommandHandlerError;
    type Uow = R::Uow;

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
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut organization_join_request = self
            .repository
            .read::<OrganizationJoinRequest>(uow, command.organization_join_request_id)
            .await?;

        let organization = self
            .repository
            .read::<Organization>(uow, *organization_join_request.organization_id()?)
            .await?;

        if organization.is_removed()? {
            return Err(OrganizationJoinRequestError::OrganizationRemoved.into());
        }

        organization_join_request.reject()?;

        self.repository
            .save(uow, request_context, &mut organization_join_request)
            .await?;

        Ok(OrganizationJoinRequestRejectOutput {})
    }
}
