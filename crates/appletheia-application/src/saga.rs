use std::error::Error;

use serde::{Serialize, de::DeserializeOwned};

use appletheia_domain::{Aggregate, Event};

use crate::command::Command;

pub mod default_saga_runner;
pub mod default_saga_worker;
pub mod enqueued_command_count;
pub mod saga_dependencies;
pub mod saga_descriptor;
pub mod saga_name;
pub mod saga_name_owned;
pub mod saga_name_owned_error;
pub mod saga_predecessor;
pub mod saga_processed_event_id;
pub mod saga_processed_event_id_error;
pub mod saga_processed_event_store;
pub mod saga_processed_event_store_error;
pub mod saga_run;
pub mod saga_run_id;
pub mod saga_run_id_error;
pub mod saga_run_report;
pub mod saga_run_store;
pub mod saga_run_store_error;
pub mod saga_runner;
pub mod saga_runner_error;
pub mod saga_spec;
pub mod saga_transition;
pub mod saga_worker;
pub mod saga_worker_error;

pub use default_saga_runner::DefaultSagaRunner;
pub use default_saga_worker::DefaultSagaWorker;
pub use enqueued_command_count::EnqueuedCommandCount;
pub use saga_dependencies::SagaDependencies;
pub use saga_descriptor::SagaDescriptor;
pub use saga_name::SagaName;
pub use saga_name_owned::SagaNameOwned;
pub use saga_name_owned_error::SagaNameOwnedError;
pub use saga_predecessor::SagaPredecessor;
pub use saga_processed_event_id::SagaProcessedEventId;
pub use saga_processed_event_id_error::SagaProcessedEventIdError;
pub use saga_processed_event_store::SagaProcessedEventStore;
pub use saga_processed_event_store_error::SagaProcessedEventStoreError;
pub use saga_run::SagaRun;
pub use saga_run_id::SagaRunId;
pub use saga_run_id_error::SagaRunIdError;
pub use saga_run_report::SagaRunReport;
pub use saga_run_store::SagaRunStore;
pub use saga_run_store_error::SagaRunStoreError;
pub use saga_runner::SagaRunner;
pub use saga_runner_error::SagaRunnerError;
pub use saga_spec::SagaSpec;
pub use saga_transition::SagaTransition;
pub use saga_worker::SagaWorker;
pub use saga_worker_error::SagaWorkerError;

/// Handles events for a saga run.
pub trait Saga: Send + Sync {
    type Spec: SagaSpec;
    type Context: Serialize + DeserializeOwned + Send + Sync + 'static;
    type EventAggregate: Aggregate;
    type Command: Command;
    type Error: Error + Send + Sync + 'static;

    fn on_event(
        &self,
        context: Option<Self::Context>,
        event: &Event<
            <Self::EventAggregate as Aggregate>::Id,
            <Self::EventAggregate as Aggregate>::EventPayload,
        >,
    ) -> Result<SagaTransition<Self::Context, Self::Command>, Self::Error>;
}
