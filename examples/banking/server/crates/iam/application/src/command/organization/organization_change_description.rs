use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::command;
use banking_iam_domain::{
    Organization, OrganizationDescription, OrganizationError, OrganizationId,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::authorization::OrganizationDescriptionChangerRelation;
use crate::projection::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};

/// Changes an organization's description.
#[command(name = "organization_change_description")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeDescriptionCommand {
    pub organization_id: OrganizationId,
    pub description: Option<OrganizationDescription>,
}

/// Returned after an organization description change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeDescriptionOutput;

/// Represents errors returned while changing an organization description.
#[derive(Debug, Error)]
pub enum OrganizationChangeDescriptionCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization was not found")]
    OrganizationNotFound,
}

/// Handles `OrganizationChangeDescriptionCommand`.
pub struct OrganizationChangeDescriptionCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationChangeDescriptionCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationChangeDescriptionCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationChangeDescriptionCommand;
    type Output = OrganizationChangeDescriptionOutput;
    type ReplayOutput = OrganizationChangeDescriptionOutput;
    type Error = OrganizationChangeDescriptionCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Organization>(
                    command.organization_id,
                    OrganizationDescriptionChangerRelation::REF,
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
            return Err(OrganizationChangeDescriptionCommandHandlerError::OrganizationNotFound);
        };

        organization.change_description(command.description.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(CommandHandled::same(OrganizationChangeDescriptionOutput))
    }
}
