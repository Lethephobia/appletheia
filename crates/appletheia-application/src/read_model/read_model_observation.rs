use appletheia_domain::EventId;
use serde::{Deserialize, Serialize};

/// Identifies the source-event range materialized by one read model fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ReadModelObservation {
    pub source_event_id: EventId,
    pub updated_event_id: EventId,
}

impl ReadModelObservation {
    /// Creates an observation from the fragment's first and latest source events.
    pub fn new(source_event_id: EventId, updated_event_id: EventId) -> Self {
        Self {
            source_event_id,
            updated_event_id,
        }
    }

    /// Iterates over the source and latest event identifiers.
    pub fn event_ids(&self) -> impl Iterator<Item = EventId> {
        [self.source_event_id, self.updated_event_id].into_iter()
    }

    /// Collects event identifiers while preserving first-seen order and uniqueness.
    pub fn collect_event_ids(event_ids: impl IntoIterator<Item = EventId>) -> Vec<EventId> {
        let mut collected = Vec::new();

        for event_id in event_ids {
            if !collected.contains(&event_id) {
                collected.push(event_id);
            }
        }

        collected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_event_ids_preserves_order_and_removes_duplicates() {
        let first = EventId::new();
        let second = EventId::new();

        assert_eq!(
            ReadModelObservation::collect_event_ids([first, second, first]),
            vec![first, second]
        );
    }
}
