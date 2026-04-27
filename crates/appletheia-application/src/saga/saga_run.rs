use serde::{Serialize, de::DeserializeOwned};

use appletheia_domain::EventId;

use crate::request_context::MessageId;

use super::{SagaNameOwned, SagaRunId};

#[derive(Clone, Debug, PartialEq)]
pub struct SagaRun<C>
where
    C: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub saga_run_id: SagaRunId,
    pub saga_name: SagaNameOwned,
    pub trigger_event_id: EventId,
    pub dispatched_command_message_id: Option<MessageId>,
    pub context: C,
}

impl<C> SagaRun<C>
where
    C: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    pub fn new(
        saga_name: SagaNameOwned,
        trigger_event_id: EventId,
        dispatched_command_message_id: Option<MessageId>,
        context: C,
    ) -> Self {
        Self {
            saga_run_id: SagaRunId::new(),
            saga_name,
            trigger_event_id,
            dispatched_command_message_id,
            context,
        }
    }
}
