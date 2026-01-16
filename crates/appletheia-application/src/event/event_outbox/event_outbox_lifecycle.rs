use super::EventOutboxDeadLetteredAt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventOutboxLifecycle {
    Active,
    DeadLettered {
        dead_lettered_at: EventOutboxDeadLetteredAt,
    },
}
