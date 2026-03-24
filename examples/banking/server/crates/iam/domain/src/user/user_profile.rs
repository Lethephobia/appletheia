use serde::{Deserialize, Serialize};

use super::{UserDisplayName, Username};

/// Represents the onboarding state of a user's profile.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum UserProfile {
    Pending,
    Ready {
        username: Username,
        display_name: UserDisplayName,
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
}

#[cfg(test)]
mod tests {
    use super::{UserDisplayName, UserProfile, Username};

    #[test]
    fn pending_profile_has_no_public_values() {
        let profile = UserProfile::Pending;

        assert_eq!(profile.username(), None);
        assert_eq!(profile.display_name(), None);
    }

    #[test]
    fn ready_profile_exposes_values() {
        let username = Username::try_from("alice").expect("username should be valid");
        let display_name =
            UserDisplayName::try_from("Alice Example").expect("display name should be valid");
        let profile = UserProfile::Ready {
            username: username.clone(),
            display_name: display_name.clone(),
        };

        assert_eq!(profile.username(), Some(&username));
        assert_eq!(profile.display_name(), Some(&display_name));
    }
}
