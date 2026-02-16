use super::{AuthorizationAction, RelationName};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorizationRule {
    AllowAnonymous,
    AllowAuthenticated,
    RequireRelationOnResource { relation: RelationName },
    RequireAnyRelationOnResource { relations: Vec<RelationName> },
}

pub trait AuthorizationPolicy: Send + Sync {
    fn rule_for(&self, action: AuthorizationAction) -> AuthorizationRule;
}

