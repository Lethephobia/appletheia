use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    Organization, OrganizationMembershipRolesChangeRejectionReason,
    OrganizationMembershipRolesChangeResult, User,
};

use super::{
    UserOrganizationMembershipRolesChangeCommand,
    UserOrganizationMembershipRolesChangeCommandHandlerError,
    UserOrganizationMembershipRolesChangeOutput,
};
use crate::authorization::OrganizationAdminRelation;

/// Handles `UserOrganizationMembershipRolesChangeCommand`.
pub struct UserOrganizationMembershipRolesChangeCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    user_repository: UR,
}

impl<ORG, UR> UserOrganizationMembershipRolesChangeCommandHandler<ORG, UR>
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

impl<ORG, UR> CommandHandler for UserOrganizationMembershipRolesChangeCommandHandler<ORG, UR>
where
    ORG: Repository<Organization>,
    UR: Repository<User, Uow = ORG::Uow>,
{
    type Command = UserOrganizationMembershipRolesChangeCommand;
    type Output = UserOrganizationMembershipRolesChangeOutput;
    type Error = UserOrganizationMembershipRolesChangeCommandHandlerError;
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
    ) -> Result<Self::Output, Self::Error> {
        let organization = self
            .organization_repository
            .read(uow, command.organization_id)
            .await?;

        let mut user = self.user_repository.read(uow, command.user_id).await?;

        if organization.is_removed()? {
            let reason = OrganizationMembershipRolesChangeRejectionReason::OrganizationRemoved;
            user.reject_change_organization_membership_roles(
                command.organization_id,
                command.roles.clone(),
                reason,
            )?;

            self.user_repository
                .save(uow, request_context, &mut user)
                .await?;

            return Ok(UserOrganizationMembershipRolesChangeOutput::Rejected { reason });
        }

        let result = user
            .change_organization_membership_roles(command.organization_id, command.roles.clone())?;

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let output = match result {
            OrganizationMembershipRolesChangeResult::Changed => {
                UserOrganizationMembershipRolesChangeOutput::Changed
            }
            OrganizationMembershipRolesChangeResult::Rejected { reason } => {
                UserOrganizationMembershipRolesChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
