use super::{
    OutboxAttemptCount, OutboxLeaseExpiresAt, OutboxNextAttemptAt, OutboxPublishedAt,
    OutboxRelayInstance,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutboxState {
    Pending {
        attempt_count: OutboxAttemptCount,
        next_attempt_after: OutboxNextAttemptAt,
    },
    Leased {
        attempt_count: OutboxAttemptCount,
        next_attempt_after: OutboxNextAttemptAt,
        lease_owner: OutboxRelayInstance,
        lease_until: OutboxLeaseExpiresAt,
    },
    Published {
        published_at: OutboxPublishedAt,
        attempt_count: OutboxAttemptCount,
    },
}

impl OutboxState {
    pub fn attempt_count(&self) -> OutboxAttemptCount {
        match self {
            OutboxState::Pending { attempt_count, .. }
            | OutboxState::Leased { attempt_count, .. }
            | OutboxState::Published { attempt_count, .. } => *attempt_count,
        }
    }
}
