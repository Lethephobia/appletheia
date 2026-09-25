use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationDisplayNameChangeResult};

use super::{
    OrganizationDisplayNameChangeCommand, OrganizationDisplayNameChangeCommandHandlerError,
    OrganizationDisplayNameChangeOutput,
};
use crate::authorization::OrganizationProfileEditorRelation;

/// Handles `OrganizationDisplayNameChangeCommand`.
pub struct OrganizationDisplayNameChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationDisplayNameChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationDisplayNameChangeCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationDisplayNameChangeCommand;
    type Output = OrganizationDisplayNameChangeOutput;
    type Error = OrganizationDisplayNameChangeCommandHandlerError;
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

        let result = organization.change_display_name(command.display_name.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        let output = match result {
            OrganizationDisplayNameChangeResult::Changed => {
                OrganizationDisplayNameChangeOutput::Changed
            }
            OrganizationDisplayNameChangeResult::Rejected { reason } => {
                OrganizationDisplayNameChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
