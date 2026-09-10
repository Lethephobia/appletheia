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

pub use default_saga_command_failure_worker::*;
pub use default_saga_event_worker::*;
pub use default_saga_runner::*;
pub use enqueued_command_count::*;
pub use saga_command_failure_run_report::*;
pub use saga_command_failure_worker::*;
pub use saga_command_failure_worker_error::*;
pub use saga_command_origin::*;
pub use saga_context::*;
pub use saga_context_error::*;
pub use saga_definition::*;
pub use saga_definition_builder::*;
pub use saga_definition_builder_error::*;
pub use saga_definition_error::*;
pub use saga_dispatched_command::*;
pub use saga_error::*;
pub use saga_event_handler::*;
pub use saga_event_handler_builder::*;
pub use saga_event_run_report::*;
pub use saga_event_worker::*;
pub use saga_event_worker_error::*;
pub use saga_failure_handler::*;
pub use saga_failure_handler_builder::*;
pub use saga_failure_step_builder::*;
pub use saga_instance::*;
pub use saga_instance_error::*;
pub use saga_instance_id::*;
pub use saga_instance_id_error::*;
pub use saga_instance_store::*;
pub use saga_instance_store_error::*;
pub use saga_name::*;
pub use saga_name_owned::*;
pub use saga_name_owned_error::*;
pub use saga_processed_command_failure_id::*;
pub use saga_processed_command_failure_id_error::*;
pub use saga_processed_command_failure_store::*;
pub use saga_processed_command_failure_store_error::*;
pub use saga_processed_event_id::*;
pub use saga_processed_event_id_error::*;
pub use saga_processed_event_store::*;
pub use saga_processed_event_store_error::*;
pub use saga_route::*;
pub use saga_route_error::*;
pub use saga_runner::*;
pub use saga_runner_error::*;
pub use saga_start_event_handler_builder::*;
pub use saga_start_step_builder::*;
pub use saga_state::*;
pub use saga_step::*;
pub use saga_step_builder::*;
pub use serialized_saga_step::*;
pub use serialized_saga_step_error::*;

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
