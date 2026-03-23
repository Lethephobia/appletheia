use serde::{Deserialize, Serialize};

use super::UserIdentityProviderError;

/// Represents the external identity provider identifier.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct UserIdentityProvider(String);

impl UserIdentityProvider {
    /// Returns the provider identifier value.
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for UserIdentityProvider {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl From<UserIdentityProvider> for String {
    fn from(provider: UserIdentityProvider) -> Self {
        provider.0
    }
}

impl TryFrom<String> for UserIdentityProvider {
    type Error = UserIdentityProviderError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(UserIdentityProviderError::Empty);
        }

        if trimmed.len() > 255 {
            return Err(UserIdentityProviderError::TooLong);
        }

        Ok(Self(trimmed.to_owned()))
    }
}

impl TryFrom<&str> for UserIdentityProvider {
    type Error = UserIdentityProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::UserIdentityProvider;

    #[test]
    fn accepts_valid_provider() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");

        assert_eq!(provider.value(), "https://accounts.example.com");
    }

    #[test]
    fn rejects_empty_provider() {
        let error = UserIdentityProvider::try_from("   ").expect_err("provider should be invalid");

        assert!(matches!(error, super::UserIdentityProviderError::Empty));
    }

    #[test]
    fn rejects_too_long_provider() {
        let long_value = "a".repeat(256);
        let error =
            UserIdentityProvider::try_from(long_value).expect_err("provider should be invalid");

        assert!(matches!(error, super::UserIdentityProviderError::TooLong));
    }
}
