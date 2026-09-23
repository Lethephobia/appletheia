use crate::messaging::{CloudEventSource, CloudEventTypePrefix};

#[derive(Clone, Debug)]
pub struct CommandFailurePublisherConfig {
    pub source: CloudEventSource,
    pub type_prefix: Option<CloudEventTypePrefix>,
}
