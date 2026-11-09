use crate::oidc::OidcCompletionRedirectUris;

/// Configuration for `BeginOidcFlowCommandHandler`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BeginOidcFlowCommandHandlerConfig {
    allowed_completion_redirect_uris: OidcCompletionRedirectUris,
}

impl BeginOidcFlowCommandHandlerConfig {
    /// Creates the configuration for starting an OIDC flow.
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
