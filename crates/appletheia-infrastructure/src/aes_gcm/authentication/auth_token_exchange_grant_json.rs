use appletheia_application::authentication::AuthTokenExchangeGrant;
use appletheia_application::authorization::AggregateRef;
use appletheia_application::event::{AggregateIdValue, AggregateTypeOwned};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::auth_token_exchange_oidc_tokens_json::AuthTokenExchangeOidcTokensJson;

/// Represents an exchange grant in the JSON form used by the grant cipher.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AuthTokenExchangeGrantJson {
    pub subject_aggregate_type: String,
    pub subject_aggregate_id: Uuid,
    pub oidc_tokens: Option<AuthTokenExchangeOidcTokensJson>,
}

impl AuthTokenExchangeGrantJson {
    pub(crate) fn from_grant(grant: &AuthTokenExchangeGrant) -> Self {
        Self {
            subject_aggregate_type: grant.subject().aggregate_type.value().to_owned(),
            subject_aggregate_id: grant.subject().aggregate_id.value(),
            oidc_tokens: grant
                .oidc_tokens()
                .map(AuthTokenExchangeOidcTokensJson::from_oidc_tokens),
        }
    }

    pub(crate) fn into_grant(
        self,
    ) -> Result<AuthTokenExchangeGrant, Box<dyn std::error::Error + Send + Sync>> {
        let aggregate_type = AggregateTypeOwned::new(self.subject_aggregate_type)?;
        let subject = AggregateRef {
            aggregate_type,
            aggregate_id: AggregateIdValue::from(self.subject_aggregate_id),
        };
        let oidc_tokens = self
            .oidc_tokens
            .map(AuthTokenExchangeOidcTokensJson::into_oidc_tokens)
            .transpose()?;

        Ok(AuthTokenExchangeGrant::new(subject, oidc_tokens))
    }
}
