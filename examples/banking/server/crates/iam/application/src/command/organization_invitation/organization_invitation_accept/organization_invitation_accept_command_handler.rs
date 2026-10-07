use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationInvitationError;
use banking_iam_domain::{Organization, OrganizationInvitation};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use crate::authorization::OrganizationInvitationInviteeRelation;

use super::{
    OrganizationInvitationAcceptCommand, OrganizationInvitationAcceptCommandHandlerError,
    OrganizationInvitationAcceptOutput,
};

pub struct OrganizationInvitationAcceptCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationInvitationAcceptCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationInvitationAcceptCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationInvitationAcceptCommand;
    type Output = OrganizationInvitationAcceptOutput;
    type Error = OrganizationInvitationAcceptCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationInvitation,
            >(
                command.organization_invitation_id,
                OrganizationInvitationInviteeRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut organization_invitation = self
            .repository
            .read::<OrganizationInvitation>(uow, command.organization_invitation_id)
            .await?;

        let organization = self
            .repository
            .read::<Organization>(uow, *organization_invitation.organization_id()?)
            .await?;

        if organization.is_removed()? {
            return Err(OrganizationInvitationError::OrganizationRemoved.into());
        }

        organization_invitation.accept(CurrentDateTime::new())?;

        self.repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        Ok(OrganizationInvitationAcceptOutput {})
    }
}
