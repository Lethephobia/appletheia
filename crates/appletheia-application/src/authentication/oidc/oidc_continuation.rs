use serde::Serialize;
use serde::de::DeserializeOwned;

use super::{OidcContinuationConsumedAt, OidcContinuationExpiresAt, OidcState};

/// Carries application-defined continuation data across the OIDC authorization round-trip.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OidcContinuation<P>
where
    P: Serialize + DeserializeOwned,
{
    state: OidcState,
    payload: P,
    expires_at: OidcContinuationExpiresAt,
    consumed_at: Option<OidcContinuationConsumedAt>,
}

impl<P> OidcContinuation<P>
where
    P: Serialize + DeserializeOwned,
{
    /// Creates a new continuation envelope.
    pub fn new(state: OidcState, payload: P, expires_at: OidcContinuationExpiresAt) -> Self {
        Self {
            state,
            payload,
            expires_at,
            consumed_at: None,
        }
    }

    /// Reconstructs a persisted continuation envelope.
    pub fn from_persisted(
        state: OidcState,
        payload: P,
        expires_at: OidcContinuationExpiresAt,
        consumed_at: Option<OidcContinuationConsumedAt>,
    ) -> Self {
        Self {
            state,
            payload,
            expires_at,
            consumed_at,
        }
    }

    /// Returns the OIDC `state` associated with this continuation.
    pub fn state(&self) -> &OidcState {
        &self.state
    }

    /// Returns the application-defined payload.
    pub fn payload(&self) -> &P {
        &self.payload
    }

    /// Returns the application-defined payload, consuming the envelope.
    pub fn into_payload(self) -> P {
        self.payload
    }

    /// Returns when the continuation expires.
    pub fn expires_at(&self) -> OidcContinuationExpiresAt {
        self.expires_at
    }

    /// Returns when the continuation was consumed, if it has been used.
    pub fn consumed_at(&self) -> Option<OidcContinuationConsumedAt> {
        self.consumed_at
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use uuid::Uuid;

    use super::OidcContinuation;
    use crate::authentication::oidc::{
        OidcContinuationConsumedAt, OidcContinuationExpiresAt, OidcLoginAttemptExpiresAt,
        OidcLoginAttemptExpiresIn, OidcLoginAttemptStartedAt, OidcState,
    };

    #[test]
    fn new_initializes_unconsumed_continuation() {
        let state = OidcState::from(Uuid::now_v7());
        let started_at = OidcLoginAttemptStartedAt::now();
        let expires_at =
            OidcContinuationExpiresAt::from(OidcLoginAttemptExpiresAt::from_started_at(
                started_at,
                OidcLoginAttemptExpiresIn::new(Duration::minutes(5)),
            ));

        let continuation = OidcContinuation::new(state.clone(), "payload".to_string(), expires_at);

        assert_eq!(continuation.state(), &state);
        assert_eq!(continuation.payload(), "payload");
        assert_eq!(continuation.expires_at(), expires_at);
        assert_eq!(continuation.consumed_at(), None);
    }

    #[test]
    fn from_persisted_restores_consumed_at() {
        let state = OidcState::from(Uuid::now_v7());
        let started_at = OidcLoginAttemptStartedAt::now();
        let expires_at =
            OidcContinuationExpiresAt::from(OidcLoginAttemptExpiresAt::from_started_at(
                started_at,
                OidcLoginAttemptExpiresIn::new(Duration::minutes(5)),
            ));
        let consumed_at = OidcContinuationConsumedAt::now();

        let continuation = OidcContinuation::from_persisted(
            state,
            "payload".to_string(),
            expires_at,
            Some(consumed_at),
        );

        assert_eq!(continuation.consumed_at(), Some(consumed_at));
    }
}
