use appletheia::domain::EventId;

/// Observed event range for a read model fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ReadModelObservation {
    pub source_event_id: EventId,
    pub updated_event_id: EventId,
}

impl ReadModelObservation {
    pub fn new(source_event_id: EventId, updated_event_id: EventId) -> Self {
        Self {
            source_event_id,
            updated_event_id,
        }
    }

    pub fn event_ids(&self) -> impl Iterator<Item = EventId> {
        [self.source_event_id, self.updated_event_id].into_iter()
    }

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
