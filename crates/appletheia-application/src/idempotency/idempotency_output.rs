#[derive(Clone, Debug, PartialEq)]
pub struct IdempotencyOutput(serde_json::Value);

impl IdempotencyOutput {
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }

    pub fn as_json(&self) -> &serde_json::Value {
        &self.0
    }

    pub fn into_json(self) -> serde_json::Value {
        self.0
    }
}

impl From<serde_json::Value> for IdempotencyOutput {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

impl From<IdempotencyOutput> for serde_json::Value {
    fn from(value: IdempotencyOutput) -> Self {
        value.into_json()
    }
}
