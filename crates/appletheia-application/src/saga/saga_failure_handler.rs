use super::SagaContext;
use crate::command::CommandFailureEnvelope;

/// Handles a terminal command failure belonging to the matched step.
pub type SagaFailureHandler<'a, S, T, E> =
    dyn Fn(&mut SagaContext<'_, S, T>, &CommandFailureEnvelope) -> Result<(), E> + Send + Sync + 'a;
