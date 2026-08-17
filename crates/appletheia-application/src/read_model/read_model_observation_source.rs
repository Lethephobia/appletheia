use appletheia_domain::EventId;

use super::ReadModelObservation;

/// Exposes every source-event observation materialized by a read model value.
pub trait ReadModelObservationSource {
    /// Returns the observations represented by this value.
    fn observations(&self) -> Vec<ReadModelObservation>;

    /// Returns unique source-event identifiers in observation order.
    fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observations().into_iter().flat_map(|observation| {
                [observation.source_event_id, observation.updated_event_id]
            }),
        )
    }
}

impl<T> ReadModelObservationSource for Option<T>
where
    T: ReadModelObservationSource,
{
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.as_ref()
            .map(ReadModelObservationSource::observations)
            .unwrap_or_default()
    }
}
