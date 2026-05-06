use super::SortDirection;

/// Cursor pagination options for query reads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CursorOptions<K, C> {
    pub sort_key: K,
    pub sort_direction: SortDirection,
    pub cursor: Option<C>,
}
