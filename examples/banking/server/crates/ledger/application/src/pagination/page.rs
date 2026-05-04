/// Page returned by cursor-paginated queries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page<T, C> {
    pub items: Vec<T>,
    pub next_cursor: Option<C>,
}
