use appletheia::application::authorization::{
    Relation, Relationship, RelationshipChange, RelationshipStore, RelationshipSubject,
};
use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{
    Organization, OrganizationMembership, OrganizationMembershipEventPayload, OrganizationRole,
    User,
};

use super::{
    OrganizationRoleRelationshipProjectorError, OrganizationRoleRelationshipProjectorSpec,
};
use crate::authorization::{
    OrganizationAdminRelation, OrganizationFinanceManagerRelation, OrganizationTreasurerRelation,
};

/// Projects elevated organization roles granted through memberships.
pub struct OrganizationRoleRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    relationship_store: RS,
}

impl<RS> OrganizationRoleRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    pub fn new(relationship_store: RS) -> Self {
        Self { relationship_store }
    }

    fn upsert_role_relationships(
        organization_id: banking_iam_domain::OrganizationId,
        user_id: banking_iam_domain::UserId,
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

    fn delete_all_role_relationships(
        organization_id: banking_iam_domain::OrganizationId,
        user_id: banking_iam_domain::UserId,
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

    fn relation_for_role(
        role: OrganizationRole,
    ) -> appletheia::application::authorization::RelationRef {
        match role {
            OrganizationRole::Admin => OrganizationAdminRelation::REF,
            OrganizationRole::FinanceManager => OrganizationFinanceManagerRelation::REF,
            OrganizationRole::Treasurer => OrganizationTreasurerRelation::REF,
        }
    }
}

impl<RS> Projector for OrganizationRoleRelationshipProjector<RS>
where
    RS: RelationshipStore,
{
    type Spec = OrganizationRoleRelationshipProjectorSpec;
    type Uow = RS::Uow;
    type Error = OrganizationRoleRelationshipProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if !event.is_for_aggregate::<OrganizationMembership>() {
            return Ok(());
        }

        let domain_event = event.try_into_domain_event::<OrganizationMembership>()?;
        let changes = match domain_event.payload() {
            OrganizationMembershipEventPayload::Created {
                organization_id,
                user_id,
                ..
            } => Self::upsert_role_relationships(*organization_id, *user_id, &[]),
            OrganizationMembershipEventPayload::Activated {
                organization_id,
                user_id,
                roles,
            } => Self::upsert_role_relationships(*organization_id, *user_id, roles),
            OrganizationMembershipEventPayload::RoleGranted {
                organization_id,
                user_id,
                role,
            } => vec![RelationshipChange::Upsert(
                Relationship::new::<Organization>(
                    *organization_id,
                    Self::relation_for_role(*role),
                    RelationshipSubject::aggregate::<User>(*user_id),
                ),
            )],
            OrganizationMembershipEventPayload::RoleRevoked {
                organization_id,
                user_id,
                role,
            } => vec![RelationshipChange::Delete(
                Relationship::new::<Organization>(
                    *organization_id,
                    Self::relation_for_role(*role),
                    RelationshipSubject::aggregate::<User>(*user_id),
                ),
            )],
            OrganizationMembershipEventPayload::Inactivated {
                organization_id,
                user_id,
            }
            | OrganizationMembershipEventPayload::Removed {
                organization_id,
                user_id,
            } => Self::delete_all_role_relationships(*organization_id, *user_id),
        };

        if changes.is_empty() {
            return Ok(());
        }

        self.relationship_store.apply_changes(uow, &changes).await?;
        Ok(())
    }
}
