use super::{ReadModel, ReadModelDocumentError, ReadModelIncluded, ReadModelResourceDocument};

pub struct ReadModelDocument<R, I>
where
    R: ReadModel,
    I: ReadModelIncluded,
{
    pub data: Option<R>,
    pub included: Vec<I>,
}

impl<R, I> ReadModelDocument<R, I>
where
    R: ReadModel,
    I: ReadModelIncluded,
{
    pub fn try_to_resource_document(
        &self,
    ) -> Result<ReadModelResourceDocument, ReadModelDocumentError> {
        let data = self
            .data
            .as_ref()
            .map(ReadModel::try_to_resource)
            .transpose()?;
        let included = self
            .included
            .iter()
            .map(ReadModelIncluded::try_to_resource)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadModelResourceDocument { data, included })
    }
}
