use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    Organization, OrganizationMembership, OrganizationMembershipActivateRejectionReason,
    OrganizationMembershipActivateResult,
};

use super::{
    OrganizationMembershipActivateCommand, OrganizationMembershipActivateCommandHandlerError,
    OrganizationMembershipActivateOutput,
};
use crate::authorization::OrganizationMembershipActivatorRelation;

/// Handles `OrganizationMembershipActivateCommand`.
pub struct OrganizationMembershipActivateCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_membership_repository: MR,
}

impl<ORG, MR> OrganizationMembershipActivateCommandHandler<ORG, MR>
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

impl<ORG, MR> CommandHandler for OrganizationMembershipActivateCommandHandler<ORG, MR>
where
    ORG: Repository<Organization>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    type Command = OrganizationMembershipActivateCommand;
    type Output = OrganizationMembershipActivateOutput;
    type ReplayOutput = OrganizationMembershipActivateOutput;
    type Error = OrganizationMembershipActivateCommandHandlerError;
    type Uow = ORG::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationMembership,
            >(
                command.organization_membership_id,
                OrganizationMembershipActivatorRelation::REF,
            )),
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
                OrganizationMembershipActivateCommandHandlerError::TargetOrganizationMembershipNotFound,
            );
        };

        let Some(organization) = self
            .organization_repository
            .find(uow, *organization_membership.organization_id()?)
            .await?
        else {
            return Err(OrganizationMembershipActivateCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            let reason = OrganizationMembershipActivateRejectionReason::OrganizationRemoved;
            organization_membership.reject_activate(reason)?;

            self.organization_membership_repository
                .save(uow, request_context, &mut organization_membership)
                .await?;

            return Ok(CommandHandled::same(
                OrganizationMembershipActivateOutput::Rejected { reason },
            ));
        }

        let result = organization_membership.activate()?;

        self.organization_membership_repository
            .save(uow, request_context, &mut organization_membership)
            .await?;

        let output = match result {
            OrganizationMembershipActivateResult::Activated => {
                OrganizationMembershipActivateOutput::Activated
            }
            OrganizationMembershipActivateResult::Rejected { reason } => {
                OrganizationMembershipActivateOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
