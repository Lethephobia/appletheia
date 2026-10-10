use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue};
use banking_ledger_domain::currency_registrar_invitation::CurrencyRegistrarInvitationError;
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarInvitation, CurrencyRegistrarInvitationIssuer,
    CurrencyRegistrarInvitationState, CurrencyRegistrarMembership,
    CurrencyRegistrarMembershipState, User,
};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use crate::authorization::{CurrencyRegistrarMemberRelation, UserOwnerRelation};

use super::{
    CurrencyRegistrarInvitationIssueCommand, CurrencyRegistrarInvitationIssueCommandHandlerError,
    CurrencyRegistrarInvitationIssueOutput,
};

pub struct CurrencyRegistrarInvitationIssueCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarInvitationIssueCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn registrar_user_unique_value(
        command: &CurrencyRegistrarInvitationIssueCommand,
    ) -> Result<UniqueValue, CurrencyRegistrarInvitationIssueCommandHandlerError> {
        let currency_registrar_id = command.currency_registrar_id.value().to_string();
        let invitee_id = command.invitee_id.value().to_string();
        Ok(UniqueValue::from_strings([
            currency_registrar_id.as_str(),
            invitee_id.as_str(),
        ])?)
    }

    fn registrar_invitee_unique_value(
        command: &CurrencyRegistrarInvitationIssueCommand,
    ) -> Result<UniqueValue, CurrencyRegistrarInvitationIssueCommandHandlerError> {
        let currency_registrar_id = command.currency_registrar_id.value().to_string();
        let invitee_id = command.invitee_id.value().to_string();
        Ok(UniqueValue::from_strings([
            currency_registrar_id.as_str(),
            invitee_id.as_str(),
        ])?)
    }
}

impl<R> CommandHandler for CurrencyRegistrarInvitationIssueCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarInvitationIssueCommand;
    type Output = CurrencyRegistrarInvitationIssueOutput;
    type Error = CurrencyRegistrarInvitationIssueCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        let principal_requirement = match command.issuer {
            CurrencyRegistrarInvitationIssuer::System => PrincipalRequirement::System,
            CurrencyRegistrarInvitationIssuer::User(user_id) => {
                PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::All(
                    vec![
                        RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                        RelationshipRequirement::check::<CurrencyRegistrar>(
                            command.currency_registrar_id,
                            CurrencyRegistrarMemberRelation::REF,
                        ),
                    ],
                ))
            }
        };

        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            principal_requirement,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut currency_registrar_invitation = CurrencyRegistrarInvitation::new();
        let currency_registrar_invitation_id = currency_registrar_invitation.aggregate_id();

        self.repository
            .read_shared::<CurrencyRegistrar>(uow, command.currency_registrar_id)
            .await?;

        let membership_unique_value = Self::registrar_user_unique_value(command)?;
        if self
            .repository
            .find_shared_by_unique_value::<CurrencyRegistrarMembership>(
                uow,
                CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY,
                &membership_unique_value,
            )
            .await?
            .is_some()
        {
            return Err(CurrencyRegistrarInvitationError::InviteeAlreadyMember.into());
        }

        let unique_value = Self::registrar_invitee_unique_value(command)?;
        if self
            .repository
            .find_shared_by_unique_value::<CurrencyRegistrarInvitation>(
                uow,
                CurrencyRegistrarInvitationState::REGISTRAR_INVITEE_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(CurrencyRegistrarInvitationError::AlreadyIssued.into());
        }

        currency_registrar_invitation.issue(
            command.currency_registrar_id,
            command.invitee_id,
            command.issuer,
            command.expires_at,
            CurrentDateTime::new(),
        )?;

        self.repository
            .save(uow, request_context, &mut currency_registrar_invitation)
            .await?;

        Ok(CurrencyRegistrarInvitationIssueOutput {
            currency_registrar_invitation_id,
        })
    }
}
