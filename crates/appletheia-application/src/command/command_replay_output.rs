use serde::Serialize;

/// Holds a borrowed or owned replay-safe command output.
#[derive(Serialize)]
#[serde(untagged)]
pub enum CommandReplayOutput<'a, R> {
    /// Borrows an immediate output that is itself safe to persist.
    Borrowed(&'a R),
    /// Owns a distinct replay-safe representation of an immediate output.
    Owned(R),
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::CommandReplayOutput;

    #[derive(Serialize)]
    struct TestReplayOutput {
        value: &'static str,
    }

    #[test]
    fn borrowed_and_owned_values_have_the_same_serialized_shape() {
        let replay_output = TestReplayOutput { value: "completed" };
        let borrowed = serde_json::to_value(CommandReplayOutput::Borrowed(&replay_output))
            .expect("borrowed replay output should serialize");
        let owned = serde_json::to_value(CommandReplayOutput::Owned(replay_output))
            .expect("owned replay output should serialize");

        assert_eq!(borrowed, owned);
    }
}
