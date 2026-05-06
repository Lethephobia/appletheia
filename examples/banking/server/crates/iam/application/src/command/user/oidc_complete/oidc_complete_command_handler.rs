use appletheia::application::authentication::oidc::{
    OidcCallbackParams, OidcContinuationStore, OidcLoginFlow,
};
use appletheia::application::authentication::{
    AuthTokenExchangeCodeIssueRequest, AuthTokenExchangeCodeIssuer, AuthTokenExchangeGrant,
    AuthTokenIssueRequest, AuthTokenIssuer,
};
use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    Email, User, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject, UserState,
};

use crate::oidc::{OidcCompletionPurpose, OidcContinuationPayload};

use super::{
    OidcCompleteCommand, OidcCompleteCommandHandlerError, OidcCompleteOutput,
    OidcCompleteReplayOutput,
};

/// Handles `OidcCompleteCommand`.
pub struct OidcCompleteCommandHandler<OLF, OCS, UR, ATI, ATECI>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
    UR: Repository<User, Uow = OLF::Uow>,
    ATI: AuthTokenIssuer,
    ATECI: AuthTokenExchangeCodeIssuer<Uow = OLF::Uow>,
{
    oidc_login_flow: OLF,
    oidc_continuation_store: OCS,
    user_repository: UR,
    auth_token_issuer: ATI,
    auth_token_exchange_code_issuer: ATECI,
}

impl<OLF, OCS, UR, ATI, ATECI> OidcCompleteCommandHandler<OLF, OCS, UR, ATI, ATECI>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
    UR: Repository<User, Uow = OLF::Uow>,
    ATI: AuthTokenIssuer,
    ATECI: AuthTokenExchangeCodeIssuer<Uow = OLF::Uow>,
{
    pub fn new(
        oidc_login_flow: OLF,
        oidc_continuation_store: OCS,
        user_repository: UR,
        auth_token_issuer: ATI,
        auth_token_exchange_code_issuer: ATECI,
    ) -> Self {
        Self {
            oidc_login_flow,
            oidc_continuation_store,
            user_repository,
            auth_token_issuer,
            auth_token_exchange_code_issuer,
        }
    }

    fn provider_subject_unique_value(
        identity: &UserIdentity,
    ) -> Result<UniqueValue, OidcCompleteCommandHandlerError> {
        let provider_part = UniqueValuePart::try_from(identity.provider().as_ref())?;
        let subject_part = UniqueValuePart::try_from(identity.subject().as_ref())?;

        Ok(UniqueValue::new(vec![provider_part, subject_part])?)
    }

    async fn resolve_sign_in_user(
        &self,
        uow: &mut OLF::Uow,
        identity: &UserIdentity,
    ) -> Result<User, OidcCompleteCommandHandlerError> {
        let unique_value = Self::provider_subject_unique_value(identity)?;

        match self
            .user_repository
            .find_by_unique_value(uow, UserState::PROVIDER_SUBJECT_KEY, &unique_value)
            .await?
        {
            Some(mut user) => {
                user.change_identity_email(
                    identity.provider(),
                    identity.subject(),
                    identity.email().cloned(),
                )?;
                Ok(user)
            }
            None => {
                let mut user = User::default();
                user.register()?;
                user.link_identity(
                    identity.provider().clone(),
                    identity.subject().clone(),
                    identity.email().cloned(),
                )?;
                Ok(user)
            }
        }
    }

    async fn resolve_link_identity_user(
        &self,
        uow: &mut OLF::Uow,
        principal_user_id: UserId,
        identity: &UserIdentity,
    ) -> Result<User, OidcCompleteCommandHandlerError> {
        let unique_value = Self::provider_subject_unique_value(identity)?;

        match self
            .user_repository
            .find_by_unique_value(uow, UserState::PROVIDER_SUBJECT_KEY, &unique_value)
            .await?
        {
            Some(mut user) => {
                if user.aggregate_id() != Some(principal_user_id) {
                    return Err(
                        OidcCompleteCommandHandlerError::IdentityAlreadyLinkedToAnotherUser,
                    );
                }

                user.change_identity_email(
                    identity.provider(),
                    identity.subject(),
                    identity.email().cloned(),
                )?;
                Ok(user)
            }
            None => {
                let Some(mut user) = self.user_repository.find(uow, principal_user_id).await?
                else {
                    return Err(OidcCompleteCommandHandlerError::AuthenticatedUserNotFound);
                };

                user.link_identity(
                    identity.provider().clone(),
                    identity.subject().clone(),
                    identity.email().cloned(),
                )?;
                Ok(user)
            }
        }
    }
}

