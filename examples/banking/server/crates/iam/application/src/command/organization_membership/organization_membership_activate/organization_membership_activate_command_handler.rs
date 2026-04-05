use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationMembership;

use super::{
    OrganizationMembershipActivateCommand, OrganizationMembershipActivateCommandHandlerError,
    OrganizationMembershipActivateOutput,
};
use crate::authorization::OrganizationMembershipActivatorRelation;
use crate::projection::OrganizationMembershipOrganizationRelationshipProjectorSpec;

/// Handles `OrganizationMembershipActivateCommand`.
pub struct OrganizationMembershipActivateCommandHandler<OR>
where
    OR: Repository<OrganizationMembership>,
{
    organization_membership_repository: OR,
}

impl<OR> OrganizationMembershipActivateCommandHandler<OR>
where
    OR: Repository<OrganizationMembership>,
{
    pub fn new(organization_membership_repository: OR) -> Self {
        Self {
            organization_membership_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationMembershipActivateCommandHandler<OR>
where
    OR: Repository<OrganizationMembership>,
{
    type Command = OrganizationMembershipActivateCommand;
    type Output = OrganizationMembershipActivateOutput;
    type ReplayOutput = OrganizationMembershipActivateOutput;
    type Error = OrganizationMembershipActivateCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<OrganizationMembership>(
                        command.organization_membership_id,
                    ),
                    relation: OrganizationMembershipActivatorRelation::NAME,
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationMembershipOrganizationRelationshipProjectorSpec::DESCRIPTOR,
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
        let Some(mut organization_membership) = self
            .organization_membership_repository
            .find(uow, command.organization_membership_id)
            .await?
        else {
            return Err(
                OrganizationMembershipActivateCommandHandlerError::TargetOrganizationMembershipNotFound,
            );
        };

        organization_membership.activate()?;

        self.organization_membership_repository
            .save(uow, request_context, &mut organization_membership)
            .await?;

        Ok(CommandHandled::same(OrganizationMembershipActivateOutput))
    }
}
