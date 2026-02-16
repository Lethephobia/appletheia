use super::{AggregateRef, AuthorizationAction};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AuthorizationRequest {
    pub action: AuthorizationAction,
    pub resource: Option<AggregateRef>,
}
