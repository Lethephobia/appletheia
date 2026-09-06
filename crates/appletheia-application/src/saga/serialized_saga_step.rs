use serde::{Deserialize, Serialize};

use super::{SagaStep, SerializedSagaStepError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SerializedSagaStep(serde_json::Value);

impl SerializedSagaStep {
    pub fn new<S: SagaStep>(step: S) -> Result<Self, SerializedSagaStepError> {
        Self::try_from(serde_json::to_value(step)?)
    }

    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }

    pub fn try_into_step<S: SagaStep>(&self) -> Result<S, SerializedSagaStepError> {
        Ok(serde_json::from_value(self.0.clone())?)
    }
}

impl TryFrom<serde_json::Value> for SerializedSagaStep {
    type Error = SerializedSagaStepError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(SerializedSagaStepError::Null);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::SerializedSagaStep;
    use crate::saga::{SagaStep, SerializedSagaStepError};

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum TestSagaStep {
        FollowUp,
    }

    impl SagaStep for TestSagaStep {}

    #[test]
    fn serializes_and_deserializes_saga_step() {
        let serialized = SerializedSagaStep::new(TestSagaStep::FollowUp).expect("serialize step");

        assert_eq!(serialized.value(), &serde_json::json!("follow_up"));
        assert_eq!(
            serialized
                .try_into_step::<TestSagaStep>()
                .expect("deserialize step"),
            TestSagaStep::FollowUp
        );
    }

    #[test]
    fn rejects_null() {
        let error = SerializedSagaStep::try_from(serde_json::Value::Null)
            .expect_err("null should be rejected");

        assert!(matches!(error, SerializedSagaStepError::Null));
    }
}
