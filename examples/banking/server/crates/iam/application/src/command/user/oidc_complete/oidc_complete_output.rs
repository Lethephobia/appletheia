use appletheia::application::authentication::oidc::OidcTokens;
use appletheia::application::authentication::{
    AuthToken, AuthTokenExchangeCode, AuthTokenExchangeCodeExpiresAt, AuthTokenExpiresIn,
};
use appletheia::application::command::{CommandOutput, CommandReplayOutput};
use banking_iam_domain::UserId;

use crate::oidc::{OidcCompletionPurpose, OidcCompletionRedirectUri, OidcReturnTo};

use super::{OidcCompleteRejectionReason, OidcCompleteReplayOutput};

/// Represents the result returned after completing an OIDC flow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OidcCompleteOutput {
    Token {
        completion_redirect_uri: OidcCompletionRedirectUri,
        return_to: Option<OidcReturnTo>,
        auth_token: AuthToken,
        auth_token_expires_in: AuthTokenExpiresIn,
        oidc_tokens: OidcTokens,
    },
    ExchangeCode {
        completion_redirect_uri: OidcCompletionRedirectUri,
        return_to: Option<OidcReturnTo>,
        auth_token_exchange_code: AuthTokenExchangeCode,
        auth_token_exchange_code_expires_at: AuthTokenExchangeCodeExpiresAt,
    },
    IdentityLinked {
        user_id: UserId,
        completion_redirect_uri: OidcCompletionRedirectUri,
        return_to: Option<OidcReturnTo>,
        oidc_tokens: OidcTokens,
    },
    Rejected {
        completion_purpose: OidcCompletionPurpose,
        completion_redirect_uri: OidcCompletionRedirectUri,
        return_to: Option<OidcReturnTo>,
        reason: OidcCompleteRejectionReason,
    },
}

impl OidcCompleteOutput {
    fn replay_safe_output(&self) -> OidcCompleteReplayOutput {
        match self {
            Self::Token {
                completion_redirect_uri,
                return_to,
                ..
            } => OidcCompleteReplayOutput {
                completion_purpose: OidcCompletionPurpose::Token,
                completion_redirect_uri: completion_redirect_uri.clone(),
                return_to: return_to.clone(),
                rejection_reason: None,
            },
            Self::ExchangeCode {
                completion_redirect_uri,
                return_to,
                ..
            } => OidcCompleteReplayOutput {
                completion_purpose: OidcCompletionPurpose::ExchangeCode,
                completion_redirect_uri: completion_redirect_uri.clone(),
                return_to: return_to.clone(),
                rejection_reason: None,
            },
            Self::IdentityLinked {
                user_id,
                completion_redirect_uri,
                return_to,
                ..
            } => OidcCompleteReplayOutput {
                completion_purpose: OidcCompletionPurpose::LinkIdentity { user_id: *user_id },
                completion_redirect_uri: completion_redirect_uri.clone(),
                return_to: return_to.clone(),
                rejection_reason: None,
            },
            Self::Rejected {
                completion_purpose,
                completion_redirect_uri,
                return_to,
                reason,
            } => OidcCompleteReplayOutput {
                completion_purpose: *completion_purpose,
                completion_redirect_uri: completion_redirect_uri.clone(),
                return_to: return_to.clone(),
                rejection_reason: Some(reason.clone()),
            },
        }
    }
}

impl CommandOutput for OidcCompleteOutput {
    type ReplayOutput = OidcCompleteReplayOutput;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Owned(self.replay_safe_output())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authentication::oidc::{
        OidcAccessToken, OidcIdToken, OidcRefreshToken, OidcTokens,
    };
    use appletheia::application::authentication::{
        AuthToken, AuthTokenExchangeCode, AuthTokenExchangeCodeExpiresAt, AuthTokenExpiresIn,
    };
    use appletheia::application::command::CommandOutput;
    use banking_iam_domain::UserId;
    use banking_iam_domain::user::UserIdentityLinkRejectionReason;
    use chrono::{Duration, Utc};

    use super::{OidcCompleteOutput, OidcCompleteRejectionReason, OidcCompleteReplayOutput};
    use crate::oidc::{OidcCompletionPurpose, OidcCompletionRedirectUri, OidcReturnTo};

