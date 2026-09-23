use crate::messaging::CloudEventTypePrefix;

#[derive(Clone, Debug)]
pub struct CommandSubscriberConfig {
    pub type_prefix: Option<CloudEventTypePrefix>,
}
