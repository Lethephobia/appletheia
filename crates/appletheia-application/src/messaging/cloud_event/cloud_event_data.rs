use serde_json::Value;

/// Payload representation before transport encoding. Text is non-JSON content;
/// a JSON string is represented by `Json(Value::String(...))`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CloudEventData {
    Binary(Vec<u8>),
    Text(String),
    Json(Value),
}
