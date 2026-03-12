use super::PrincipalRequirement;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AuthorizationPlan {
    #[default]
    None,
    OnlyPrincipals(Vec<PrincipalRequirement>),
}
