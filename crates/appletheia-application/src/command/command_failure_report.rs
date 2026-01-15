use std::error::Error;
use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandFailureReport {
    pub message: String,
    pub chain: Vec<String>,
}

impl CommandFailureReport {
    pub const DEFAULT_MAX_DEPTH: usize = 16;

    pub fn from_error_with_max_depth(error: &(dyn Error + 'static), max_depth: usize) -> Self {
        let mut chain = Vec::new();

        let mut current: Option<&(dyn Error + 'static)> = Some(error);
        let mut depth = 0usize;
        while let Some(err) = current {
            if depth >= max_depth {
                break;
            }
            chain.push(err.to_string());
            current = err.source();
            depth += 1;
        }

        let message = chain.first().cloned().unwrap_or_default();
        Self { message, chain }
    }
}

impl<E> From<&E> for CommandFailureReport
where
    E: Error + 'static,
{
    fn from(value: &E) -> Self {
        Self::from_error_with_max_depth(value, Self::DEFAULT_MAX_DEPTH)
    }
}

impl Display for CommandFailureReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
