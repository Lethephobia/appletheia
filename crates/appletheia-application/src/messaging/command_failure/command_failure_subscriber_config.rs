use crate::messaging::CloudEventTypePrefix;

#[derive(Clone, Debug)]
pub struct CommandFailureSubscriberConfig {
    pub type_prefix: Option<CloudEventTypePrefix>,
}
