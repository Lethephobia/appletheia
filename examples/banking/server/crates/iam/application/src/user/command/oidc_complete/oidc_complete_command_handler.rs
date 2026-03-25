use appletheia::application::authentication::oidc::{
    OidcCallbackParams, OidcContinuationStore, OidcLoginFlow,
};
use appletheia::application::authentication::{
    AuthTokenExchangeCodeIssueRequest, AuthTokenExchangeCodeIssuer, AuthTokenExchangeGrant,
    AuthTokenIssueRequest, AuthTokenIssuer,
};
use appletheia::application::authorization::AggregateRef;
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    Email, User, UserError, UserIdentity, UserIdentityProvider, UserIdentitySubject, UserState,
    UserStatus,
};

use crate::user::{OidcCompletionMode, OidcContinuationPayload};

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
        provider: &UserIdentityProvider,
        subject: &UserIdentitySubject,
    ) -> Result<UniqueValue, OidcCompleteCommandHandlerError> {
        let provider_part = UniqueValuePart::try_from(provider.as_ref())?;
        let subject_part = UniqueValuePart::try_from(subject.as_ref())?;

        Ok(UniqueValue::new(vec![provider_part, subject_part])?)
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

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let callback_params = OidcCallbackParams {
            state: command.state,
            authorization_code: command.authorization_code,
        };
        let continuation = self
            .oidc_continuation_store
            .consume_by_state(uow, &callback_params.state)
            .await?;
        let complete_result = self.oidc_login_flow.complete(uow, callback_params).await?;

        let provider =
            UserIdentityProvider::try_from(complete_result.id_token_claims.issuer_url.to_string())?;
        let subject = UserIdentitySubject::try_from(
            complete_result.id_token_claims.subject.value().to_owned(),
        )?;
        let email = complete_result
            .id_token_claims
            .email
            .as_ref()
            .map(|email| Email::try_from(email.value().to_owned()))
            .transpose()?;
        let identity = UserIdentity::new(provider.clone(), subject.clone(), email.clone());
        let unique_value = Self::provider_subject_unique_value(&provider, &subject)?;
        let mut user = match self
            .user_repository
            .find_by_unique_value(uow, UserState::PROVIDER_SUBJECT_KEY, &unique_value)
            .await?
        {
            Some(mut user) => {
                match user.status()? {
                    UserStatus::Active => {}
                    UserStatus::Inactive => {
                        return Err(UserError::Inactive.into());
                    }
                    UserStatus::Removed => {
                        return Err(UserError::Removed.into());
                    }
                }
                user.change_identity_email(provider.clone(), subject.clone(), email)?;
                user
            }
            None => {
                let mut user = User::default();
                user.register(identity)?;
                user
            }
        };

        self.user_repository
            .save(uow, request_context, &mut user)
            .await?;

        let subject = AggregateRef::try_from_aggregate(&user)?;
        let payload = continuation.into_payload();
        let replay_output = OidcCompleteReplayOutput {
            completion_mode: payload.completion_mode,
            completion_redirect_uri: payload.completion_redirect_uri.clone(),
        };

        let output = match payload.completion_mode {
            OidcCompletionMode::Token => {
                let result = self
                    .auth_token_issuer
                    .issue(AuthTokenIssueRequest::new(subject))
                    .await?;

                OidcCompleteOutput::Token {
                    completion_redirect_uri: payload.completion_redirect_uri,
                    auth_token: result.token().clone(),
                    auth_token_expires_in: result.expires_in()?,
                    oidc_tokens: complete_result.tokens.clone(),
                }
            }
            OidcCompletionMode::ExchangeCode => {
                let result = self
                    .auth_token_exchange_code_issuer
                    .issue(
                        uow,
                        AuthTokenExchangeCodeIssueRequest::new(
                            AuthTokenExchangeGrant::new(subject, Some(complete_result.tokens)),
                            payload.code_challenge,
                        ),
                    )
                    .await?;

                OidcCompleteOutput::ExchangeCode {
                    completion_redirect_uri: payload.completion_redirect_uri,
                    auth_token_exchange_code: result.code().clone(),
                    auth_token_exchange_code_expires_at: result.expires_at(),
                }
            }
        };

        Ok(CommandHandled::new(output, replay_output))
    }
}
