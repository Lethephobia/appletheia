use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct SagaInstanceUpdate {
    pub state: serde_json::Value,
    pub completed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
}
