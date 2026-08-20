use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{OrganizationMembership, OrganizationMembershipRemoveResult};

use super::{
    OrganizationMembershipRemoveCommand, OrganizationMembershipRemoveCommandHandlerError,
    OrganizationMembershipRemoveOutput,
};
use crate::authorization::OrganizationMembershipRemoverRelation;

/// Handles `OrganizationMembershipRemoveCommand`.
pub struct OrganizationMembershipRemoveCommandHandler<MR>
where
    MR: Repository<OrganizationMembership>,
{
    organization_membership_repository: MR,
}

impl<MR> OrganizationMembershipRemoveCommandHandler<MR>
where
    MR: Repository<OrganizationMembership>,
{
    pub fn new(organization_membership_repository: MR) -> Self {
        Self {
            organization_membership_repository,
        }
    }
}

impl<MR> CommandHandler for OrganizationMembershipRemoveCommandHandler<MR>
where
    MR: Repository<OrganizationMembership>,
{
    type Command = OrganizationMembershipRemoveCommand;
    type Output = OrganizationMembershipRemoveOutput;
    type Error = OrganizationMembershipRemoveCommandHandlerError;
    type Uow = MR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationMembership,
            >(
                command.organization_membership_id,
                OrganizationMembershipRemoverRelation::REF,
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

        let result = membership.remove()?;

        self.organization_membership_repository
            .save(uow, request_context, &mut membership)
            .await?;

        let output = match result {
            OrganizationMembershipRemoveResult::Removed => {
                OrganizationMembershipRemoveOutput::Removed
            }
            OrganizationMembershipRemoveResult::Rejected { reason } => {
                OrganizationMembershipRemoveOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
