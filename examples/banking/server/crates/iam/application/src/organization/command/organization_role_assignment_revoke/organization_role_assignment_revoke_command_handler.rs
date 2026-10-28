use super::{
    OrganizationRoleAssignmentRevokeCommand, OrganizationRoleAssignmentRevokeCommandHandlerError,
    OrganizationRoleAssignmentRevokeOutput,
};
use crate::organization::authorization::OrganizationRoleAssignmentManagerRelation;
use appletheia::application::{
    authorization::{AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement},
    command::CommandHandler,
    repository::Repository,
    request_context::RequestContext,
};
use banking_iam_domain::OrganizationRoleAssignment;

pub struct OrganizationRoleAssignmentRevokeCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationRoleAssignmentRevokeCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationRoleAssignmentRevokeCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationRoleAssignmentRevokeCommand;
    type Output = OrganizationRoleAssignmentRevokeOutput;
    type Error = OrganizationRoleAssignmentRevokeCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationRoleAssignment,
            >(
                command.organization_role_assignment_id,
                OrganizationRoleAssignmentManagerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut assignment = self
            .repository
            .read::<OrganizationRoleAssignment>(uow, command.organization_role_assignment_id)
            .await?;
        assignment.revoke()?;
        self.repository
            .save(uow, request_context, &mut assignment)
            .await?;
        Ok(OrganizationRoleAssignmentRevokeOutput {})
    }
}
