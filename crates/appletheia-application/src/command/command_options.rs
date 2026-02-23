use super::CommandConsistency;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct CommandOptions {
    pub consistency: CommandConsistency,
}
