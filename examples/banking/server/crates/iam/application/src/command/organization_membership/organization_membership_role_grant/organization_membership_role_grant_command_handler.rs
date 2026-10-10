use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationMembershipError;
use banking_iam_domain::{Organization, OrganizationMembership};

use super::{
    OrganizationMembershipRoleGrantCommand, OrganizationMembershipRoleGrantCommandHandlerError,
    OrganizationMembershipRoleGrantOutput,
};
use crate::authorization::OrganizationMembershipRolesChangerRelation;

pub struct OrganizationMembershipRoleGrantCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationMembershipRoleGrantCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationMembershipRoleGrantCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationMembershipRoleGrantCommand;
    type Output = OrganizationMembershipRoleGrantOutput;
    type Error = OrganizationMembershipRoleGrantCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationMembership,
            >(
                command.organization_membership_id,
                OrganizationMembershipRolesChangerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut membership = self
            .repository
            .read::<OrganizationMembership>(uow, command.organization_membership_id)
            .await?;

        let organization = self
            .repository
            .read_shared::<Organization>(uow, *membership.organization_id()?)
            .await?;
        if organization.is_removed()? {
            return Err(OrganizationMembershipError::OrganizationRemoved.into());
        }

        membership.grant_role(command.role)?;

        self.repository
            .save(uow, request_context, &mut membership)
            .await?;

        Ok(OrganizationMembershipRoleGrantOutput {})
    }
}
