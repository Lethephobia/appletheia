use serde::{Deserialize, Serialize};

use super::{UserBio, UserDisplayName, Username};

/// Represents the onboarding state of a user's profile.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserProfile {
    Pending,
    Ready {
        username: Username,
        display_name: UserDisplayName,
        bio: Option<UserBio>,
    },
}

impl UserProfile {
    /// Returns the username when the profile is ready.
    pub fn username(&self) -> Option<&Username> {
        match self {
            Self::Pending => None,
            Self::Ready { username, .. } => Some(username),
        }
    }

    /// Returns the display name when the profile is ready.
    pub fn display_name(&self) -> Option<&UserDisplayName> {
        match self {
            Self::Pending => None,
            Self::Ready { display_name, .. } => Some(display_name),
        }
    }

    /// Returns the bio when the profile is ready.
    pub fn bio(&self) -> Option<&UserBio> {
        match self {
            Self::Pending => None,
            Self::Ready { bio, .. } => bio.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UserBio, UserDisplayName, UserProfile, Username};

    #[test]
    fn pending_profile_has_no_public_values() {
        let profile = UserProfile::Pending;

        assert_eq!(profile.username(), None);
        assert_eq!(profile.display_name(), None);
        assert_eq!(profile.bio(), None);
    }

    #[test]
    fn ready_profile_exposes_values() {
        let username = Username::try_from("alice").expect("username should be valid");
        let display_name =
            UserDisplayName::try_from("Alice Example").expect("display name should be valid");
        let profile = UserProfile::Ready {
            username: username.clone(),
            display_name: display_name.clone(),
            bio: Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid")),
        };

        assert_eq!(profile.username(), Some(&username));
        assert_eq!(profile.display_name(), Some(&display_name));
        assert_eq!(
            profile.bio().expect("bio should exist").value(),
            "Banking enthusiast"
        );
    }
}
