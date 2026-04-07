use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationRefOwned,
    RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationMembership};

use super::{
    OrganizationMembershipRemoveCommand, OrganizationMembershipRemoveCommandHandlerError,
    OrganizationMembershipRemoveOutput,
};
use crate::authorization::OrganizationMembershipRemoverRelation;
use crate::projection::OrganizationMembershipOrganizationRelationshipProjectorSpec;

/// Handles `OrganizationMembershipRemoveCommand`.
pub struct OrganizationMembershipRemoveCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_membership_repository: MR,
}

impl<ORG, MR> OrganizationMembershipRemoveCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    pub fn new(organization_repository: ORG, organization_membership_repository: MR) -> Self {
        Self {
            organization_repository,
            organization_membership_repository,
        }
    }
}

impl<ORG, MR> CommandHandler for OrganizationMembershipRemoveCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    type Command = OrganizationMembershipRemoveCommand;
    type Output = OrganizationMembershipRemoveOutput;
    type ReplayOutput = OrganizationMembershipRemoveOutput;
    type Error = OrganizationMembershipRemoveCommandHandlerError;
    type Uow = ORG::Uow;

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
                    relation: RelationRefOwned::from(OrganizationMembershipRemoverRelation::REF),
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
                OrganizationMembershipRemoveCommandHandlerError::TargetOrganizationMembershipNotFound,
            );
        };

        let Some(organization) = self
            .organization_repository
            .find(uow, *organization_membership.organization_id()?)
            .await?
        else {
            return Err(OrganizationMembershipRemoveCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationMembershipRemoveCommandHandlerError::OrganizationRemoved);
        }

        organization_membership.remove()?;

        self.organization_membership_repository
            .save(uow, request_context, &mut organization_membership)
            .await?;

        Ok(CommandHandled::same(OrganizationMembershipRemoveOutput))
    }
}
