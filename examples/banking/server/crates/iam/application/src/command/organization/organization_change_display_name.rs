use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::command;
use banking_iam_domain::{
    Organization, OrganizationDisplayName, OrganizationError, OrganizationId,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::authorization::OrganizationDisplayNameChangerRelation;
use crate::projection::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};

/// Changes an organization's display name.
#[command(name = "organization_change_display_name")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeDisplayNameCommand {
    pub organization_id: OrganizationId,
    pub display_name: OrganizationDisplayName,
}

/// Returned after an organization display name change request is applied.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationChangeDisplayNameOutput;

/// Represents errors returned while changing an organization display name.
#[derive(Debug, Error)]
pub enum OrganizationChangeDisplayNameCommandHandlerError {
    #[error("organization repository failed")]
    OrganizationRepository(#[from] RepositoryError<Organization>),

    #[error("organization aggregate failed")]
    Organization(#[from] OrganizationError),

    #[error("organization was not found")]
    OrganizationNotFound,
}

/// Handles `OrganizationChangeDisplayNameCommand`.
pub struct OrganizationChangeDisplayNameCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    organization_repository: OR,
}

impl<OR> OrganizationChangeDisplayNameCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    pub fn new(organization_repository: OR) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationChangeDisplayNameCommandHandler<OR>
where
    OR: Repository<Organization>,
{
    type Command = OrganizationChangeDisplayNameCommand;
    type Output = OrganizationChangeDisplayNameOutput;
    type ReplayOutput = OrganizationChangeDisplayNameOutput;
    type Error = OrganizationChangeDisplayNameCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Organization>(
                    command.organization_id,
                    OrganizationDisplayNameChangerRelation::REF,
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
            return Err(OrganizationChangeDisplayNameCommandHandlerError::OrganizationNotFound);
        };

        organization.change_display_name(command.display_name.clone())?;

        self.organization_repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(CommandHandled::same(OrganizationChangeDisplayNameOutput))
    }
}
