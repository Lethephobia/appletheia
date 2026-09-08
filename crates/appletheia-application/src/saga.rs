use std::error::Error;

mod default_saga_command_failure_worker;
pub mod default_saga_event_worker;
pub mod default_saga_runner;
pub mod enqueued_command_count;
pub mod saga_command_failure_run_report;
pub mod saga_command_failure_worker;
pub mod saga_command_failure_worker_error;
pub mod saga_command_origin;
pub mod saga_context;
pub mod saga_context_error;
pub mod saga_definition;
pub mod saga_definition_builder;
pub mod saga_definition_builder_error;
pub mod saga_definition_error;
pub mod saga_dispatched_command;
pub mod saga_error;
pub mod saga_event_handler;
pub mod saga_event_handler_builder;
pub mod saga_event_run_report;
pub mod saga_event_worker;
pub mod saga_event_worker_error;
pub mod saga_failure_handler;
pub mod saga_failure_handler_builder;
pub mod saga_failure_step_builder;
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
pub mod saga_route;
pub mod saga_route_error;
pub mod saga_runner;
pub mod saga_runner_error;
pub mod saga_start_event_handler_builder;
pub mod saga_start_step_builder;
pub mod saga_state;
pub mod saga_step;
pub mod saga_step_builder;
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
pub use saga_context::SagaContext;
pub use saga_context_error::SagaContextError;
pub use saga_definition::SagaDefinition;
pub use saga_definition_builder::SagaDefinitionBuilder;
pub use saga_definition_builder_error::SagaDefinitionBuilderError;
pub use saga_definition_error::SagaDefinitionError;
pub use saga_dispatched_command::SagaDispatchedCommand;
pub use saga_error::SagaError;
pub use saga_event_handler::SagaEventHandler;
pub use saga_event_handler_builder::SagaEventHandlerBuilder;
pub use saga_event_run_report::SagaEventRunReport;
pub use saga_event_worker::SagaEventWorker;
pub use saga_event_worker_error::SagaEventWorkerError;
pub use saga_failure_handler::SagaFailureHandler;
pub use saga_failure_handler_builder::SagaFailureHandlerBuilder;
pub use saga_failure_step_builder::SagaFailureStepBuilder;
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
pub use saga_route::SagaRoute;
pub use saga_route_error::SagaRouteError;
pub use saga_runner::SagaRunner;
pub use saga_runner_error::SagaRunnerError;
pub use saga_start_event_handler_builder::SagaStartEventHandlerBuilder;
pub use saga_start_step_builder::SagaStartStepBuilder;
pub use saga_state::SagaState;
pub use saga_step::SagaStep;
pub use saga_step_builder::SagaStepBuilder;
pub use serialized_saga_step::SerializedSagaStep;
pub use serialized_saga_step_error::SerializedSagaStepError;

/// Builds the routes for an application saga, optionally borrowing injected services.
pub trait Saga: Send + Sync {
    type State: SagaState;
    type Step: SagaStep;
    type HandlerError: Error + Send + Sync + 'static;

    /// Builds a deterministic route definition without side effects.
    ///
    /// Each worker calls this once at startup, before subscribing to messages.
    #[allow(
        clippy::type_complexity,
        reason = "Keep the saga state, step, and handler error explicit"
    )]
    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError>;
}
