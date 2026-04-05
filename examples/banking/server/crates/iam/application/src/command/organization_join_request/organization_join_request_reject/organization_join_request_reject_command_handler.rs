use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationJoinRequest;

use crate::authorization::OrganizationJoinRequestRejecterRelation;
use crate::projection::{
    OrganizationJoinRequestOrganizationRelationshipProjectorSpec,
    OrganizationOwnerRelationshipProjectorSpec,
};

use super::{
    OrganizationJoinRequestRejectCommand, OrganizationJoinRequestRejectCommandHandlerError,
    OrganizationJoinRequestRejectOutput,
};

/// Handles `OrganizationJoinRequestRejectCommand`.
pub struct OrganizationJoinRequestRejectCommandHandler<JR>
where
    JR: Repository<OrganizationJoinRequest>,
{
    organization_join_request_repository: JR,
}

impl<JR> OrganizationJoinRequestRejectCommandHandler<JR>
where
    JR: Repository<OrganizationJoinRequest>,
{
    pub fn new(organization_join_request_repository: JR) -> Self {
        Self {
            organization_join_request_repository,
        }
    }
}

impl<JR> CommandHandler for OrganizationJoinRequestRejectCommandHandler<JR>
where
    JR: Repository<OrganizationJoinRequest>,
{
    type Command = OrganizationJoinRequestRejectCommand;
    type Output = OrganizationJoinRequestRejectOutput;
    type ReplayOutput = OrganizationJoinRequestRejectOutput;
    type Error = OrganizationJoinRequestRejectCommandHandlerError;
    type Uow = JR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<OrganizationJoinRequest>(
                        command.organization_join_request_id,
                    ),
                    relation: OrganizationJoinRequestRejecterRelation::NAME,
                },
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
                OrganizationJoinRequestRejectCommandHandlerError::TargetOrganizationJoinRequestNotFound,
            );
        };

        organization_join_request.reject()?;

        self.organization_join_request_repository
            .save(uow, _request_context, &mut organization_join_request)
            .await?;

        Ok(CommandHandled::same(OrganizationJoinRequestRejectOutput))
    }
}
