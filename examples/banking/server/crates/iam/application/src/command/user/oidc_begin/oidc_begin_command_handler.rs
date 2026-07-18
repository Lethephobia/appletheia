use appletheia::application::authentication::oidc::{
    OidcBeginOptions, OidcContinuation, OidcContinuationExpiresAt, OidcContinuationStore,
    OidcLoginFlow,
};
use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::User;

use crate::authorization::UserOwnerRelation;
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
            OidcCompletionPurpose::LinkIdentity { user_id } => {
                vec![PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                )]
            }
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
        _request_context: &RequestContext,
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
        };
        let continuation = OidcContinuation::new(
            begin_result.state.clone(),
            payload,
            OidcContinuationExpiresAt::from(begin_result.expires_at),
        );
        self.oidc_continuation_store
            .save(uow, &continuation)
            .await?;

        let output = OidcBeginOutput {
            authorization_url: begin_result.authorization_url,
            expires_at: continuation.expires_at(),
        };

        Ok(CommandHandled::same(output))
    }
}
