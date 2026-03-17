use std::fmt::{self, Display};
use std::str::FromStr;

use icu_locale::Locale;
use serde::{Deserialize, Serialize};

/// Represents the OIDC `locale` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OidcLocale(Locale);

impl OidcLocale {
    /// Creates an OIDC locale claim value.
    pub fn new(value: Locale) -> Self {
        Self(value)
    }

    /// Returns the claim value.
    pub fn value(&self) -> &Locale {
        &self.0
    }
}

impl Display for OidcLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for OidcLocale {
    type Err = icu_locale::ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

impl TryFrom<String> for OidcLocale {
    type Error = icu_locale::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<OidcLocale> for String {
    fn from(value: OidcLocale) -> Self {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::OidcLocale;

    #[test]
    fn accepts_valid_locale() {
        let locale = OidcLocale::try_from("ja-JP".to_owned()).expect("locale should be valid");

        assert_eq!(locale.to_string(), "ja-JP");
    }
}
