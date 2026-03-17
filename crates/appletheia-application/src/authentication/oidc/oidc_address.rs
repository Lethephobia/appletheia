use serde::{Deserialize, Serialize};

/// Represents the OIDC `address` standard claim.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcAddress {
    pub formatted: Option<String>,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

impl OidcAddress {
    /// Creates an OIDC address claim value.
    pub fn new(
        formatted: Option<String>,
        street_address: Option<String>,
        locality: Option<String>,
        region: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
    ) -> Self {
        Self {
            formatted,
            street_address,
            locality,
            region,
            postal_code,
            country,
        }
    }
}
