use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationMembership;

use super::{
    OrganizationMembershipDeleteCommand, OrganizationMembershipDeleteCommandHandlerError,
    OrganizationMembershipDeleteOutput,
};
use crate::organization::authorization::OrganizationMembershipManagerRelation;

pub struct OrganizationMembershipDeleteCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationMembershipDeleteCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationMembershipDeleteCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationMembershipDeleteCommand;
    type Output = OrganizationMembershipDeleteOutput;
    type Error = OrganizationMembershipDeleteCommandHandlerError;
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
                OrganizationMembershipManagerRelation::REF,
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

        membership.delete()?;

        self.repository
            .save(uow, request_context, &mut membership)
            .await?;

        Ok(OrganizationMembershipDeleteOutput {})
    }
}
