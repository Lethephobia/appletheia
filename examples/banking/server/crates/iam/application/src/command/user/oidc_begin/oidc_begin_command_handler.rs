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

use super::{
    OidcBeginCommand, OidcBeginCommandHandlerConfig, OidcBeginCommandHandlerError, OidcBeginOutput,
};

/// Handles `OidcBeginCommand`.
pub struct OidcBeginCommandHandler<OLF, OCS>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
{
    oidc_login_flow: OLF,
    oidc_continuation_store: OCS,
    config: OidcBeginCommandHandlerConfig,
}

impl<OLF, OCS> OidcBeginCommandHandler<OLF, OCS>
where
    OLF: OidcLoginFlow,
    OCS: OidcContinuationStore<OidcContinuationPayload, Uow = OLF::Uow>,
{
    pub fn new(
        oidc_login_flow: OLF,
        oidc_continuation_store: OCS,
        config: OidcBeginCommandHandlerConfig,
    ) -> Self {
        Self {
            oidc_login_flow,
            oidc_continuation_store,
            config,
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

        if !self
            .config
            .allowed_completion_redirect_uris()
            .contains(&completion_redirect_uri)
        {
            return Err(OidcBeginCommandHandlerError::CompletionRedirectUriNotAllowed);
        }

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

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use appletheia::application::Retryability;
    use appletheia::application::authentication::oidc::{
        OidcAuthorizationUrl, OidcBeginOptions, OidcBeginResult, OidcCallbackParams,
        OidcCompleteResult, OidcContinuation, OidcContinuationStore, OidcContinuationStoreError,
        OidcExtraAuthorizeParams, OidcLoginAttemptExpiresAt, OidcLoginFlow, OidcLoginFlowError,
        OidcScopes, OidcState,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use chrono::Utc;
    use url::Url;
    use uuid::Uuid;

    use crate::oidc::{
        OidcCompletionPurpose, OidcCompletionRedirectUri, OidcCompletionRedirectUris,
        OidcContinuationPayload,
    };

    use super::{
        OidcBeginCommand, OidcBeginCommandHandler, OidcBeginCommandHandlerConfig,
        OidcBeginCommandHandlerError, OidcBeginOutput,
    };

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestOidcLoginFlow {
        begin_calls: Arc<AtomicUsize>,
        begin_result: OidcBeginResult,
    }

    impl TestOidcLoginFlow {
        fn new(begin_result: OidcBeginResult) -> Self {
            Self {
                begin_calls: Arc::new(AtomicUsize::new(0)),
                begin_result,
            }
        }
    }

    impl OidcLoginFlow for TestOidcLoginFlow {
        type Uow = TestUow;

        async fn begin(
            &self,
            _uow: &mut Self::Uow,
            _options: OidcBeginOptions,
        ) -> Result<OidcBeginResult, OidcLoginFlowError> {
            self.begin_calls.fetch_add(1, Ordering::Relaxed);
            Ok(self.begin_result.clone())
        }

        async fn complete(
            &self,
            _uow: &mut Self::Uow,
            _callback_params: OidcCallbackParams,
        ) -> Result<OidcCompleteResult, OidcLoginFlowError> {
            panic!("complete should not be called by the begin handler")
        }
    }

    #[derive(Clone, Default)]
    struct TestOidcContinuationStore {
        saved: Arc<Mutex<Vec<OidcContinuation<OidcContinuationPayload>>>>,
    }

    impl OidcContinuationStore<OidcContinuationPayload> for TestOidcContinuationStore {
        type Uow = TestUow;

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            continuation: &OidcContinuation<OidcContinuationPayload>,
        ) -> Result<(), OidcContinuationStoreError> {
            self.saved
                .lock()
                .expect("lock should be available")
                .push(continuation.clone());
            Ok(())
        }

        async fn consume_by_state(
            &self,
            _uow: &mut Self::Uow,
            _state: &OidcState,
        ) -> Result<OidcContinuation<OidcContinuationPayload>, OidcContinuationStoreError> {
            panic!("consume should not be called by the begin handler")
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Anonymous,
        )
        .expect("request context should be valid")
    }

    fn completion_redirect_uri(value: &str) -> OidcCompletionRedirectUri {
        OidcCompletionRedirectUri::try_from(value.to_owned())
            .expect("completion redirect URI should be valid")
    }

    fn command(completion_redirect_uri: OidcCompletionRedirectUri) -> OidcBeginCommand {
        OidcBeginCommand {
            completion_purpose: OidcCompletionPurpose::ExchangeCode,
            completion_redirect_uri,
            code_challenge: None,
            scopes: OidcScopes::default(),
            display: None,
            prompt: None,
            extra_authorize_params: OidcExtraAuthorizeParams::default(),
        }
    }

    fn begin_result() -> OidcBeginResult {
        OidcBeginResult {
            authorization_url: OidcAuthorizationUrl::new(
                Url::parse("https://accounts.example.com/authorize")
                    .expect("authorization URL should be valid"),
            ),
            state: OidcState::new(),
            expires_at: OidcLoginAttemptExpiresAt::from(Utc::now()),
        }
    }

    #[tokio::test]
    async fn handle_accepts_an_exactly_allowed_completion_redirect_uri() {
        let allowed_redirect_uri = completion_redirect_uri("com.example.app:/oidc/complete");
        let config = OidcBeginCommandHandlerConfig::new(OidcCompletionRedirectUris::from([
            allowed_redirect_uri.clone(),
        ]));
        let login_flow = TestOidcLoginFlow::new(begin_result());
        let begin_calls = login_flow.begin_calls.clone();
        let continuation_store = TestOidcContinuationStore::default();
        let saved_continuations = continuation_store.saved.clone();
        let handler = OidcBeginCommandHandler::new(login_flow, continuation_store, config);
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &command(allowed_redirect_uri.clone()),
            )
            .await
            .expect("allowed completion redirect URI should be accepted");

        let output = handled.into_output();
        let saved = saved_continuations
            .lock()
            .expect("lock should be available");
        assert_eq!(begin_calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            output,
            OidcBeginOutput {
                authorization_url: OidcAuthorizationUrl::new(
                    Url::parse("https://accounts.example.com/authorize")
                        .expect("authorization URL should be valid"),
                ),
                expires_at: saved[0].expires_at(),
            }
        );
        assert_eq!(
            saved[0].payload().completion_redirect_uri,
            allowed_redirect_uri
        );
    }

    #[tokio::test]
    async fn handle_rejects_a_completion_redirect_uri_before_beginning_the_flow() {
        let allowed_redirect_uri = completion_redirect_uri("https://app.example.com/oidc/complete");
        let config = OidcBeginCommandHandlerConfig::new(OidcCompletionRedirectUris::from([
            allowed_redirect_uri,
        ]));
        let login_flow = TestOidcLoginFlow::new(begin_result());
        let begin_calls = login_flow.begin_calls.clone();
        let continuation_store = TestOidcContinuationStore::default();
        let saved_continuations = continuation_store.saved.clone();
        let handler = OidcBeginCommandHandler::new(login_flow, continuation_store, config);
        let mut uow = TestUow;

        let handle_error = handler
            .handle(
                &mut uow,
                &request_context(),
                &command(completion_redirect_uri(
                    "https://app.example.com/oidc/another",
                )),
            )
            .await
            .expect_err("unlisted completion redirect URI should be rejected");

        assert!(matches!(
            handle_error,
            OidcBeginCommandHandlerError::CompletionRedirectUriNotAllowed
        ));
        assert!(!handle_error.is_retryable());
        assert_eq!(begin_calls.load(Ordering::Relaxed), 0);
        assert!(
            saved_continuations
                .lock()
                .expect("lock should be available")
                .is_empty()
        );
    }
}
