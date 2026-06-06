use appletheia::application::authorization::{
    Relation, RelationRef, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationRole, OrganizationRoles, User, UserId,
};

use super::{UserOwnerRelation, UserRelationshipUpdater, UserRelationshipUpdaterError};
use crate::authorization::{
    OrganizationAdminRelation, OrganizationFinanceManagerRelation, OrganizationMemberRelation,
    OrganizationTreasurerRelation,
};

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

    fn relation_for_role(role: OrganizationRole) -> RelationRef {
        match role {
            OrganizationRole::Admin => OrganizationAdminRelation::REF,
            OrganizationRole::FinanceManager => OrganizationFinanceManagerRelation::REF,
            OrganizationRole::Treasurer => OrganizationTreasurerRelation::REF,
        }
    }

    fn role_upserts(
        organization_id: OrganizationId,
        user_id: UserId,
        roles: &OrganizationRoles,
    ) -> Vec<RelationshipChange> {
        roles
            .iter()
            .copied()
            .map(|role| {
                RelationshipChange::Upsert(Relationship::new::<Organization>(
                    organization_id,
                    Self::relation_for_role(role),
                    RelationshipSubject::aggregate::<User>(user_id),
                ))
            })
            .collect()
    }

    fn all_role_deletes(
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Vec<RelationshipChange> {
        [
            OrganizationRole::Admin,
            OrganizationRole::FinanceManager,
            OrganizationRole::Treasurer,
        ]
        .into_iter()
        .map(|role| {
            RelationshipChange::Delete(Relationship::new::<Organization>(
                organization_id,
                Self::relation_for_role(role),
                RelationshipSubject::aggregate::<User>(user_id),
            ))
        })
        .collect()
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

    async fn upsert_organization_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(
                    Relationship::new::<Organization>(
                        organization_id,
                        OrganizationMemberRelation::REF,
                        RelationshipSubject::aggregate::<User>(user_id),
                    ),
                )],
            )
            .await?;

        Ok(())
    }

    async fn remove_organization_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Delete(
                    Relationship::new::<Organization>(
                        organization_id,
                        OrganizationMemberRelation::REF,
                        RelationshipSubject::aggregate::<User>(user_id),
                    ),
                )],
            )
            .await?;

        Ok(())
    }

    async fn replace_organization_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: &OrganizationRoles,
    ) -> Result<(), UserRelationshipUpdaterError> {
        let mut changes = Self::all_role_deletes(organization_id, user_id);
        changes.extend(Self::role_upserts(organization_id, user_id, roles));
        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }

    async fn remove_all_organization_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError> {
        let changes = Self::all_role_deletes(organization_id, user_id);
        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }
}