impl<OLF, OCS, UR, ATI, ATECI> CommandHandler
    for OidcCompleteCommandHandler<OLF, OCS, UR, ATI, ATECI>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
    UR: Repository<User, Uow = OLF::Uow>,
    ATI: AuthTokenIssuer,
    ATECI: AuthTokenExchangeCodeIssuer<Uow = OLF::Uow>,
{
    type Command = OidcCompleteCommand;
    type Output = OidcCompleteOutput;
    type ReplayOutput = OidcCompleteReplayOutput;
    type Error = OidcCompleteCommandHandlerError;
    type Uow = OLF::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::Anonymous,
            PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let callback_params = OidcCallbackParams {
            state: command.state.clone(),
            authorization_code: command.authorization_code.clone(),
        };
        let continuation = self
            .oidc_continuation_store
            .consume_by_state(uow, &callback_params.state)
            .await?;
        let complete_result = self.oidc_login_flow.complete(uow, callback_params).await?;

        let email = complete_result
            .id_token_claims
            .email
            .as_ref()
            .map(|email| Email::try_from(email.value().to_owned()))
            .transpose()?;

        let provider =
            UserIdentityProvider::try_from(complete_result.id_token_claims.issuer_url.to_string())?;
        let subject = UserIdentitySubject::try_from(
            complete_result.id_token_claims.subject.value().to_owned(),
        )?;
        let identity = UserIdentity::new(provider.clone(), subject.clone(), email.clone());

        let OidcContinuationPayload {
            completion_purpose,
            completion_redirect_uri,
            code_challenge,
            principal_user_id,
        } = continuation.into_payload();

        let mut user = match completion_purpose {
            OidcCompletionPurpose::Token | OidcCompletionPurpose::ExchangeCode => {
                self.resolve_sign_in_user(uow, &identity).await?
            }
            OidcCompletionPurpose::LinkIdentity => {
                let principal_user_id = principal_user_id.ok_or(
                    OidcCompleteCommandHandlerError::LinkIdentityRequiresAuthenticatedPrincipal,
                )?;
                self.resolve_link_identity_user(uow, principal_user_id, &identity)
                    .await?
            }
        };

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let replay_output = OidcCompleteReplayOutput {
            completion_purpose,
            completion_redirect_uri: completion_redirect_uri.clone(),
        };

        let output = match completion_purpose {
            OidcCompletionPurpose::Token => {
                let subject = AggregateRef::try_from_aggregate(&user)?;
                let result = self
                    .auth_token_issuer
                    .issue(AuthTokenIssueRequest::new(subject))
                    .await?;

                OidcCompleteOutput::Token {
                    completion_redirect_uri,
                    auth_token: result.token().clone(),
                    auth_token_expires_in: result.expires_in()?,
                    oidc_tokens: complete_result.tokens,
                }
            }
            OidcCompletionPurpose::ExchangeCode => {
                let subject = AggregateRef::try_from_aggregate(&user)?;
                let result = self
                    .auth_token_exchange_code_issuer
                    .issue(
                        uow,
                        AuthTokenExchangeCodeIssueRequest::new(
                            AuthTokenExchangeGrant::new(subject, Some(complete_result.tokens)),
                            code_challenge,
                        ),
                    )
                    .await?;

                OidcCompleteOutput::ExchangeCode {
                    completion_redirect_uri,
                    auth_token_exchange_code: result.code().clone(),
                    auth_token_exchange_code_expires_at: result.expires_at(),
                }
            }
            OidcCompletionPurpose::LinkIdentity => OidcCompleteOutput::IdentityLinked {
                completion_redirect_uri,
                oidc_tokens: complete_result.tokens,
            },
        };

        Ok(CommandHandled::new(output, replay_output))
    }
}
