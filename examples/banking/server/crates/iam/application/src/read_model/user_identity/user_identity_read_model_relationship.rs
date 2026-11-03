use appletheia::application::read_model::{
    ReadModelRelationship, ReadModelRelationshipError, ReadModelResourceRelationship,
};

#[derive(Clone, Debug)]
pub enum UserIdentityReadModelRelationship {}

impl ReadModelRelationship for UserIdentityReadModelRelationship {
    fn try_into_resource_relationship(
        self,
    ) -> Result<ReadModelResourceRelationship, ReadModelRelationshipError> {
        match self {}
    }
}
