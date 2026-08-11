use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationDescriptionChangeResult};

use super::{
    OrganizationDescriptionChangeCommand, OrganizationDescriptionChangeCommandHandlerError,
    OrganizationDescriptionChangeOutput,
};
use crate::authorization::OrganizationProfileEditorRelation;

/// Handles `OrganizationDescriptionChangeCommand`.
pub struct OrganizationDescriptionChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationDescriptionChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationDescriptionChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationDescriptionChangeCommand;
    type Output = OrganizationDescriptionChangeOutput;
    type Error = OrganizationDescriptionChangeCommandHandlerError;
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
    ) -> Result<Self::Output, Self::Error> {
        let mut organization = self
            .organization_repository
            .read(uow, command.organization_id)
            .await?;

        let result = organization.change_description(command.description.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        let output = match result {
            OrganizationDescriptionChangeResult::Changed => {
                OrganizationDescriptionChangeOutput::Changed
            }
            OrganizationDescriptionChangeResult::Rejected { reason } => {
                OrganizationDescriptionChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
