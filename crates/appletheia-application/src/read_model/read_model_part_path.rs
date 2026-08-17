use serde::{Deserialize, Serialize};

use super::ReadModelPartPathSegment;

/// Locates one replaceable part within a read model snapshot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelPartPath(Vec<ReadModelPartPathSegment>);

impl ReadModelPartPath {
    /// Creates one concrete replacement path.
    pub fn new(segments: Vec<ReadModelPartPathSegment>) -> Self {
        Self(segments)
    }

    /// Appends a relative path to this path.
    pub fn append(mut self, relative_path: Self) -> Self {
        self.0.extend(relative_path.0);
        self
    }

    /// Returns the ordered path segments.
    pub fn segments(&self) -> &[ReadModelPartPathSegment] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_a_relative_path_without_changing_the_structured_key() {
        let key = serde_json::json!({
            "user_id": "user-1",
            "organization_id": "organization-1",
        });

        let parent = ReadModelPartPath::new(vec![
            ReadModelPartPathSegment::Attribute("items".to_owned()),
            ReadModelPartPathSegment::Key(key),
        ]);
        let path = parent.append(ReadModelPartPath::new(vec![
            ReadModelPartPathSegment::Attribute("organization".to_owned()),
        ]));
        let serialized = serde_json::to_value(path).expect("replacement path should serialize");

        assert_eq!(
            serialized,
            serde_json::json!([
                { "type": "attribute", "value": "items" },
                {
                    "type": "key",
                    "value": {
                        "user_id": "user-1",
                        "organization_id": "organization-1",
                    },
                },
                { "type": "attribute", "value": "organization" },
            ])
        );
    }
}
