use crate::messaging::{CloudEventSource, CloudEventTypePrefix};

#[derive(Clone, Debug)]
pub struct CommandPublisherConfig {
    pub source: CloudEventSource,
    pub type_prefix: Option<CloudEventTypePrefix>,
}
