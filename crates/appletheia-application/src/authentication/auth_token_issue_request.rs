use serde::{Deserialize, Serialize};

use crate::aggregate::AggregateRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenIssueRequest {
    subject: AggregateRef,
}

impl AuthTokenIssueRequest {
    pub fn new(subject: AggregateRef) -> Self {
        Self { subject }
    }

    pub fn subject(&self) -> &AggregateRef {
        &self.subject
    }
}
