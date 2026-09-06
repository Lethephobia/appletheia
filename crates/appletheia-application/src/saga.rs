use std::error::Error;

use crate::command::CommandFailureEnvelope;
use crate::event::EventEnvelope;

pub mod default_saga_command_failure_worker;
pub mod default_saga_event_worker;
pub mod default_saga_runner;
pub mod enqueued_command_count;
pub mod saga_command_failure_run_report;
pub mod saga_command_failure_worker;
pub mod saga_command_failure_worker_error;
pub mod saga_command_origin;
pub mod saga_dependencies;
pub mod saga_descriptor;
pub mod saga_dispatched_command;
pub mod saga_event_run_report;
pub mod saga_event_worker;
pub mod saga_event_worker_error;
pub mod saga_instance;
pub mod saga_instance_error;
pub mod saga_instance_id;
pub mod saga_instance_id_error;
pub mod saga_instance_store;
pub mod saga_instance_store_error;
pub mod saga_name;
pub mod saga_name_owned;
pub mod saga_name_owned_error;
pub mod saga_processed_command_failure_id;
pub mod saga_processed_command_failure_id_error;
pub mod saga_processed_command_failure_store;
pub mod saga_processed_command_failure_store_error;
pub mod saga_processed_event_id;
pub mod saga_processed_event_id_error;
pub mod saga_processed_event_store;
pub mod saga_processed_event_store_error;
pub mod saga_runner;
pub mod saga_runner_error;
pub mod saga_spec;
pub mod saga_start_events;
pub mod saga_state;
pub mod saga_status;
pub mod saga_step;
pub mod serialized_saga_step;
pub mod serialized_saga_step_error;

pub use default_saga_command_failure_worker::DefaultSagaCommandFailureWorker;
pub use default_saga_event_worker::DefaultSagaEventWorker;
pub use default_saga_runner::DefaultSagaRunner;
pub use enqueued_command_count::EnqueuedCommandCount;
pub use saga_command_failure_run_report::SagaCommandFailureRunReport;
pub use saga_command_failure_worker::SagaCommandFailureWorker;
pub use saga_command_failure_worker_error::SagaCommandFailureWorkerError;
pub use saga_command_origin::SagaCommandOrigin;
pub use saga_dependencies::SagaDependencies;
pub use saga_descriptor::SagaDescriptor;
pub use saga_dispatched_command::SagaDispatchedCommand;
pub use saga_event_run_report::SagaEventRunReport;
pub use saga_event_worker::SagaEventWorker;
pub use saga_event_worker_error::SagaEventWorkerError;
pub use saga_instance::SagaInstance;
pub use saga_instance_error::SagaInstanceError;
pub use saga_instance_id::SagaInstanceId;
pub use saga_instance_id_error::SagaInstanceIdError;
pub use saga_instance_store::SagaInstanceStore;
pub use saga_instance_store_error::SagaInstanceStoreError;
pub use saga_name::SagaName;
pub use saga_name_owned::SagaNameOwned;
pub use saga_name_owned_error::SagaNameOwnedError;
pub use saga_processed_command_failure_id::SagaProcessedCommandFailureId;
pub use saga_processed_command_failure_id_error::SagaProcessedCommandFailureIdError;
pub use saga_processed_command_failure_store::SagaProcessedCommandFailureStore;
pub use saga_processed_command_failure_store_error::SagaProcessedCommandFailureStoreError;
pub use saga_processed_event_id::SagaProcessedEventId;
pub use saga_processed_event_id_error::SagaProcessedEventIdError;
pub use saga_processed_event_store::SagaProcessedEventStore;
pub use saga_processed_event_store_error::SagaProcessedEventStoreError;
pub use saga_runner::SagaRunner;
pub use saga_runner_error::SagaRunnerError;
pub use saga_spec::SagaSpec;
pub use saga_start_events::SagaStartEvents;
pub use saga_state::SagaState;
pub use saga_status::SagaStatus;
pub use saga_step::SagaStep;
pub use serialized_saga_step::SerializedSagaStep;
pub use serialized_saga_step_error::SerializedSagaStepError;

/// Handles events for a saga instance.
pub trait Saga: Send + Sync {
    type Spec: SagaSpec;
    type Step: SagaStep;
    type Error: Error + Send + Sync + 'static;

    /// Handles an event with the saga step that dispatched its causative command.
    ///
    /// `step` is `None` only when `event` is one of the saga's start events.
    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        event: &EventEnvelope,
        causative_step: Option<Self::Step>,
    ) -> Result<(), Self::Error>;

    /// Handles one terminal command failure routed to this saga.
    fn on_command_failed(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
        _failure: &CommandFailureEnvelope,
        _causative_step: Self::Step,
    ) -> Result<(), Self::Error> {
        instance.fail();
        Ok(())
    }
}
