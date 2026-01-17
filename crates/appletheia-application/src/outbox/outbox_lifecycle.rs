use super::OutboxDeadLetteredAt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutboxLifecycle {
    Active,
    DeadLettered {
        dead_lettered_at: OutboxDeadLetteredAt,
    },
}
