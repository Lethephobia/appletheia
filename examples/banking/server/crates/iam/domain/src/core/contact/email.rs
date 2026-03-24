use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::EmailError;

/// Represents a validated email value.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    /// Creates an email from user input.
    pub fn new(value: String) -> Result<Self, EmailError> {
        let normalized = value.trim();

        if normalized.is_empty() {
            return Err(EmailError::Empty);
        }

        if normalized.len() > 254 {
            return Err(EmailError::TooLong);
        }

        if normalized.chars().any(char::is_whitespace) {
            return Err(EmailError::ContainsWhitespace);
        }

        let Some((local_part, domain)) = normalized.split_once('@') else {
            return Err(EmailError::MissingSeparator);
        };

        if local_part.is_empty() || domain.is_empty() {
            return Err(EmailError::InvalidFormat);
        }

        if domain.starts_with('.') || domain.ends_with('.') || !domain.contains('.') {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(normalized.to_owned()))
    }

    /// Returns the validated email.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for Email {
    type Err = EmailError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Email, EmailError};

    #[test]
    fn accepts_valid_email() {
        let email = Email::try_from("  alice@example.com ").expect("email should be valid");

        assert_eq!(email.value(), "alice@example.com");
    }

    #[test]
    fn rejects_empty_email() {
        let error = Email::try_from("   ").expect_err("empty email should fail");

        assert!(matches!(error, EmailError::Empty));
    }

    #[test]
    fn rejects_email_without_separator() {
        let error =
            Email::try_from("alice.example.com").expect_err("email without separator should fail");

        assert!(matches!(error, EmailError::MissingSeparator));
    }

    #[test]
    fn rejects_email_with_invalid_domain() {
        let error = Email::try_from("alice@example").expect_err("email with invalid domain fails");

        assert!(matches!(error, EmailError::InvalidFormat));
    }

    #[test]
    fn rejects_email_with_whitespace() {
        let error =
            Email::try_from("alice @example.com").expect_err("email with whitespace should fail");

        assert!(matches!(error, EmailError::ContainsWhitespace));
    }
}
