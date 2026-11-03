use appletheia::application::read_model::{
    ReadModelRelationship, ReadModelRelationshipError, ReadModelResourceRelationship,
};

#[derive(Clone, Debug)]
pub enum UserReadModelRelationship {}

impl ReadModelRelationship for UserReadModelRelationship {
    fn try_into_resource_relationship(
        self,
    ) -> Result<ReadModelResourceRelationship, ReadModelRelationshipError> {
        match self {}
    }
}
