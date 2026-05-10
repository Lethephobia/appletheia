/// Direction used by query-side sorting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SortDirection {
    Asc,
    Desc,
}
