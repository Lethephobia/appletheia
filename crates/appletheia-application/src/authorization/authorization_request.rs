use super::{AuthorizationAction, ResourceRef};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AuthorizationRequest {
    pub action: AuthorizationAction,
    pub resource: Option<ResourceRef>,
}

