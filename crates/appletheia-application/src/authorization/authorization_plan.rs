use super::PrincipalRequirement;

/// Describes the authorization requirements for an application operation.
///
/// An `AuthorizationPlan` is typically returned by handlers to declare which
/// principals are allowed to execute the current action.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AuthorizationPlan {
    /// Allows the operation without requiring an authenticated principal.
    #[default]
    None,
    /// Requires the caller to satisfy at least one of the listed principal requirements.
    OnlyPrincipals(Vec<PrincipalRequirement>),
}
