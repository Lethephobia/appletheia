use crate::messaging::CloudEventTypePrefix;

#[derive(Clone, Debug)]
pub struct CommandFailureSubscriberConfig {
    pub cloud_event_type_prefix: Option<CloudEventTypePrefix>,
}
