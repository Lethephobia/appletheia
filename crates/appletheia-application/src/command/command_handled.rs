use serde::Serialize;

use crate::command::IdempotencyOutput;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandHandled<O, R> {
    Same(O),
    Distinct { output: O, replay_output: R },
}

impl<O, R> CommandHandled<O, R> {
    pub fn new(output: O, replay_output: R) -> Self {
        Self::Distinct {
            output,
            replay_output,
        }
    }

    pub fn into_output(self) -> O {
        match self {
            Self::Same(output) => output,
            Self::Distinct { output, .. } => output,
        }
    }

    pub fn idempotency_output(&self) -> Result<IdempotencyOutput, serde_json::Error>
    where
        O: Serialize,
        R: Serialize,
    {
        let value = match self {
            Self::Same(output) => serde_json::to_value(output)?,
            Self::Distinct { replay_output, .. } => serde_json::to_value(replay_output)?,
        };
        Ok(IdempotencyOutput::from(value))
    }
}

impl<O> CommandHandled<O, O> {
    pub fn same(output: O) -> Self {
        Self::Same(output)
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
    fn same_uses_output_for_idempotency_output() {
        let handled = CommandHandled::same(Output { value: "ok" });

        let replay_output = handled.idempotency_output().expect("serialize");

        assert_eq!(
            replay_output.value(),
            &serde_json::json!({
                "value": "ok"
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
