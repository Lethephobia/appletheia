use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    Organization, OrganizationMembership, OrganizationMembershipRolesChangeRejectionReason,
    OrganizationMembershipRolesChangeResult,
};

use super::{
    OrganizationMembershipRolesChangeCommand, OrganizationMembershipRolesChangeCommandHandlerError,
    OrganizationMembershipRolesChangeOutput,
};
use crate::authorization::OrganizationMembershipRolesChangerRelation;

/// Handles `OrganizationMembershipRolesChangeCommand`.
pub struct OrganizationMembershipRolesChangeCommandHandler<OR, MR>
where
    OR: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
{
    organization_repository: OR,
    organization_membership_repository: MR,
}

impl<OR, MR> OrganizationMembershipRolesChangeCommandHandler<OR, MR>
where
    OR: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
{
    pub fn new(organization_repository: OR, organization_membership_repository: MR) -> Self {
        Self {
            organization_repository,
            organization_membership_repository,
        }
    }
}

impl<OR, MR> CommandHandler for OrganizationMembershipRolesChangeCommandHandler<OR, MR>
where
    OR: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
{
    type Command = OrganizationMembershipRolesChangeCommand;
    type Output = OrganizationMembershipRolesChangeOutput;
    type Error = OrganizationMembershipRolesChangeCommandHandlerError;
    type Uow = OR::Uow;

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
            .organization_membership_repository
            .read(uow, command.organization_membership_id)
            .await?;

        let organization = self
            .organization_repository
            .read(uow, *membership.organization_id()?)
            .await?;
        if organization.is_removed()? {
            let reason = OrganizationMembershipRolesChangeRejectionReason::OrganizationRemoved;
            membership.reject_change_roles(command.roles.clone(), reason)?;

            self.organization_membership_repository
                .save(uow, request_context, &mut membership)
                .await?;

            return Ok(OrganizationMembershipRolesChangeOutput::Rejected { reason });
        }

        let result = membership.change_roles(command.roles.clone())?;

        self.organization_membership_repository
            .save(uow, request_context, &mut membership)
            .await?;

        let output = match result {
            OrganizationMembershipRolesChangeResult::Changed => {
                OrganizationMembershipRolesChangeOutput::Changed
            }
            OrganizationMembershipRolesChangeResult::Rejected { reason } => {
                OrganizationMembershipRolesChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
