use crate::messaging::{CloudEventSource, CloudEventTypePrefix};

#[derive(Clone, Debug)]
pub struct ReadModelInvalidationPublisherConfig {
    pub source: CloudEventSource,
    pub type_prefix: Option<CloudEventTypePrefix>,
}
