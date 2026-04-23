use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::command;
use banking_iam_domain::{Organization, OrganizationError, OrganizationId, OrganizationWebsiteUrl};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::authorization::OrganizationWebsiteUrlChangerRelation;
use crate::projection::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};

/// Changes an organization's website URL.
#[command(name = "organization_change_website_url")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeWebsiteUrlCommand {
    pub organization_id: OrganizationId,
    pub website_url: Option<OrganizationWebsiteUrl>,
}

/// Returned after an organization website URL change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeWebsiteUrlOutput;

/// Represents errors returned while changing an organization website URL.
#[derive(Debug, Error)]
pub enum OrganizationChangeWebsiteUrlCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization was not found")]
    OrganizationNotFound,
}

/// Handles `OrganizationChangeWebsiteUrlCommand`.
pub struct OrganizationChangeWebsiteUrlCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationChangeWebsiteUrlCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationChangeWebsiteUrlCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationChangeWebsiteUrlCommand;
    type Output = OrganizationChangeWebsiteUrlOutput;
    type ReplayOutput = OrganizationChangeWebsiteUrlOutput;
    type Error = OrganizationChangeWebsiteUrlCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Organization>(
                    command.organization_id,
                    OrganizationWebsiteUrlChangerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
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
            return Err(OrganizationChangeWebsiteUrlCommandHandlerError::OrganizationNotFound);
        };

        organization.change_website_url(command.website_url.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(CommandHandled::same(OrganizationChangeWebsiteUrlOutput))
    }
}
