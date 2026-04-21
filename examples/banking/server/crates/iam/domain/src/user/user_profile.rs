use serde::{Deserialize, Serialize};

use super::{UserBio, UserDisplayName, UserPictureRef};

/// Stores the public profile information for a user.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserProfile {
    display_name: UserDisplayName,
    bio: Option<UserBio>,
    picture: Option<UserPictureRef>,
}

impl UserProfile {
    /// Creates a new user profile.
    pub fn new(
        display_name: UserDisplayName,
        bio: Option<UserBio>,
        picture: Option<UserPictureRef>,
    ) -> Self {
        Self {
            display_name,
            bio,
            picture,
        }
    }

    /// Returns the display name.
    pub fn display_name(&self) -> &UserDisplayName {
        &self.display_name
    }

    /// Returns the bio.
    pub fn bio(&self) -> Option<&UserBio> {
        self.bio.as_ref()
    }

    /// Returns the picture.
    pub fn picture(&self) -> Option<&UserPictureRef> {
        self.picture.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::UserPictureObjectName;

    use super::{UserBio, UserDisplayName, UserPictureRef, UserProfile};

    #[test]
    fn exposes_stored_values() {
        let profile = UserProfile::new(
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid")),
            Some(UserPictureRef::object_name(
                UserPictureObjectName::try_from(
                    "users/00000000-0000-0000-0000-000000000001/picture",
                )
                .expect("picture object name should be valid"),
            )),
        );

        assert_eq!(profile.display_name().value(), "Alice Example");
        assert_eq!(
            profile.bio().expect("bio should exist").value(),
            "Banking enthusiast"
        );
        assert_eq!(
            profile
                .picture()
                .expect("picture should exist")
                .as_object_name()
                .expect("picture should be stored in object storage")
                .value(),
            "users/00000000-0000-0000-0000-000000000001/picture"
        );
    }
}
