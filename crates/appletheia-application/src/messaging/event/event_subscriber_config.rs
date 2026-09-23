use crate::messaging::CloudEventTypePrefix;

#[derive(Clone, Debug)]
pub struct EventSubscriberConfig {
    pub cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}
