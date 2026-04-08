use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationJoinRequest};

use crate::authorization::OrganizationJoinRequestApproverRelation;
use crate::projection::{
    OrganizationJoinRequestOrganizationRelationshipProjectorSpec,
    OrganizationOwnerRelationshipProjectorSpec,
};

use super::{
    OrganizationJoinRequestApproveCommand, OrganizationJoinRequestApproveCommandHandlerError,
    OrganizationJoinRequestApproveOutput,
};

/// Handles `OrganizationJoinRequestApproveCommand`.
pub struct OrganizationJoinRequestApproveCommandHandler<ORG, JR>
where
    ORG: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_join_request_repository: JR,
}

impl<ORG, JR> OrganizationJoinRequestApproveCommandHandler<ORG, JR>
where
    ORG: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = ORG::Uow>,
{
    pub fn new(organization_repository: ORG, organization_join_request_repository: JR) -> Self {
        Self {
            organization_repository,
            organization_join_request_repository,
        }
    }
}

impl<ORG, JR> CommandHandler for OrganizationJoinRequestApproveCommandHandler<ORG, JR>
where
    ORG: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = ORG::Uow>,
{
    type Command = OrganizationJoinRequestApproveCommand;
    type Output = OrganizationJoinRequestApproveOutput;
    type ReplayOutput = OrganizationJoinRequestApproveOutput;
    type Error = OrganizationJoinRequestApproveCommandHandlerError;
    type Uow = JR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<OrganizationJoinRequest>(
                    command.organization_join_request_id,
                    OrganizationJoinRequestApproverRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationJoinRequestOrganizationRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut organization_join_request) = self
            .organization_join_request_repository
            .find(uow, command.organization_join_request_id)
            .await?
        else {
            return Err(
                OrganizationJoinRequestApproveCommandHandlerError::TargetOrganizationJoinRequestNotFound,
            );
        };

        let Some(organization) = self
            .organization_repository
            .find(uow, *organization_join_request.organization_id()?)
            .await?
        else {
            return Err(OrganizationJoinRequestApproveCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationJoinRequestApproveCommandHandlerError::OrganizationRemoved);
        }

        organization_join_request.approve()?;

        self.organization_join_request_repository
            .save(uow, _request_context, &mut organization_join_request)
            .await?;

        Ok(CommandHandled::same(OrganizationJoinRequestApproveOutput))
    }
}
