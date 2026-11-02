use serde::{Deserialize, Serialize};

use super::{ReadModelWindowCursor, ReadModelWindowSize, ReadModelWindowSort};

/// Selects an unordered result or a window within an explicit ordering.
/// Unordered requests return no window metadata or cursors. Ordered requests start at
/// the beginning; continuation cursors carry the ordering and query-specific boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
#[serde(bound(
    serialize = "C::SortKey: Serialize",
    deserialize = "C::SortKey: Deserialize<'de>"
))]
pub enum ReadModelWindowRequest<C>
where
    C: ReadModelWindowCursor,
{
    Unordered {
        limit: ReadModelWindowSize,
    },
    Ordered {
        sort: ReadModelWindowSort<C::SortKey>,
        limit: ReadModelWindowSize,
    },
    Before {
        cursor: C,
        limit: ReadModelWindowSize,
    },
    After {
        cursor: C,
        limit: ReadModelWindowSize,
    },
    Around {
        cursor: C,
        before_limit: ReadModelWindowSize,
        after_limit: ReadModelWindowSize,
    },
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::super::{
        ReadModelResourceWindowCursor, ReadModelWindowCursorError, ReadModelWindowSortDirection,
    };

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum SortKey {
        Name,
        CreatedAt,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Cursor {
        position: u64,
    }

    impl ReadModelWindowCursor for Cursor {
        type SortKey = SortKey;

        fn sort(&self) -> ReadModelWindowSort<Self::SortKey> {
            ReadModelWindowSort {
                key: SortKey::CreatedAt,
                direction: ReadModelWindowSortDirection::Desc,
            }
        }
    }

    #[test]
    fn uses_default_cursor_conversion_with_serde_derives() {
        #[derive(Serialize, Deserialize)]
        struct Position(u64);

        impl ReadModelWindowCursor for Position {
            type SortKey = SortKey;

            fn sort(&self) -> ReadModelWindowSort<Self::SortKey> {
                ReadModelWindowSort {
                    key: SortKey::Name,
                    direction: ReadModelWindowSortDirection::Asc,
                }
            }
        }

        let request: ReadModelWindowRequest<Position> = ReadModelWindowRequest::After {
            cursor: Position(42),
            limit: ReadModelWindowSize::new(20).unwrap(),
        };
        let ReadModelWindowRequest::After { cursor, limit } = request else {
            panic!("expected after request");
        };
        assert_eq!(cursor.0, 42);
        assert_eq!(limit.value(), 20);
        let sort = cursor.sort();
        assert!(matches!(sort.key, SortKey::Name));
        assert_eq!(sort.direction, ReadModelWindowSortDirection::Asc);
        let resource_cursor = cursor.try_into_resource_window_cursor().unwrap();
        let restored = Position::try_from_resource_window_cursor(resource_cursor).unwrap();
        assert_eq!(restored.0, 42);
    }

    #[test]
    fn typed_cursor_rejects_incompatible_json_shape() {
        for value in [json!("not a position"), json!({"position":"invalid"})] {
            assert!(matches!(
                Cursor::try_from_resource_window_cursor(
                    ReadModelResourceWindowCursor::try_from_json(value).unwrap()
                ),
                Err(ReadModelWindowCursorError::Json(_))
            ));
        }
    }

    #[test]
    fn round_trips_unordered_ordered_and_cursor_requests() {
        for value in [
            json!({"type": "unordered", "data": {"limit": 20}}),
            json!({"type": "ordered", "data": {"sort": {"key": "created_at", "direction": "desc"}, "limit": 20}}),
            json!({"type": "before", "data": {"cursor": {"position": 42}, "limit": 20}}),
            json!({"type": "after", "data": {"cursor": {"position": 42}, "limit": 20}}),
            json!({"type": "around", "data": {"cursor": {"position": 42}, "before_limit": 10, "after_limit": 20}}),
        ] {
            let request: ReadModelWindowRequest<Cursor> =
                serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(request).unwrap(), value);
        }
    }

    #[test]
    fn rejects_missing_sort_fields_cursors_and_zero_limits() {
        for value in [
            json!({"type": "unordered", "data": {"limit": 0}}),
            json!({"type": "unordered", "data": {}}),
            json!({"type": "ordered", "data": {"limit": 20}}),
            json!({"type": "ordered", "data": {"sort": {"direction": "asc"}, "limit": 20}}),
            json!({"type": "ordered", "data": {"sort": {"key": "name"}, "limit": 20}}),
            json!({"type": "before", "data": {"limit": 20}}),
            json!({"type": "after", "data": {"cursor": null, "limit": 20}}),
            json!({"type": "around", "data": {"before_limit": 10, "after_limit": 20}}),
            json!({"type": "ordered", "data": {"sort": {"key": "name", "direction": "asc"}, "limit": 0}}),
            json!({"type": "around", "data": {"cursor": {"position": 42}, "before_limit": 0, "after_limit": 20}}),
            json!({"type": "around", "data": {"cursor": {"position": 42}, "before_limit": 10, "after_limit": 0}}),
        ] {
            assert!(serde_json::from_value::<ReadModelWindowRequest<Cursor>>(value).is_err());
        }
    }
}
