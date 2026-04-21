use serde::{Deserialize, Serialize};
use url::Url;

use super::UserPictureUrlError;

/// Represents a user's public picture URL.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct UserPictureUrl(Url);

impl UserPictureUrl {
    /// Creates a new user picture URL.
    pub fn new(value: Url) -> Self {
        Self(value)
    }

    /// Returns the picture URL value.
    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl TryFrom<String> for UserPictureUrl {
    type Error = UserPictureUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(Url::parse(&value)?))
    }
}

impl TryFrom<&str> for UserPictureUrl {
    type Error = UserPictureUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl From<UserPictureUrl> for String {
    fn from(value: UserPictureUrl) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::UserPictureUrl;

    #[test]
    fn accepts_valid_picture_url() {
        let picture_url = UserPictureUrl::try_from("https://cdn.example.com/alice.png")
            .expect("picture URL should be valid");

        assert_eq!(
            picture_url.value().as_str(),
            "https://cdn.example.com/alice.png"
        );
    }
}
