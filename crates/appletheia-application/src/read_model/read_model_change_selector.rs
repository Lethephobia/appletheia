use super::{ReadModelChangeEnvelope, ReadModelType};
use crate::messaging::Selector;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReadModelChangeSelector {
    pub read_model_type: ReadModelType,
}

impl ReadModelChangeSelector {
    pub const fn new(read_model_type: ReadModelType) -> Self {
        Self { read_model_type }
    }
}

impl Selector<ReadModelChangeEnvelope> for ReadModelChangeSelector {
    fn matches(&self, message: &ReadModelChangeEnvelope) -> bool {
        self.read_model_type.value() == message.change.read_model_ref().read_model_type.value()
    }
}
