use super::PkceCodeChallengeMethod;

/// Configures whether PKCE protection is disabled or enabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PkceMode {
    Disabled,
    Enabled {
        code_challenge_method: PkceCodeChallengeMethod,
    },
}
