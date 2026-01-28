use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum PublishDispatchError {
    Transient { code: String, message: String },
    Permanent { code: String, message: String },
}

