use serde::Serialize;

use crate::command::IdempotencyOutput;

/// Holds the result of a successfully handled command.
///
/// `CommandHandled` keeps both the immediate `output` for the current execution and the
/// replay-safe `replay_output` used for idempotent replays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandHandled<O, R> {
    output: O,
    replay_output: R,
}

impl<O, R> CommandHandled<O, R> {
    /// Creates a handled result from the immediate output and replay-safe output.
    pub fn new(output: O, replay_output: R) -> Self {
        Self {
            output,
            replay_output,
        }
    }

    /// Consumes the handled result and returns the immediate output.
    pub fn into_output(self) -> O {
        self.output
    }

    /// Serializes the replay-safe output for idempotency persistence.
    pub fn idempotency_output(&self) -> Result<IdempotencyOutput, serde_json::Error>
    where
        R: Serialize,
    {
        let value = serde_json::to_value(&self.replay_output)?;
        Ok(IdempotencyOutput::from(value))
    }
}

impl<O: Clone> CommandHandled<O, O> {
    /// Creates a handled result where the immediate and replay-safe outputs are the same.
    pub fn same(output: O) -> Self {
        Self::new(output.clone(), output)
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::CommandHandled;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
    struct Output {
        value: &'static str,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
    struct ReplayOutput {
        value: &'static str,
    }

    #[test]
    fn same_uses_replay_output_for_idempotency_output() {
        let handled = CommandHandled::same(Output { value: "replayed" });

        let replay_output = handled.idempotency_output().expect("serialize");

        assert_eq!(
            replay_output.value(),
            &serde_json::json!({
                "value": "replayed"
            })
        );
    }

    #[test]
    fn distinct_uses_replay_output_for_idempotency_output() {
        let handled =
            CommandHandled::new(Output { value: "secret" }, ReplayOutput { value: "issued" });

        let replay_output = handled.idempotency_output().expect("serialize");

        assert_eq!(
            replay_output.value(),
            &serde_json::json!({
                "value": "issued"
            })
        );
    }
}
