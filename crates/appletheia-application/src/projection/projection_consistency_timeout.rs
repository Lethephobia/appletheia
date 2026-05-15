use std::time::Duration as StdDuration;

use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::ProjectionConsistencyTimeoutError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Duration", into = "Duration")]
pub struct ProjectionConsistencyTimeout(Duration);

impl ProjectionConsistencyTimeout {
    pub fn new(value: Duration) -> Result<Self, ProjectionConsistencyTimeoutError> {
        if value < Duration::zero() {
            return Err(ProjectionConsistencyTimeoutError::Negative);
        }

        value
            .to_std()
            .map_err(|_| ProjectionConsistencyTimeoutError::OutOfRange)?;

        Ok(Self(value))
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl From<StdDuration> for ProjectionConsistencyTimeout {
    fn from(value: StdDuration) -> Self {
        let duration = Duration::from_std(value)
            .expect("std::time::Duration should fit within chrono::Duration");
        Self::new(duration).expect("std::time::Duration should be a valid timeout")
    }
}

impl TryFrom<Duration> for ProjectionConsistencyTimeout {
    type Error = ProjectionConsistencyTimeoutError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ProjectionConsistencyTimeout> for Duration {
    fn from(value: ProjectionConsistencyTimeout) -> Self {
        value.value()
    }
}

impl From<ProjectionConsistencyTimeout> for StdDuration {
    fn from(value: ProjectionConsistencyTimeout) -> Self {
        value.value().to_std().expect(
            "validated projection consistency timeout should fit within std::time::Duration",
        )
    }
}
