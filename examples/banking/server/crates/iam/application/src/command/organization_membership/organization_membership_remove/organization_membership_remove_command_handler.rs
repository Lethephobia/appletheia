use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationMembership;

use super::{
    OrganizationMembershipRemoveCommand, OrganizationMembershipRemoveCommandHandlerError,
    OrganizationMembershipRemoveOutput,
};
use crate::authorization::OrganizationMembershipRemoverRelation;

pub struct OrganizationMembershipRemoveCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationMembershipRemoveCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationMembershipRemoveCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationMembershipRemoveCommand;
    type Output = OrganizationMembershipRemoveOutput;
    type Error = OrganizationMembershipRemoveCommandHandlerError;
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
            .repository
            .read::<OrganizationMembership>(uow, command.organization_membership_id)
            .await?;

        membership.remove()?;

        self.repository
            .save::<OrganizationMembership>(uow, request_context, &mut membership)
            .await?;

        Ok(OrganizationMembershipRemoveOutput {})
    }
}
