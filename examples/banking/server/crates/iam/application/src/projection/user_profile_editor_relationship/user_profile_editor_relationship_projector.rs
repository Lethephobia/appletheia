use appletheia::application::authorization::{
    AggregateRef, Relation, RelationNameOwned, Relationship, RelationshipChange, RelationshipStore,
    RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{User, UserEventPayload, UserId};

use super::{
    UserProfileEditorRelationshipProjectorError, UserProfileEditorRelationshipProjectorSpec,
};
use crate::authorization::UserProfileEditorRelation;

/// Projects `user#profile_editor@user` relationships from `User` events.
pub struct UserProfileEditorRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> UserProfileEditorRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn relationship(user_id: UserId) -> Relationship {
        Relationship {
            aggregate: AggregateRef::from_id::<User>(user_id),
            relation: RelationNameOwned::from(UserProfileEditorRelation::NAME),
            subject: RelationshipSubject::Aggregate(AggregateRef::from_id::<User>(user_id)),
        }
    }
}

impl<RS> Projector for UserProfileEditorRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = UserProfileEditorRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = UserProfileEditorRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event = event.try_into_domain_event::<User>()?;

        let change = match event.payload() {
            UserEventPayload::Registered { id, .. } => {
                RelationshipChange::Upsert(Self::relationship(*id))
            }
            UserEventPayload::Removed => {
                RelationshipChange::Delete(Self::relationship(event.aggregate_id()))
            }
            _ => return Ok(()),
        };

        self.relationship_store
            .apply_changes(uow, &[change])
            .await?;
        Ok(())
    }
}
