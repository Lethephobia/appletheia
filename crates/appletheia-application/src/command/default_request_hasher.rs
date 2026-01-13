use std::fmt::Write;

use sha2::{Digest, Sha256};

use crate::command::RequestHasher;

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultRequestHasher;

impl DefaultRequestHasher {
    pub fn new() -> Self {
        Self
    }

    fn canonicalize_json(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Array(array) => serde_json::Value::Array(
                array
                    .into_iter()
                    .map(Self::canonicalize_json)
                    .collect::<Vec<_>>(),
            ),
            serde_json::Value::Object(map) => {
                let mut entries: Vec<(String, serde_json::Value)> = map.into_iter().collect();
                entries.sort_by(|(a, _), (b, _)| a.cmp(b));

                let mut sorted = serde_json::Map::with_capacity(entries.len());
                for (key, value) in entries {
                    sorted.insert(key, Self::canonicalize_json(value));
                }
                serde_json::Value::Object(sorted)
            }
            other => other,
        }
    }

    fn to_lower_hex(bytes: &[u8]) -> String {
        let mut out = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            let _ = write!(&mut out, "{:02x}", b);
        }
        out
    }
}

impl RequestHasher for DefaultRequestHasher {
    fn request_hash(&self, value: serde_json::Value) -> String {
        let canonical = Self::canonicalize_json(value);
        let json = serde_json::to_string(&canonical).unwrap_or_default();

        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        Self::to_lower_hex(&hasher.finalize())
    }
}
