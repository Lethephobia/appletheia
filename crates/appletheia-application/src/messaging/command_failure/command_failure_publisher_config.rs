use crate::messaging::{CloudEventSource, CloudEventTypePrefix};

#[derive(Clone, Debug)]
pub struct CommandFailurePublisherConfig {
    pub source: CloudEventSource,
    pub cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}
