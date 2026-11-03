use super::{ReadModelResourceWindow, ReadModelWindowCursor, ReadModelWindowError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWindow<C>
where
    C: ReadModelWindowCursor,
{
    pub start_cursor: Option<C>,
    pub end_cursor: Option<C>,
    pub has_before: bool,
    pub has_after: bool,
}

impl<C> ReadModelWindow<C>
where
    C: ReadModelWindowCursor,
{
    pub fn try_into_resource_window(self) -> Result<ReadModelResourceWindow, ReadModelWindowError> {
        Ok(ReadModelResourceWindow {
            start_cursor: self
                .start_cursor
                .map(ReadModelWindowCursor::try_into_resource_window_cursor)
                .transpose()?,
            end_cursor: self
                .end_cursor
                .map(ReadModelWindowCursor::try_into_resource_window_cursor)
                .transpose()?,
            has_before: self.has_before,
            has_after: self.has_after,
        })
    }
}
