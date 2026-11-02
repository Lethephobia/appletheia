use std::collections::HashSet;

use super::{
    ReadModel, ReadModelDocument, ReadModelDocumentError, ReadModelIncluded,
    ReadModelListDocumentMeta, ReadModelRef, ReadModelResourceDocument, ReadModelView,
};

pub struct ReadModelListDocument<R, I>
where
    R: ReadModel,
    I: ReadModelIncluded,
{
    pub data: Vec<ReadModelView<R>>,
    pub included: Vec<I>,
    pub meta: Option<ReadModelListDocumentMeta>,
}

impl<R, I> ReadModelDocument for ReadModelListDocument<R, I>
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

    fn try_into_resource_document(
        self,
    ) -> Result<ReadModelResourceDocument, ReadModelDocumentError> {
        let data = self
            .data
            .into_iter()
            .map(ReadModelView::try_into_resource)
            .collect::<Result<Vec<_>, _>>()?;
        let included = self
            .included
            .into_iter()
            .map(ReadModelIncluded::try_into_resource)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ReadModelResourceDocument::List {
            data,
            included,
            meta: self.meta,
        })
    }
}
