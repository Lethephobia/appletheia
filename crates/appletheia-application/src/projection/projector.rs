use std::error::Error;

use crate::event::EventEnvelopeError;
use crate::unit_of_work::UnitOfWork;

use super::{ProjectorDefinition, ProjectorError};

/// Defines typed event routes that update stored query data.
///
/// Handler futures may borrow injected services and the transaction. They are not
/// required to be `Send`; execute the worker directly or on a local task executor.
pub trait Projector: Send + Sync {
    type Uow: UnitOfWork;
    type HandlerError: Error + From<EventEnvelopeError> + Send + Sync + 'static;

    /// Builds routes without side effects, once per worker or rebuild invocation.
    fn definition(
        &self,
    ) -> Result<ProjectorDefinition<'_, Self::Uow, Self::HandlerError>, ProjectorError>;
}
