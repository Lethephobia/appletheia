/// Represents an encrypted auth token exchange grant ready for persistence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncryptedAuthTokenExchangeGrant(Vec<u8>);

impl EncryptedAuthTokenExchangeGrant {
    /// Creates a new encrypted grant value.
    pub fn new(value: Vec<u8>) -> Self {
        Self(value)
    }

    /// Returns the encrypted grant bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consumes the value and returns the underlying bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}
