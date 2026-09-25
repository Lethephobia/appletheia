use crate::oidc::OidcCompletionRedirectUris;

/// Configuration for `OidcBeginCommandHandler`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OidcBeginCommandHandlerConfig {
    allowed_completion_redirect_uris: OidcCompletionRedirectUris,
}

impl OidcBeginCommandHandlerConfig {
    /// Creates an OIDC-begin command handler configuration.
    pub fn new(allowed_completion_redirect_uris: OidcCompletionRedirectUris) -> Self {
        Self {
            allowed_completion_redirect_uris,
        }
    }

    /// Returns the exact redirect URIs allowed after OIDC completion.
    pub fn allowed_completion_redirect_uris(&self) -> &OidcCompletionRedirectUris {
        &self.allowed_completion_redirect_uris
    }
}
