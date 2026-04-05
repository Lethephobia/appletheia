use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationJoinRequest;

use crate::authorization::OrganizationJoinRequestCancelerRelation;
use crate::projection::OrganizationJoinRequestRequesterRelationshipProjectorSpec;

use super::{
    OrganizationJoinRequestCancelCommand, OrganizationJoinRequestCancelCommandHandlerError,
    OrganizationJoinRequestCancelOutput,
};

/// Handles `OrganizationJoinRequestCancelCommand`.
pub struct OrganizationJoinRequestCancelCommandHandler<JR>
where
    JR: Repository<OrganizationJoinRequest>,
{
    organization_join_request_repository: JR,
}

impl<JR> OrganizationJoinRequestCancelCommandHandler<JR>
where
    JR: Repository<OrganizationJoinRequest>,
{
    pub fn new(organization_join_request_repository: JR) -> Self {
        Self {
            organization_join_request_repository,
        }
    }
}

impl<JR> CommandHandler for OrganizationJoinRequestCancelCommandHandler<JR>
where
    JR: Repository<OrganizationJoinRequest>,
{
    type Command = OrganizationJoinRequestCancelCommand;
    type Output = OrganizationJoinRequestCancelOutput;
    type ReplayOutput = OrganizationJoinRequestCancelOutput;
    type Error = OrganizationJoinRequestCancelCommandHandlerError;
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
                    relation: OrganizationJoinRequestCancelerRelation::NAME,
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationJoinRequestRequesterRelationshipProjectorSpec::DESCRIPTOR,
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
                OrganizationJoinRequestCancelCommandHandlerError::TargetOrganizationJoinRequestNotFound,
            );
        };

        organization_join_request.cancel()?;

        self.organization_join_request_repository
            .save(uow, _request_context, &mut organization_join_request)
            .await?;

        Ok(CommandHandled::same(OrganizationJoinRequestCancelOutput))
    }
}
