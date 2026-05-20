use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationPictureChangeResult};

use super::{
    OrganizationPictureChangeCommand, OrganizationPictureChangeCommandHandlerError,
    OrganizationPictureChangeOutput,
};
use crate::authorization::OrganizationProfileEditorRelation;

/// Handles `OrganizationPictureChangeCommand`.
pub struct OrganizationPictureChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationPictureChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationPictureChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationPictureChangeCommand;
    type Output = OrganizationPictureChangeOutput;
    type ReplayOutput = OrganizationPictureChangeOutput;
    type Error = OrganizationPictureChangeCommandHandlerError;
    type Uow = OR::Uow;

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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut organization) = self
            .organization_repository
            .find(uow, command.organization_id)
            .await?
        else {
            return Err(OrganizationPictureChangeCommandHandlerError::OrganizationNotFound);
        };

        let result = organization.change_picture(command.picture.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        let output = match result {
            OrganizationPictureChangeResult::Changed => OrganizationPictureChangeOutput::Changed,
            OrganizationPictureChangeResult::Rejected { reason } => {
                OrganizationPictureChangeOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
