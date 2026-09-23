use crate::messaging::CloudEventTypePrefix;

#[derive(Clone, Debug)]
pub struct EventSubscriberConfig {
    pub type_prefix: Option<CloudEventTypePrefix>,
}
