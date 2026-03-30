use appletheia::application::authentication::oidc::{
    OidcBeginOptions, OidcContinuation, OidcContinuationExpiresAt, OidcContinuationStore,
    OidcLoginFlow,
};
use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::request_context::{Principal, RequestContext};
use appletheia::domain::{Aggregate, AggregateId};
use banking_iam_domain::{User, UserId};

use crate::oidc::{OidcCompletionPurpose, OidcContinuationPayload};

use super::{OidcBeginCommand, OidcBeginCommandHandlerError, OidcBeginOutput};

/// Handles `OidcBeginCommand`.
pub struct OidcBeginCommandHandler<OLF, OCS>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
{
    oidc_login_flow: OLF,
    oidc_continuation_store: OCS,
}

impl<OLF, OCS> OidcBeginCommandHandler<OLF, OCS>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
{
    pub fn new(oidc_login_flow: OLF, oidc_continuation_store: OCS) -> Self {
        Self {
            oidc_login_flow,
            oidc_continuation_store,
        }
    }

    fn principal_user_id(
        request_context: &RequestContext,
    ) -> Result<Option<UserId>, OidcBeginCommandHandlerError> {
        let Principal::Authenticated { subject } = &request_context.principal else {
            return Ok(None);
        };

        if subject.aggregate_type.value() != User::TYPE.value() {
            return Ok(None);
        }

        UserId::try_from_uuid(subject.aggregate_id.value())
            .map(Some)
            .map_err(OidcBeginCommandHandlerError::InvalidAuthenticatedPrincipalUserId)
    }
}

impl<OLF, OCS> CommandHandler for OidcBeginCommandHandler<OLF, OCS>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
{
    type Command = OidcBeginCommand;
    type Output = OidcBeginOutput;
    type ReplayOutput = OidcBeginOutput;
    type Error = OidcBeginCommandHandlerError;
    type Uow = OLF::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        let principal_requirements = match command.completion_purpose {
            OidcCompletionPurpose::LinkIdentity => vec![PrincipalRequirement::Authenticated],
            OidcCompletionPurpose::Token | OidcCompletionPurpose::ExchangeCode => vec![
                PrincipalRequirement::Anonymous,
                PrincipalRequirement::Authenticated,
            ],
        };

        Ok(AuthorizationPlan::OnlyPrincipals(principal_requirements))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let OidcBeginCommand {
            completion_purpose,
            completion_redirect_uri,
            code_challenge,
            scopes,
            display,
            prompt,
            extra_authorize_params,
        } = command.clone();
        let options = OidcBeginOptions {
            scopes,
            display,
            max_age: None,
            prompt,
            extra_authorize_params,
        };
        let begin_result = self.oidc_login_flow.begin(uow, options).await?;

        let payload = OidcContinuationPayload {
            completion_purpose,
            completion_redirect_uri,
            code_challenge,
            principal_user_id: Self::principal_user_id(request_context)?,
        };
        let continuation = OidcContinuation::new(
            begin_result.state.clone(),
            payload,
            OidcContinuationExpiresAt::from(begin_result.expires_at),
        );
        self.oidc_continuation_store
            .save(uow, &continuation)
            .await?;

        let output =
            OidcBeginOutput::new(begin_result.authorization_url, continuation.expires_at());

        Ok(CommandHandled::same(output))
    }
}
