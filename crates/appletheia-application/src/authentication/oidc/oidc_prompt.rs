use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcPrompt {
    None,
    Login,
    Consent,
    SelectAccount,
}

impl OidcPrompt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Login => "login",
            Self::Consent => "consent",
            Self::SelectAccount => "select_account",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_matches_spec_values() {
        assert_eq!(OidcPrompt::None.as_str(), "none");
        assert_eq!(OidcPrompt::Login.as_str(), "login");
        assert_eq!(OidcPrompt::Consent.as_str(), "consent");
        assert_eq!(OidcPrompt::SelectAccount.as_str(), "select_account");
    }
}
