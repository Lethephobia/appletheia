use serde::{Deserialize, Serialize};

use super::{ReadModelWindowCursor, ReadModelWindowSize, ReadModelWindowSort};

/// Selects a window relative to the ordering established by the initial query.
/// Continuation cursors encode that ordering together with the query-specific boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReadModelWindowRequest<K> {
    Initial {
        sort: ReadModelWindowSort<K>,
        limit: ReadModelWindowSize,
    },
    Before {
        cursor: ReadModelWindowCursor,
        limit: ReadModelWindowSize,
    },
    After {
        cursor: ReadModelWindowCursor,
        limit: ReadModelWindowSize,
    },
    Around {
        cursor: ReadModelWindowCursor,
        before_limit: ReadModelWindowSize,
        after_limit: ReadModelWindowSize,
    },
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum SortKey {
        Name,
        CreatedAt,
    }

    #[test]
    fn round_trips_initial_and_cursor_required_requests() {
        for value in [
            json!({"type": "initial", "data": {"sort": {"key": "created_at", "direction": "desc"}, "limit": 20}}),
            json!({"type": "before", "data": {"cursor": "boundary", "limit": 20}}),
            json!({"type": "after", "data": {"cursor": "boundary", "limit": 20}}),
            json!({"type": "around", "data": {"cursor": "boundary", "before_limit": 10, "after_limit": 20}}),
        ] {
            let request: ReadModelWindowRequest<SortKey> =
                serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(request).unwrap(), value);
        }
    }

    #[test]
    fn rejects_missing_sort_fields_cursors_and_zero_limits() {
        for value in [
            json!({"type": "initial", "data": {"sort": {"direction": "asc"}, "limit": 20}}),
            json!({"type": "initial", "data": {"sort": {"key": "name"}, "limit": 20}}),
            json!({"type": "before", "data": {"limit": 20}}),
            json!({"type": "after", "data": {"cursor": null, "limit": 20}}),
            json!({"type": "around", "data": {"before_limit": 10, "after_limit": 20}}),
            json!({"type": "initial", "data": {"sort": {"key": "name", "direction": "asc"}, "limit": 0}}),
            json!({"type": "around", "data": {"cursor": "boundary", "before_limit": 0, "after_limit": 20}}),
            json!({"type": "around", "data": {"cursor": "boundary", "before_limit": 10, "after_limit": 0}}),
        ] {
            assert!(serde_json::from_value::<ReadModelWindowRequest<SortKey>>(value).is_err());
        }
    }
}
