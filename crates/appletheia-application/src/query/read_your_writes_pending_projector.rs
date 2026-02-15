use crate::event::EventSequence;
use crate::projection::ProjectorNameOwned;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadYourWritesPendingProjector {
    pub projector_name: ProjectorNameOwned,
    pub last_checkpoint: Option<EventSequence>,
}