    #[test]
    fn idempotency_outputs_exclude_authentication_secrets() {
        let auth_token_secret = "auth-token-secret";
        let oidc_id_token_secret = "oidc-id-token-secret";
        let oidc_access_token_secret = "oidc-access-token-secret";
        let oidc_refresh_token_secret = "oidc-refresh-token-secret";
        let exchange_code = AuthTokenExchangeCode::new();
        let exchange_code_secret = exchange_code.value().to_owned();
        let completion_redirect_uri =
            OidcCompletionRedirectUri::try_from("https://client.example.test/complete".to_owned())
                .expect("completion redirect URI should be valid");
        let expected_completion_redirect_uri = completion_redirect_uri.clone();
        let return_to = OidcReturnTo::try_from("/settings/identities")
            .expect("return destination should be valid");
        let expected_return_to = return_to.clone();
        let oidc_tokens = OidcTokens::new(
            OidcIdToken::new(oidc_id_token_secret.to_owned()),
            Some(OidcAccessToken::new(oidc_access_token_secret.to_owned())),
            Some(OidcRefreshToken::new(oidc_refresh_token_secret.to_owned())),
            None,
        );

        let token_output = OidcCompleteOutput::Token {
            completion_redirect_uri: completion_redirect_uri.clone(),
            return_to: Some(return_to.clone()),
            auth_token: AuthToken::new(auth_token_secret.to_owned()),
            auth_token_expires_in: AuthTokenExpiresIn::new(Duration::minutes(5))
                .expect("auth token expiry should be valid"),
            oidc_tokens: oidc_tokens.clone(),
        };
        let exchange_code_output = OidcCompleteOutput::ExchangeCode {
            completion_redirect_uri: completion_redirect_uri.clone(),
            return_to: Some(return_to.clone()),
            auth_token_exchange_code: exchange_code,
            auth_token_exchange_code_expires_at: AuthTokenExchangeCodeExpiresAt::from(Utc::now()),
        };
        let user_id = UserId::new();
        let identity_linked_output = OidcCompleteOutput::IdentityLinked {
            user_id,
            completion_redirect_uri: completion_redirect_uri.clone(),
            return_to: Some(return_to.clone()),
            oidc_tokens,
        };
        let rejection_reason = OidcCompleteRejectionReason::IdentityLink {
            reason: UserIdentityLinkRejectionReason::AlreadyLinked,
        };
        let rejected_output = OidcCompleteOutput::Rejected {
            completion_purpose: OidcCompletionPurpose::LinkIdentity { user_id },
            completion_redirect_uri: completion_redirect_uri.clone(),
            return_to: Some(return_to),
            reason: rejection_reason.clone(),
        };

        let token_json = serde_json::to_string(&token_output.replay_output())
            .expect("token replay output should serialize");
        let exchange_code_json = serde_json::to_string(&exchange_code_output.replay_output())
            .expect("exchange-code replay output should serialize");
        let identity_linked_json = serde_json::to_string(&identity_linked_output.replay_output())
            .expect("identity-linked replay output should serialize");
        let rejected_json = serde_json::to_string(&rejected_output.replay_output())
            .expect("rejected replay output should serialize");
        let token_replay_output: OidcCompleteReplayOutput =
            serde_json::from_str(&token_json).expect("token replay output should deserialize");
        let identity_linked_replay_output: OidcCompleteReplayOutput =
            serde_json::from_str(&identity_linked_json)
                .expect("identity-linked replay output should deserialize");
        let rejected_replay_output: OidcCompleteReplayOutput = serde_json::from_str(&rejected_json)
            .expect("rejected replay output should deserialize");

        assert_eq!(
            token_replay_output,
            OidcCompleteReplayOutput {
                completion_purpose: OidcCompletionPurpose::Token,
                completion_redirect_uri: expected_completion_redirect_uri,
                return_to: Some(expected_return_to.clone()),
                rejection_reason: None,
            }
        );
        assert_eq!(
            identity_linked_replay_output,
            OidcCompleteReplayOutput {
                completion_purpose: OidcCompletionPurpose::LinkIdentity { user_id },
                completion_redirect_uri: completion_redirect_uri.clone(),
                return_to: Some(expected_return_to.clone()),
                rejection_reason: None,
            }
        );
        assert_eq!(
            rejected_replay_output,
            OidcCompleteReplayOutput {
                completion_purpose: OidcCompletionPurpose::LinkIdentity { user_id },
                completion_redirect_uri,
                return_to: Some(expected_return_to),
                rejection_reason: Some(rejection_reason),
            }
        );
        for secret in [
            auth_token_secret,
            oidc_id_token_secret,
            oidc_access_token_secret,
            oidc_refresh_token_secret,
        ] {
            assert!(!token_json.contains(secret));
            assert!(!identity_linked_json.contains(secret));
        }
        assert!(!exchange_code_json.contains(&exchange_code_secret));
    }
}
