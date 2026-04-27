use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use appletheia::domain::AggregateId;

use super::{UserId, UserPictureObjectNameError};

/// Represents a user picture object name in object storage.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserPictureObjectName(String);

impl UserPictureObjectName {
    /// Creates a new picture object name for the given user.
    pub fn new(user_id: UserId) -> Self {
        Self(format!(
            "users/{}/pictures/{}",
            user_id.value(),
            Uuid::now_v7()
        ))
    }

    /// Returns the picture object name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for UserPictureObjectName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for UserPictureObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for UserPictureObjectName {
    type Err = UserPictureObjectNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(UserPictureObjectNameError::Empty);
        }

        let segments = value.split('/').collect::<Vec<_>>();
        if segments.len() != 4 || segments[0] != "users" || segments[2] != "pictures" {
            return Err(UserPictureObjectNameError::InvalidFormat);
        }

        UserId::try_from_uuid(
            Uuid::parse_str(segments[1]).map_err(|_| UserPictureObjectNameError::InvalidFormat)?,
        )
        .map_err(|_| UserPictureObjectNameError::InvalidFormat)?;
        Uuid::parse_str(segments[3]).map_err(|_| UserPictureObjectNameError::InvalidFormat)?;

        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<&str> for UserPictureObjectName {
    type Error = UserPictureObjectNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for UserPictureObjectName {
    type Error = UserPictureObjectNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<UserPictureObjectName> for String {
    fn from(value: UserPictureObjectName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use appletheia::domain::AggregateId;

    use super::{UserId, UserPictureObjectName, UserPictureObjectNameError};

    #[test]
    fn new_generates_picture_object_name_for_user() {
        let user_id = UserId::try_from_uuid(Uuid::nil()).expect("user ID should be valid");
        let object_name = UserPictureObjectName::new(user_id);
        let segments = object_name.value().split('/').collect::<Vec<_>>();

        assert_eq!(segments.len(), 4);
        assert_eq!(segments[0], "users");
        assert_eq!(segments[1], "00000000-0000-0000-0000-000000000000");
        assert_eq!(segments[2], "pictures");
        Uuid::parse_str(segments[3]).expect("picture ID should be a UUID");
    }

    #[test]
    fn try_from_accepts_valid_picture_object_name() {
        let object_name = UserPictureObjectName::try_from(
            "users/00000000-0000-0000-0000-000000000001/pictures/00000000-0000-0000-0000-000000000002",
        )
        .expect("name should be valid");

        assert_eq!(
            object_name.value(),
            "users/00000000-0000-0000-0000-000000000001/pictures/00000000-0000-0000-0000-000000000002"
        );
    }

    #[test]
    fn try_from_rejects_invalid_picture_object_name() {
        let error = UserPictureObjectName::try_from("users/not-a-uuid/pictures/not-a-uuid")
            .expect_err("name should be invalid");

        assert!(matches!(error, UserPictureObjectNameError::InvalidFormat));
    }
}
