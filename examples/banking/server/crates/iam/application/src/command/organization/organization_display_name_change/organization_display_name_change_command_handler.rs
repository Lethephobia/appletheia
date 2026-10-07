use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::Organization;

use super::{
    OrganizationDisplayNameChangeCommand, OrganizationDisplayNameChangeCommandHandlerError,
    OrganizationDisplayNameChangeOutput,
};
use crate::authorization::OrganizationProfileEditorRelation;

pub struct OrganizationDisplayNameChangeCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationDisplayNameChangeCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationDisplayNameChangeCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationDisplayNameChangeCommand;
    type Output = OrganizationDisplayNameChangeOutput;
    type Error = OrganizationDisplayNameChangeCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                command.organization_id,
                OrganizationProfileEditorRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut organization = self
            .repository
            .read::<Organization>(uow, command.organization_id)
            .await?;

        organization.change_display_name(command.display_name.clone())?;

        self.repository
            .save::<Organization>(uow, request_context, &mut organization)
            .await?;

        Ok(OrganizationDisplayNameChangeOutput {})
    }
}
