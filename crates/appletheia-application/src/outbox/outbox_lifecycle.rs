use super::DeadLetteredAt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutboxLifecycle {
    Active,
    DeadLettered { dead_lettered_at: DeadLetteredAt },
}
