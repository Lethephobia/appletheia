use serde::{Deserialize, Serialize};

use super::{
    ReadModelWatchCloseReason, ReadModelWatchFailure, ReadModelWatchRevision,
    ReadModelWatchSubscriptionId, SerializedReadModelListChunk, SerializedReadModelSnapshot,
};

/// Defines the complete-snapshot protocol visible to client SDKs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReadModelWatchEvent {
    SnapshotUpdated {
        subscription_id: ReadModelWatchSubscriptionId,
        revision: ReadModelWatchRevision,
        value: SerializedReadModelSnapshot,
    },
    ListSnapshotUpdated {
        subscription_id: ReadModelWatchSubscriptionId,
        revision: ReadModelWatchRevision,
        chunks: Vec<SerializedReadModelListChunk>,
    },
    SubscriptionError {
        subscription_id: ReadModelWatchSubscriptionId,
        failure: ReadModelWatchFailure,
    },
    SubscriptionClosed {
        subscription_id: ReadModelWatchSubscriptionId,
        reason: ReadModelWatchCloseReason,
    },
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializes_snapshot_updates_without_internal_invalidation_details() {
        let subscription_id = ReadModelWatchSubscriptionId::new();
        let revision = ReadModelWatchRevision::initial()
            .checked_next()
            .expect("revision should increment");
        let event = ReadModelWatchEvent::SnapshotUpdated {
            subscription_id,
            revision,
            value: SerializedReadModelSnapshot::from(json!({ "name": "Ada" })),
        };

        let value = serde_json::to_value(event).expect("event should serialize");

        assert_eq!(value["type"], "snapshot_updated");
        assert_eq!(value["data"]["revision"], 1);
        assert_eq!(value["data"]["value"], json!({ "name": "Ada" }));
        assert!(value.get("invalidation").is_none());
        assert!(value.get("dependencies").is_none());
    }
}
