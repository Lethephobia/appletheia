use super::{
    ReadModel, ReadModelIncluded, ReadModelListDocumentError, ReadModelResourceListDocument,
};

pub struct ReadModelListDocument<R, I>
where
    R: ReadModel,
    I: ReadModelIncluded,
{
    pub data: Vec<R>,
    pub included: Vec<I>,
}

impl<R, I> ReadModelListDocument<R, I>
where
    R: ReadModel,
    I: ReadModelIncluded,
{
    pub fn try_to_resource_document(
        &self,
    ) -> Result<ReadModelResourceListDocument, ReadModelListDocumentError> {
        let data = self
            .data
            .iter()
            .map(ReadModel::try_to_resource)
            .collect::<Result<Vec<_>, _>>()?;
        let included = self
            .included
            .iter()
            .map(ReadModelIncluded::try_to_resource)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadModelResourceListDocument { data, included })
    }
}
