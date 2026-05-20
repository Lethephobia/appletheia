use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationWebsiteUrlChangeResult};

use super::{
    OrganizationWebsiteUrlChangeCommand, OrganizationWebsiteUrlChangeCommandHandlerError,
    OrganizationWebsiteUrlChangeOutput,
};
use crate::authorization::OrganizationProfileEditorRelation;

/// Handles `OrganizationWebsiteUrlChangeCommand`.
pub struct OrganizationWebsiteUrlChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationWebsiteUrlChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationWebsiteUrlChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationWebsiteUrlChangeCommand;
    type Output = OrganizationWebsiteUrlChangeOutput;
    type ReplayOutput = OrganizationWebsiteUrlChangeOutput;
    type Error = OrganizationWebsiteUrlChangeCommandHandlerError;
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
            return Err(OrganizationWebsiteUrlChangeCommandHandlerError::OrganizationNotFound);
        };

        let result = organization.change_website_url(command.website_url.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        let output = match result {
            OrganizationWebsiteUrlChangeResult::Changed => {
                OrganizationWebsiteUrlChangeOutput::Changed
            }
            OrganizationWebsiteUrlChangeResult::Rejected { reason } => {
                OrganizationWebsiteUrlChangeOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
