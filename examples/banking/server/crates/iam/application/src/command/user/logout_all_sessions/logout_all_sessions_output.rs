use serde::{Deserialize, Serialize};

/// Returned after a logout-all request advances the subject-wide revocation cutoff.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogoutAllSessionsOutput;
