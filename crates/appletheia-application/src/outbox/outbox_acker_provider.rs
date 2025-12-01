use super::OutboxAcker;

pub trait OutboxAckerProvider {
    type OutboxAcker<'c>: OutboxAcker
    where
        Self: 'c;

    fn outbox_acker(&mut self) -> Self::OutboxAcker<'_>;
}
