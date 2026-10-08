use crate::event::EventSelector;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectorDefinitionError {
    #[error("duplicate event route: {selector:?}")]
    DuplicateEventRoute { selector: EventSelector },
}
