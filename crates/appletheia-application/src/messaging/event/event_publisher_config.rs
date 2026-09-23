use crate::messaging::{CloudEventSource, CloudEventTypePrefix};

#[derive(Clone, Debug)]
pub struct EventPublisherConfig {
    pub source: CloudEventSource,
    pub type_prefix: Option<CloudEventTypePrefix>,
}
