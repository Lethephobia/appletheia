use super::OidcPkceCodeChallengeMethod;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OidcPkceMode {
    Disabled,
    Enabled {
        code_challenge_method: OidcPkceCodeChallengeMethod,
    },
}
