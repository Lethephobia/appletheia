use crate::messaging::{CloudEventSource, CloudEventTypePrefix};

#[derive(Clone, Debug)]
pub struct CommandPublisherConfig {
    pub source: CloudEventSource,
    pub cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}
