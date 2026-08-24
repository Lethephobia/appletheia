use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue};
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarInvitation, CurrencyRegistrarInvitationIssuance,
    CurrencyRegistrarInvitationIssueRejectionReason, CurrencyRegistrarInvitationIssueResult,
    CurrencyRegistrarInvitationIssuer, CurrencyRegistrarInvitationState,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipState, User,
};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use crate::authorization::{CurrencyRegistrarMemberRelation, UserOwnerRelation};

use super::{
    CurrencyRegistrarInvitationIssueCommand, CurrencyRegistrarInvitationIssueCommandHandlerError,
    CurrencyRegistrarInvitationIssueOutput,
};

/// Handles `CurrencyRegistrarInvitationIssueCommand`.
pub struct CurrencyRegistrarInvitationIssueCommandHandler<ORG, IR, MR>
where
    ORG: Repository<CurrencyRegistrar>,
    IR: Repository<CurrencyRegistrarInvitation, Uow = ORG::Uow>,
    MR: Repository<CurrencyRegistrarMembership, Uow = ORG::Uow>,
{
    currency_registrar_repository: ORG,
    currency_registrar_invitation_repository: IR,
    membership_repository: MR,
}

impl<ORG, IR, MR> CurrencyRegistrarInvitationIssueCommandHandler<ORG, IR, MR>
where
    ORG: Repository<CurrencyRegistrar>,
    IR: Repository<CurrencyRegistrarInvitation, Uow = ORG::Uow>,
    MR: Repository<CurrencyRegistrarMembership, Uow = ORG::Uow>,
{
    pub fn new(
        currency_registrar_repository: ORG,
        currency_registrar_invitation_repository: IR,
        membership_repository: MR,
    ) -> Self {
        Self {
            currency_registrar_repository,
            currency_registrar_invitation_repository,
            membership_repository,
        }
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

impl<ORG, IR, MR> CommandHandler for CurrencyRegistrarInvitationIssueCommandHandler<ORG, IR, MR>
where
    ORG: Repository<CurrencyRegistrar>,
    IR: Repository<CurrencyRegistrarInvitation, Uow = ORG::Uow>,
    MR: Repository<CurrencyRegistrarMembership, Uow = ORG::Uow>,
{
    type Command = CurrencyRegistrarInvitationIssueCommand;
    type Output = CurrencyRegistrarInvitationIssueOutput;
    type Error = CurrencyRegistrarInvitationIssueCommandHandlerError;
    type Uow = ORG::Uow;

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
        let issuance = CurrencyRegistrarInvitationIssuance {
            currency_registrar_id: command.currency_registrar_id,
            invitee_id: command.invitee_id,
            issuer: command.issuer,
            expires_at: command.expires_at,
        };

        self.currency_registrar_repository
            .read(uow, command.currency_registrar_id)
            .await?;

        let membership_unique_value = Self::registrar_user_unique_value(command)?;
        if self
            .membership_repository
            .find_by_unique_value(
                uow,
                CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY,
                &membership_unique_value,
            )
            .await?
            .is_some()
        {
            let reason = CurrencyRegistrarInvitationIssueRejectionReason::InviteeAlreadyMember;
            currency_registrar_invitation.reject_issue(issuance, reason)?;

            self.currency_registrar_invitation_repository
                .save(uow, request_context, &mut currency_registrar_invitation)
                .await?;

            return Ok(CurrencyRegistrarInvitationIssueOutput::Rejected {
                currency_registrar_invitation_id,
                reason,
            });
        }

        let unique_value = Self::registrar_invitee_unique_value(command)?;
        if self
            .currency_registrar_invitation_repository
            .find_by_unique_value(
                uow,
                CurrencyRegistrarInvitationState::REGISTRAR_INVITEE_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            let reason = CurrencyRegistrarInvitationIssueRejectionReason::AlreadyIssued;
            currency_registrar_invitation.reject_issue(issuance, reason)?;

            self.currency_registrar_invitation_repository
                .save(uow, request_context, &mut currency_registrar_invitation)
                .await?;

            return Ok(CurrencyRegistrarInvitationIssueOutput::Rejected {
                currency_registrar_invitation_id,
                reason,
            });
        }

        let result = currency_registrar_invitation.issue(issuance, CurrentDateTime::new())?;

        self.currency_registrar_invitation_repository
            .save(uow, request_context, &mut currency_registrar_invitation)
            .await?;

        let output = match result {
            CurrencyRegistrarInvitationIssueResult::Issued => {
                CurrencyRegistrarInvitationIssueOutput::Issued {
                    currency_registrar_invitation_id,
                }
            }
            CurrencyRegistrarInvitationIssueResult::Rejected { reason } => {
                CurrencyRegistrarInvitationIssueOutput::Rejected {
                    currency_registrar_invitation_id,
                    reason,
                }
            }
        };

        Ok(output)
    }
}
