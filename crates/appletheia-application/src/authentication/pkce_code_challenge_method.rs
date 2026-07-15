use std::str::FromStr;

use super::PkceCodeChallengeMethodError;

/// Declares how a PKCE code verifier is transformed into a challenge.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PkceCodeChallengeMethod {
    Plain,
    S256,
}

impl PkceCodeChallengeMethod {
    /// Returns the protocol string representation.
    pub fn value(&self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::S256 => "S256",
        }
    }
}

impl FromStr for PkceCodeChallengeMethod {
    type Err = PkceCodeChallengeMethodError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "plain" => Ok(Self::Plain),
            "S256" => Ok(Self::S256),
            _ => Err(PkceCodeChallengeMethodError::Invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PkceCodeChallengeMethod, PkceCodeChallengeMethodError};

    #[test]
    fn parses_known_methods() {
        assert_eq!("plain".parse(), Ok(PkceCodeChallengeMethod::Plain));
        assert_eq!("S256".parse(), Ok(PkceCodeChallengeMethod::S256));
    }

    #[test]
    fn rejects_unknown_method() {
        let error = "sha256"
            .parse::<PkceCodeChallengeMethod>()
            .expect_err("method should fail");

        assert_eq!(error, PkceCodeChallengeMethodError::Invalid);
    }
}
