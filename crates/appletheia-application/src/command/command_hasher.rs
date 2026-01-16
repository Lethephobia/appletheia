use crate::command::CommandHash;

pub trait CommandHasher: Send + Sync {
    fn command_hash(&self, value: serde_json::Value) -> CommandHash;
}
