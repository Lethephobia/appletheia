use serde::Serialize;
use serde::de::DeserializeOwned;

use super::{ReadModelResourceWindowCursor, ReadModelWindowCursorError, ReadModelWindowSort};

pub trait ReadModelWindowCursor: Serialize + DeserializeOwned + Send + Sync {
    type SortKey;

    fn sort(&self) -> ReadModelWindowSort<Self::SortKey>;

    fn try_from_resource_window_cursor(
        cursor: ReadModelResourceWindowCursor,
    ) -> Result<Self, ReadModelWindowCursorError> {
        Ok(serde_json::from_value(cursor.try_into_json()?)?)
    }

    fn try_into_resource_window_cursor(
        self,
    ) -> Result<ReadModelResourceWindowCursor, ReadModelWindowCursorError> {
        Ok(ReadModelResourceWindowCursor::try_from_json(
            serde_json::to_value(self)?,
        )?)
    }
}
