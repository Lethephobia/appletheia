use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    Organization, OrganizationMembershipRemoveRejectionReason, OrganizationMembershipRemoveResult,
    User,
};

use super::{
    UserOrganizationMembershipRemoveCommand, UserOrganizationMembershipRemoveCommandHandlerError,
    UserOrganizationMembershipRemoveOutput,
};
use crate::authorization::OrganizationAdminRelation;

/// Handles `UserOrganizationMembershipRemoveCommand`.
pub struct UserOrganizationMembershipRemoveCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    user_repository: UR,
}

impl<ORG, UR> UserOrganizationMembershipRemoveCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    pub fn new(organization_repository: ORG, user_repository: UR) -> Self {
        Self {
            organization_repository,
            user_repository,
        }
    }
}

impl<ORG, UR> CommandHandler for UserOrganizationMembershipRemoveCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    type Command = UserOrganizationMembershipRemoveCommand;
    type Output = UserOrganizationMembershipRemoveOutput;
    type ReplayOutput = UserOrganizationMembershipRemoveOutput;
    type Error = UserOrganizationMembershipRemoveCommandHandlerError;
    type Uow = ORG::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                command.organization_id,
                OrganizationAdminRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(organization) = self
            .organization_repository
            .find(uow, command.organization_id)
            .await?
        else {
            return Err(UserOrganizationMembershipRemoveCommandHandlerError::OrganizationNotFound);
        };

        let Some(mut user) = self.user_repository.find(uow, command.user_id).await? else {
            return Err(UserOrganizationMembershipRemoveCommandHandlerError::UserNotFound);
        };

        if organization.is_removed()? {
            let reason = OrganizationMembershipRemoveRejectionReason::OrganizationRemoved;
            user.reject_remove_organization_membership(command.organization_id, reason)?;

            self.user_repository
                .save(uow, request_context, &mut user)
                .await?;

            return Ok(CommandHandled::same(
                UserOrganizationMembershipRemoveOutput::Rejected { reason },
            ));
        }

        let result = user.remove_organization_membership(command.organization_id)?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let output = match result {
            OrganizationMembershipRemoveResult::Removed => {
                UserOrganizationMembershipRemoveOutput::Removed
            }
            OrganizationMembershipRemoveResult::Rejected { reason } => {
                UserOrganizationMembershipRemoveOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
