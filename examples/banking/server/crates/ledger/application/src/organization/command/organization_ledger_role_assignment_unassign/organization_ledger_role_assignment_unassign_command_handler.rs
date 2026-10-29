use super::{
    OrganizationLedgerRoleAssignmentUnassignCommand,
    OrganizationLedgerRoleAssignmentUnassignCommandHandlerError,
    OrganizationLedgerRoleAssignmentUnassignOutput,
};
use crate::organization::authorization::OrganizationLedgerRoleAssignmentManagerRelation;
use appletheia::application::{
    authorization::{AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement},
    command::CommandHandler,
    repository::Repository,
    request_context::RequestContext,
};
use banking_ledger_domain::OrganizationLedgerRoleAssignment;

pub struct OrganizationLedgerRoleAssignmentUnassignCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationLedgerRoleAssignmentUnassignCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationLedgerRoleAssignmentUnassignCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationLedgerRoleAssignmentUnassignCommand;
    type Output = OrganizationLedgerRoleAssignmentUnassignOutput;
    type Error = OrganizationLedgerRoleAssignmentUnassignCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationLedgerRoleAssignment,
            >(
                command.organization_ledger_role_assignment_id,
                OrganizationLedgerRoleAssignmentManagerRelation::REF,
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
            .read::<OrganizationLedgerRoleAssignment>(
                uow,
                command.organization_ledger_role_assignment_id,
            )
            .await?;
        assignment.unassign()?;
        self.repository
            .save(uow, request_context, &mut assignment)
            .await?;
        Ok(OrganizationLedgerRoleAssignmentUnassignOutput {})
    }
}
