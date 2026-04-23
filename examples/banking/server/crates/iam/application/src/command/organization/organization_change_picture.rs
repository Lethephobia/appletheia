use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::command;
use banking_iam_domain::{Organization, OrganizationError, OrganizationId, OrganizationPictureRef};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::authorization::OrganizationPictureChangerRelation;
use crate::projection::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};

/// Changes an organization's picture.
#[command(name = "organization_change_picture")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangePictureCommand {
    pub organization_id: OrganizationId,
    pub picture: Option<OrganizationPictureRef>,
}

/// Returned after an organization picture change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangePictureOutput;

/// Represents errors returned while changing an organization picture.
#[derive(Debug, Error)]
pub enum OrganizationChangePictureCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization was not found")]
    OrganizationNotFound,
}

/// Handles `OrganizationChangePictureCommand`.
pub struct OrganizationChangePictureCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationChangePictureCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationChangePictureCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationChangePictureCommand;
    type Output = OrganizationChangePictureOutput;
    type ReplayOutput = OrganizationChangePictureOutput;
    type Error = OrganizationChangePictureCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Organization>(
                    command.organization_id,
                    OrganizationPictureChangerRelation::REF,
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
            return Err(OrganizationChangePictureCommandHandlerError::OrganizationNotFound);
        };

        organization.change_picture(command.picture.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(CommandHandled::same(OrganizationChangePictureOutput))
    }
}
