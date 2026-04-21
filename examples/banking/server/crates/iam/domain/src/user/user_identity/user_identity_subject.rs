use serde::{Deserialize, Serialize};

use super::user_identity_subject_error::UserIdentitySubjectError;

/// Represents the external identity subject.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct UserIdentitySubject(String);

impl UserIdentitySubject {
    /// Returns the subject value.
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for UserIdentitySubject {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl From<UserIdentitySubject> for String {
    fn from(subject: UserIdentitySubject) -> Self {
        subject.0
    }
}

impl TryFrom<String> for UserIdentitySubject {
    type Error = UserIdentitySubjectError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(UserIdentitySubjectError::Empty);
        }

        if trimmed.len() > 255 {
            return Err(UserIdentitySubjectError::TooLong);
        }

        Ok(Self(trimmed.to_owned()))
    }
}

impl TryFrom<&str> for UserIdentitySubject {
    type Error = UserIdentitySubjectError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::UserIdentitySubject;

    #[test]
    fn accepts_valid_subject() {
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");

        assert_eq!(subject.value(), "user-123");
    }

    #[test]
    fn rejects_empty_subject() {
        let error = UserIdentitySubject::try_from("   ").expect_err("subject should be invalid");

        assert!(matches!(error, super::UserIdentitySubjectError::Empty));
    }

    #[test]
    fn rejects_too_long_subject() {
        let long_value = "a".repeat(256);
        let error =
            UserIdentitySubject::try_from(long_value).expect_err("subject should be invalid");

        assert!(matches!(error, super::UserIdentitySubjectError::TooLong));
    }
}
