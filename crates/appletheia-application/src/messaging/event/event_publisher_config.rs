use crate::messaging::{CloudEventSource, CloudEventTypePrefix};

#[derive(Clone, Debug)]
pub struct EventPublisherConfig {
    pub source: CloudEventSource,
    pub cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}
