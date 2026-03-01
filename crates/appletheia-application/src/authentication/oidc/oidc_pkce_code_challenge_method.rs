#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OidcPkceCodeChallengeMethod {
    Plain,
    S256,
}

impl OidcPkceCodeChallengeMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::S256 => "S256",
        }
    }
}
