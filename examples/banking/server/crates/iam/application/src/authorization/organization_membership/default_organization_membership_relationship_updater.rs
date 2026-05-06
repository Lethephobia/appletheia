use appletheia::application::authorization::{
    AggregateRef, Relation, RelationRef, RelationRefOwned, Relationship, RelationshipChange,
    RelationshipStore, RelationshipSubject,
};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationMembership, OrganizationMembershipId,
    OrganizationRole, User, UserId,
};

use super::{
    OrganizationMembershipOrganizationRelation, OrganizationMembershipRelationshipUpdater,
    OrganizationMembershipRelationshipUpdaterError,
};
use crate::authorization::{
    OrganizationAdminRelation, OrganizationFinanceManagerRelation, OrganizationMemberRelation,
    OrganizationTreasurerRelation,
};

pub struct DefaultOrganizationMembershipRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> DefaultOrganizationMembershipRelationshipUpdater<RS>
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
        roles: &[OrganizationRole],
    ) -> Vec<RelationshipChange> {
        let mut deduplicated_roles = Vec::with_capacity(roles.len());
        for role in roles {
            if deduplicated_roles.contains(role) {
                continue;
            }

            deduplicated_roles.push(*role);
        }

        deduplicated_roles
            .into_iter()
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

impl<RS> OrganizationMembershipRelationshipUpdater
    for DefaultOrganizationMembershipRelationshipUpdater<RS>
where
    RS: RelationshipStore,
{
    type Uow = RS::Uow;
    async fn upsert_organization(
        &self,
        uow: &mut Self::Uow,
        membership_id: OrganizationMembershipId,
        organization_id: OrganizationId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(Relationship::new::<
                    OrganizationMembership,
                >(
                    membership_id,
                    OrganizationMembershipOrganizationRelation::REF,
                    RelationshipSubject::aggregate::<Organization>(organization_id),
                ))],
            )
            .await?;

        Ok(())
    }

    async fn remove_organization(
        &self,
        uow: &mut Self::Uow,
        membership_id: OrganizationMembershipId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
        let aggregate = AggregateRef::from_id::<OrganizationMembership>(membership_id);
        let relation = RelationRefOwned::from(OrganizationMembershipOrganizationRelation::REF);
        let changes = self
            .relationship_store
            .read_subjects_by_aggregate(uow, &aggregate, &relation, None)
            .await?
            .into_iter()
            .map(|subject| {
                RelationshipChange::Delete(Relationship::new::<OrganizationMembership>(
                    membership_id,
                    OrganizationMembershipOrganizationRelation::REF,
                    subject,
                ))
            })
            .collect::<Vec<_>>();

        if !changes.is_empty() {
            self.relationship_store.apply_changes(uow, &changes).await?;
        }

        Ok(())
    }

    async fn upsert_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
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

    async fn remove_member(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
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

    async fn upsert_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: &[OrganizationRole],
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
        let changes = Self::role_upserts(organization_id, user_id, roles);
        if !changes.is_empty() {
            self.relationship_store.apply_changes(uow, &changes).await?;
        }

        Ok(())
    }

    async fn upsert_role(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Upsert(
                    Relationship::new::<Organization>(
                        organization_id,
                        Self::relation_for_role(role),
                        RelationshipSubject::aggregate::<User>(user_id),
                    ),
                )],
            )
            .await?;

        Ok(())
    }

    async fn remove_role(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
        self.relationship_store
            .apply_changes(
                uow,
                &[RelationshipChange::Delete(
                    Relationship::new::<Organization>(
                        organization_id,
                        Self::relation_for_role(role),
                        RelationshipSubject::aggregate::<User>(user_id),
                    ),
                )],
            )
            .await?;

        Ok(())
    }

    async fn remove_all_roles(
        &self,
        uow: &mut Self::Uow,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipRelationshipUpdaterError> {
        let changes = Self::all_role_deletes(organization_id, user_id);
        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }
}
