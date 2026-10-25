use std::collections::BTreeMap;

use super::{
    ReadModel, ReadModelAttributeKey, ReadModelAttributeValue, ReadModelFieldFilter, ReadModelId,
    ReadModelRef, ReadModelRelationship, ReadModelRelationshipKey, ReadModelResource,
    ReadModelViewError,
};

/// Pairs a read model with the field selection for one resource representation.
pub struct ReadModelView<R>
where
    R: ReadModel,
{
    pub read_model: R,
    pub filter: R::Filter,
}

impl<R> ReadModelView<R>
where
    R: ReadModel,
{
    pub fn read_model_ref(&self) -> ReadModelRef {
        ReadModelRef {
            read_model_type: R::TYPE.into(),
            id: ReadModelId::new(self.read_model.id().to_string()),
        }
    }

    pub fn try_to_resource(&self) -> Result<ReadModelResource, ReadModelViewError> {
        let filter = &self.filter;
        let mut attributes = BTreeMap::new();
        for key in filter.attribute_keys() {
            let resource_key = key.try_to_resource_key()?;
            if attributes.contains_key(&resource_key) {
                return Err(ReadModelViewError::DuplicateField(resource_key));
            }
            let value = self
                .read_model
                .attribute(*key)
                .try_to_resource_attribute_value()?;
            attributes.insert(resource_key, value);
        }

        let mut relationships = BTreeMap::new();
        for key in filter.relationship_keys() {
            let resource_key = key.try_to_resource_key()?;
            if attributes.contains_key(&resource_key) || relationships.contains_key(&resource_key) {
                return Err(ReadModelViewError::DuplicateField(resource_key));
            }
            let relationship = self
                .read_model
                .relationship(*key)
                .try_to_resource_relationship()?;
            relationships.insert(resource_key, relationship);
        }

        Ok(ReadModelResource {
            read_model_type: R::TYPE,
            id: ReadModelId::new(self.read_model.id().to_string()),
            attributes,
            relationships,
        })
    }
}
