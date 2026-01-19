use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct SagaInstanceRow {
    pub state: serde_json::Value,
    pub state_version: i64,
    pub completed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
}

impl SagaInstanceRow {
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn is_failed(&self) -> bool {
        self.failed_at.is_some()
    }

    pub fn is_terminal(&self) -> bool {
        self.is_completed() || self.is_failed()
    }
}
