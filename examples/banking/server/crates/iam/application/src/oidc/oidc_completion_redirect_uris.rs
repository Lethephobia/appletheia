use std::collections::BTreeSet;

use super::OidcCompletionRedirectUri;

/// Represents the exact redirect URIs allowed after OIDC completion.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OidcCompletionRedirectUris(BTreeSet<OidcCompletionRedirectUri>);

impl OidcCompletionRedirectUris {
    /// Returns whether the collection contains the exact redirect URI.
    pub fn contains(&self, value: &OidcCompletionRedirectUri) -> bool {
        self.0.contains(value)
    }

    /// Returns the configured redirect URIs.
    pub fn iter(&self) -> impl Iterator<Item = &OidcCompletionRedirectUri> {
        self.0.iter()
    }
}

impl<const N: usize> From<[OidcCompletionRedirectUri; N]> for OidcCompletionRedirectUris {
    fn from(value: [OidcCompletionRedirectUri; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl FromIterator<OidcCompletionRedirectUri> for OidcCompletionRedirectUris {
    fn from_iter<T: IntoIterator<Item = OidcCompletionRedirectUri>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{OidcCompletionRedirectUri, OidcCompletionRedirectUris};

    #[test]
    fn contains_only_exact_redirect_uris() {
        let web_redirect_uri =
            OidcCompletionRedirectUri::try_from("https://app.example.com/oidc/complete".to_owned())
                .expect("web redirect URI should be valid");
        let deep_link_redirect_uri =
            OidcCompletionRedirectUri::try_from("com.example.app:/oidc/complete".to_owned())
                .expect("deep-link redirect URI should be valid");
        let redirect_uris = OidcCompletionRedirectUris::from([
            web_redirect_uri.clone(),
            deep_link_redirect_uri.clone(),
        ]);
        let different_path_redirect_uri =
            OidcCompletionRedirectUri::try_from("https://app.example.com/oidc/another".to_owned())
                .expect("different redirect URI should be valid");

        assert!(redirect_uris.contains(&web_redirect_uri));
        assert!(redirect_uris.contains(&deep_link_redirect_uri));
        assert!(!redirect_uris.contains(&different_path_redirect_uri));
    }
}
