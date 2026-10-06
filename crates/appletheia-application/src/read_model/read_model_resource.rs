use std::collections::BTreeMap;

use serde::Serialize;

use super::{
    ReadModelId, ReadModelResourceAttributeValue, ReadModelResourceKey,
    ReadModelResourceRelationship, ReadModelType,
};

/// A filtered JSON:API resource object, without a document wrapper.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelResource {
    #[serde(rename = "type")]
    pub read_model_type: ReadModelType,
    pub id: ReadModelId,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<ReadModelResourceKey, ReadModelResourceAttributeValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub relationships: BTreeMap<ReadModelResourceKey, ReadModelResourceRelationship>,
}
