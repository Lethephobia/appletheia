use serde::{Deserialize, Serialize};

use super::{PageSize, SortDirection};

/// Selects one forward or backward window in a cursor-paginated query.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "direction", rename_all = "snake_case")]
pub enum CursorWindow<C> {
    Forward {
        /// Starts after this cursor, or at the beginning when absent.
        after: Option<C>,
        /// Limits the number of items returned by the query.
        limit: PageSize,
    },
    Backward {
        /// Ends before this cursor, or at the end when absent.
        before: Option<C>,
        /// Limits the number of items returned by the query.
        limit: PageSize,
    },
}

impl<C> CursorWindow<C> {
    /// Returns the maximum number of items requested by this window.
    pub fn limit(&self) -> PageSize {
        match self {
            Self::Forward { limit, .. } | Self::Backward { limit, .. } => *limit,
        }
    }

    /// Returns the exclusive cursor boundary of this window, when present.
    pub fn boundary(&self) -> Option<&C> {
        match self {
            Self::Forward { after, .. } => after.as_ref(),
            Self::Backward { before, .. } => before.as_ref(),
        }
    }

    /// Returns whether the window is loaded toward the beginning of the list.
    pub fn is_backward(&self) -> bool {
        matches!(self, Self::Backward { .. })
    }

    /// Returns the database ordering needed to load this window efficiently.
    pub fn query_direction(&self, display_direction: SortDirection) -> SortDirection {
        if self.is_backward() {
            match display_direction {
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::Asc,
            }
        } else {
            display_direction
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backward_window_reverses_the_database_sort_direction() {
        let window = CursorWindow::<u64>::Backward {
            before: Some(42),
            limit: PageSize::try_from(20).expect("page size should be valid"),
        };

        assert_eq!(window.boundary(), Some(&42));
        assert!(window.is_backward());
        assert_eq!(
            window.query_direction(SortDirection::Asc),
            SortDirection::Desc
        );
        assert_eq!(
            window.query_direction(SortDirection::Desc),
            SortDirection::Asc
        );
    }

    #[test]
    fn forward_window_preserves_the_database_sort_direction() {
        let window = CursorWindow::<u64>::Forward {
            after: None,
            limit: PageSize::try_from(20).expect("page size should be valid"),
        };

        assert_eq!(window.boundary(), None);
        assert!(!window.is_backward());
        assert_eq!(
            window.query_direction(SortDirection::Asc),
            SortDirection::Asc
        );
        assert_eq!(
            window.query_direction(SortDirection::Desc),
            SortDirection::Desc
        );
    }
}
