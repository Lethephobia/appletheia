use super::{
    EventOutboxAttemptCount, EventOutboxLeaseExpiresAt, EventOutboxNextAttemptAt,
    EventOutboxPublishedAt, EventOutboxRelayInstance,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventOutboxState {
    Pending {
        attempt_count: EventOutboxAttemptCount,
        next_attempt_after: EventOutboxNextAttemptAt,
    },
    Leased {
        attempt_count: EventOutboxAttemptCount,
        next_attempt_after: EventOutboxNextAttemptAt,
        lease_owner: EventOutboxRelayInstance,
        lease_until: EventOutboxLeaseExpiresAt,
    },
    Published {
        published_at: EventOutboxPublishedAt,
        attempt_count: EventOutboxAttemptCount,
    },
}

impl EventOutboxState {
    pub fn attempt_count(&self) -> EventOutboxAttemptCount {
        match self {
            EventOutboxState::Pending { attempt_count, .. }
            | EventOutboxState::Leased { attempt_count, .. }
            | EventOutboxState::Published { attempt_count, .. } => *attempt_count,
        }
    }

    pub fn next_attempt_after(&self) -> Option<EventOutboxNextAttemptAt> {
        match self {
            EventOutboxState::Pending {
                next_attempt_after, ..
            }
            | EventOutboxState::Leased {
                next_attempt_after, ..
            } => Some(*next_attempt_after),
            EventOutboxState::Published { .. } => None,
        }
    }

    pub fn lease_owner(&self) -> Option<&EventOutboxRelayInstance> {
        match self {
            EventOutboxState::Leased { lease_owner, .. } => Some(lease_owner),
            EventOutboxState::Pending { .. } | EventOutboxState::Published { .. } => None,
        }
    }

    pub fn lease_until(&self) -> Option<EventOutboxLeaseExpiresAt> {
        match self {
            EventOutboxState::Leased { lease_until, .. } => Some(*lease_until),
            EventOutboxState::Pending { .. } | EventOutboxState::Published { .. } => None,
        }
    }

    pub fn published_at(&self) -> Option<EventOutboxPublishedAt> {
        match self {
            EventOutboxState::Published { published_at, .. } => Some(*published_at),
            EventOutboxState::Pending { .. } | EventOutboxState::Leased { .. } => None,
        }
    }
}
