#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OidcResponseType {
    Code,
}

impl OidcResponseType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Code => "code",
        }
    }
}
