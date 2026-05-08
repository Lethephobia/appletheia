use appletheia_domain::aggregate::AggregateId;

/// A page of source aggregate IDs returned by `ReferenceIndexLookup`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceIndexLookupPage<I>
where
    I: AggregateId,
{
    pub source_ids: Vec<I>,
    pub next_cursor: Option<I>,
}

impl<I> ReferenceIndexLookupPage<I>
where
    I: AggregateId,
{
    pub fn new(source_ids: Vec<I>, next_cursor: Option<I>) -> Self {
        Self {
            source_ids,
            next_cursor,
        }
    }

    pub fn is_last(&self) -> bool {
        self.next_cursor.is_none()
    }
}
