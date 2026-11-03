use appletheia::application::read_model::{
    ReadModelRef, ReadModelRelationship, ReadModelRelationshipData, ReadModelRelationshipError,
    ReadModelResourceRelationship,
};

use crate::read_model::{UserIdentityReadModel, UserIdentityReadModelId};

#[derive(Clone, Debug)]
pub enum UserReadModelRelationship {
    Identities(Vec<UserIdentityReadModelId>),
}

impl ReadModelRelationship for UserReadModelRelationship {
    fn try_into_resource_relationship(
        self,
    ) -> Result<ReadModelResourceRelationship, ReadModelRelationshipError> {
        match self {
            Self::Identities(ids) => Ok(ReadModelResourceRelationship {
                data: ReadModelRelationshipData::ToMany(
                    ids.into_iter()
                        .map(ReadModelRef::new::<UserIdentityReadModel>)
                        .collect(),
                ),
            }),
        }
    }
}
