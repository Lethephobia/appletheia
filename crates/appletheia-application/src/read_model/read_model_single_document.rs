use std::collections::HashSet;

use super::{
    ReadModel, ReadModelDocument, ReadModelDocumentError, ReadModelIncluded, ReadModelRef,
    ReadModelResourceDocument, ReadModelResourceDocumentData, ReadModelView,
};

pub struct ReadModelSingleDocument<R, I>
where
    R: ReadModel,
    I: ReadModelIncluded,
{
    pub data: Option<ReadModelView<R>>,
    pub included: Vec<I>,
}

impl<R, I> ReadModelDocument for ReadModelSingleDocument<R, I>
where
    R: ReadModel,
    I: ReadModelIncluded,
{
    fn read_model_refs(&self) -> Vec<ReadModelRef> {
        let mut seen = HashSet::new();
        self.data
            .iter()
            .map(ReadModelView::read_model_ref)
            .chain(self.included.iter().map(ReadModelIncluded::read_model_ref))
            .filter(|reference| seen.insert(reference.clone()))
            .collect()
    }

    fn try_to_resource_document(
        &self,
    ) -> Result<ReadModelResourceDocument, ReadModelDocumentError> {
        let data = self
            .data
            .as_ref()
            .map(ReadModelView::try_to_resource)
            .transpose()?;
        let included = self
            .included
            .iter()
            .map(ReadModelIncluded::try_to_resource)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadModelResourceDocument {
            data: ReadModelResourceDocumentData::Single(data),
            included,
        })
    }
}
