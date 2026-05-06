use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{User, UserId};

use super::{UserOwnerRelation, UserRelationshipUpdater, UserRelationshipUpdaterError};

pub struct DefaultUserRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultUserRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }
}

impl<RS> UserRelationshipUpdater for DefaultUserRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<User>(
                    user_id,
                    UserOwnerRelation::REF,
                    RelationshipSubject::aggregate::<User>(user_id),
                ))],
            )
            .await?;

        Ok(())
    }
}
