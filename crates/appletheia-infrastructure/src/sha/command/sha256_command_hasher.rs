use sha2::{Digest, Sha256};

use appletheia_application::command::{Command, CommandHash, CommandHasher, CommandHasherError};
use appletheia_application::json::CanonicalJson;

/// Hashes commands with SHA-256 over canonicalized JSON.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sha256CommandHasher;

impl Sha256CommandHasher {
    pub fn new() -> Self {
        Self
    }

    fn to_lower_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

impl CommandHasher for Sha256CommandHasher {
    fn command_hash<C: Command>(&self, command: &C) -> Result<CommandHash, CommandHasherError> {
        let json = CanonicalJson::try_from_serializable(command)?;

        let mut hasher = Sha256::new();
        hasher.update(json.as_str().as_bytes());
        let hash = Self::to_lower_hex(&hasher.finalize());
        Ok(CommandHash::new(hash)?)
    }
}
