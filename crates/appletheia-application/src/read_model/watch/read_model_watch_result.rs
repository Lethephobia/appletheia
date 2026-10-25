use crate::read_model::ReadModelDocument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadModelWatchResult<D>
where
    D: ReadModelDocument,
{
    Document(D),
    Closed,
}
