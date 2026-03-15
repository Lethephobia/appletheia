use std::str::FromStr;

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
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "plain" => Ok(Self::Plain),
            "S256" => Ok(Self::S256),
            _ => Err("invalid pkce code challenge method"),
        }
    }
}
