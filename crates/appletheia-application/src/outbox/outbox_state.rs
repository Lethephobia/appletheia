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

    pub fn next_attempt_after(&self) -> Option<OutboxNextAttemptAt> {
        match self {
            OutboxState::Pending {
                next_attempt_after, ..
            }
            | OutboxState::Leased {
                next_attempt_after, ..
            } => Some(*next_attempt_after),
            OutboxState::Published { .. } => None,
        }
    }

    pub fn lease_owner(&self) -> Option<&OutboxRelayInstance> {
        match self {
            OutboxState::Leased { lease_owner, .. } => Some(lease_owner),
            OutboxState::Pending { .. } | OutboxState::Published { .. } => None,
        }
    }

    pub fn lease_until(&self) -> Option<OutboxLeaseExpiresAt> {
        match self {
            OutboxState::Leased { lease_until, .. } => Some(*lease_until),
            OutboxState::Pending { .. } | OutboxState::Published { .. } => None,
        }
    }

    pub fn published_at(&self) -> Option<OutboxPublishedAt> {
        match self {
            OutboxState::Published { published_at, .. } => Some(*published_at),
            OutboxState::Pending { .. } | OutboxState::Leased { .. } => None,
        }
    }
}
