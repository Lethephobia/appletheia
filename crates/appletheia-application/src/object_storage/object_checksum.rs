use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::{ObjectChecksumAlgorithm, ObjectChecksumValue};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ObjectChecksum {
    algorithm: ObjectChecksumAlgorithm,
    value: ObjectChecksumValue,
}

impl ObjectChecksum {
    pub fn new(algorithm: ObjectChecksumAlgorithm, value: ObjectChecksumValue) -> Self {
        Self { algorithm, value }
    }

    pub fn algorithm(&self) -> ObjectChecksumAlgorithm {
        self.algorithm
    }

    pub fn value(&self) -> &ObjectChecksumValue {
        &self.value
    }
}

impl Display for ObjectChecksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm.as_str(), self.value())
    }
}
