use crate::messaging::CloudEventTypePrefix;

#[derive(Clone, Debug)]
pub struct CommandSubscriberConfig {
    pub cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}
