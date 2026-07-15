use std::time::Duration as StdDuration;

use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::ProjectionConsistencyPollIntervalError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Duration", into = "Duration")]
pub struct ProjectionConsistencyPollInterval(Duration);

impl ProjectionConsistencyPollInterval {
    pub fn new(value: Duration) -> Result<Self, ProjectionConsistencyPollIntervalError> {
        if value < Duration::zero() {
            return Err(ProjectionConsistencyPollIntervalError::Negative);
        }

        value
            .to_std()
            .map_err(|_| ProjectionConsistencyPollIntervalError::OutOfRange)?;

        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<StdDuration> for ProjectionConsistencyPollInterval {
    fn from(value: StdDuration) -> Self {
        let duration = Duration::from_std(value)
            .expect("std::time::Duration should fit within chrono::Duration");
        Self::new(duration).expect("std::time::Duration should be a valid poll interval")
    }
}

impl TryFrom<Duration> for ProjectionConsistencyPollInterval {
    type Error = ProjectionConsistencyPollIntervalError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ProjectionConsistencyPollInterval> for Duration {
    fn from(value: ProjectionConsistencyPollInterval) -> Self {
        value.value()
    }
}

impl From<ProjectionConsistencyPollInterval> for StdDuration {
    fn from(value: ProjectionConsistencyPollInterval) -> Self {
        value.value().to_std().expect(
            "validated projection consistency poll interval should fit within std::time::Duration",
        )
    }
}
