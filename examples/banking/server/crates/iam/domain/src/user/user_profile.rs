use serde::{Deserialize, Serialize};

use super::{UserBio, UserDisplayName, UserPictureUrl};

/// Stores the public profile information for a user.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserProfile {
    display_name: UserDisplayName,
    bio: Option<UserBio>,
    picture_url: Option<UserPictureUrl>,
}

impl UserProfile {
    /// Creates a new user profile.
    pub fn new(
        display_name: UserDisplayName,
        bio: Option<UserBio>,
        picture_url: Option<UserPictureUrl>,
    ) -> Self {
        Self {
            display_name,
            bio,
            picture_url,
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

    /// Returns the picture URL.
    pub fn picture_url(&self) -> Option<&UserPictureUrl> {
        self.picture_url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{UserBio, UserDisplayName, UserPictureUrl, UserProfile};

    #[test]
    fn exposes_stored_values() {
        let profile = UserProfile::new(
            UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid")),
            Some(
                UserPictureUrl::try_from("https://cdn.example.com/alice.png")
                    .expect("picture URL should be valid"),
            ),
        );

        assert_eq!(profile.display_name().value(), "Alice Example");
        assert_eq!(
            profile.bio().expect("bio should exist").value(),
            "Banking enthusiast"
        );
        assert_eq!(
            profile
                .picture_url()
                .expect("picture should exist")
                .value()
                .as_str(),
            "https://cdn.example.com/alice.png"
        );
    }
}
