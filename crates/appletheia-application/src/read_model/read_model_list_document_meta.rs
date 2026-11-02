use super::{
    ReadModelListDocumentMetaError, ReadModelResourceListDocumentMeta, ReadModelWindow,
    ReadModelWindowCursor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelListDocumentMeta<C>
where
    C: ReadModelWindowCursor,
{
    pub window: Option<ReadModelWindow<C>>,
}

impl<C> ReadModelListDocumentMeta<C>
where
    C: ReadModelWindowCursor,
{
    pub fn try_into_resource_list_document_meta(
        self,
    ) -> Result<ReadModelResourceListDocumentMeta, ReadModelListDocumentMetaError> {
        Ok(ReadModelResourceListDocumentMeta {
            window: self
                .window
                .map(ReadModelWindow::try_into_resource_window)
                .transpose()?,
        })
    }
}
