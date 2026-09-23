mod organization;
mod organization_invitation;
mod organization_join_request;
mod organization_membership;
mod user;

pub use organization::*;
pub use organization_invitation::*;
pub use organization_join_request::*;
pub use organization_membership::*;
pub use user::*;

/// Registers this application module's evaluation and derivation declarations.
pub fn define_iam_relations(
    model: &mut appletheia::application::authorization::InMemoryAuthorizationModel,
) {
    model.define_relation(OrganizationAdminRelation);
    model.define_relation(OrganizationFinanceManagerRelation);
    model.define_relation(OrganizationHandleChangerRelation);
    model.define_relation(OrganizationInviterRelation);
    model.define_relation(OrganizationMemberAdderRelation);
    model.define_relation(OrganizationMemberRelation);
    model.define_relation(OrganizationOwnerRelation);
    model.define_relation(OrganizationOwnershipTransfererRelation);
    model.define_relation(OrganizationProfileEditorRelation);
    model.define_relation(OrganizationRemoverRelation);
    model.define_relation(OrganizationTreasurerRelation);
    model.define_relation(OrganizationInvitationCancelerRelation);
    model.define_relation(OrganizationInvitationInviteeRelation);
    model.define_relation(OrganizationInvitationOrganizationRelation);
    model.define_relation(OrganizationJoinRequestApproverRelation);
    model.define_relation(OrganizationJoinRequestCancelerRelation);
    model.define_relation(OrganizationJoinRequestOrganizationRelation);
    model.define_relation(OrganizationJoinRequestRejecterRelation);
    model.define_relation(OrganizationJoinRequestRequesterRelation);
    model.define_relation(OrganizationMembershipOrganizationRelation);
    model.define_relation(OrganizationMembershipRemoverRelation);
    model.define_relation(OrganizationMembershipRolesChangerRelation);
    model.define_relation(UserActivatorRelation);
    model.define_relation(UserDeactivatorRelation);
    model.define_relation(UserOwnerRelation);
    model.define_relation(UserProfileEditorRelation);
    model.define_relation(UserRemoverRelation);
    model.define_relation(UserUsernameChangerRelation);
}

#[cfg(test)]
mod tests {
    use appletheia::application::aggregate::{AggregateRef, SerializedAggregateError};
    use appletheia::application::authorization::{
        InMemoryAuthorizationModel, Relation, RelationshipDerivationError, RelationshipDeriver,
        RelationshipDeriverError, RelationshipEntries, RelationshipSubject, UsersetExpr,
    };
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{
        Organization, OrganizationId, OrganizationMembership, OrganizationMembershipCreation,
        OrganizationMembershipError, OrganizationRole, OrganizationRoles, User, UserId,
    };

    use super::{
        OrganizationAdminRelation, OrganizationMemberRelation,
        OrganizationMembershipOrganizationRelation, OrganizationTreasurerRelation,
        define_iam_relations,
    };

    fn membership() -> OrganizationMembership {
        let mut aggregate = OrganizationMembership::new();
        aggregate
            .create(OrganizationMembershipCreation {
                organization_id: OrganizationId::new(),
                user_id: UserId::new(),
                roles: OrganizationRoles::new([OrganizationRole::Admin]),
            })
            .unwrap();
        aggregate
    }

    #[test]
    fn one_registration_derives_current_membership_roles_and_removal() {
        let mut model = InMemoryAuthorizationModel::new();
        define_iam_relations(&mut model);
        let snapshot = model.clone();
        assert!(
            snapshot
                .derive(&OrganizationMembership::new())
                .unwrap()
                .is_empty()
        );
        let mut aggregate = membership();
        let created = snapshot.derive(&aggregate).unwrap();
        assert_eq!(created.len(), 3);
        assert_eq!(created, snapshot.derive(&aggregate).unwrap());
        assert!(created.iter().any(|entry| entry.relation
            == OrganizationMemberRelation::REF.into()
            && entry.target
                == AggregateRef::from_id::<Organization>(*aggregate.organization_id().unwrap())
            && entry.subject
                == RelationshipSubject::aggregate::<User>(*aggregate.user_id().unwrap())));
        assert!(
            created
                .iter()
                .any(|entry| entry.relation == OrganizationAdminRelation::REF.into())
        );
        aggregate
            .change_roles(OrganizationRoles::new([OrganizationRole::Treasurer]))
            .unwrap();
        let changed = snapshot.derive(&aggregate).unwrap();
        assert!(
            !changed
                .iter()
                .any(|entry| entry.relation == OrganizationAdminRelation::REF.into())
        );
        assert!(
            changed
                .iter()
                .any(|entry| entry.relation == OrganizationTreasurerRelation::REF.into())
        );
        aggregate.remove().unwrap();
        let removed = snapshot.derive(&aggregate).unwrap();
        assert_eq!(removed.len(), 1);
        assert_eq!(
            removed[0].relation,
            OrganizationMembershipOrganizationRelation::REF.into()
        );
    }

    #[test]
    fn cloned_models_are_snapshots_and_computed_relations_do_not_copy_sources() {
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(OrganizationMemberRelation);
        let snapshot = model.clone();
        {
            use appletheia::application::authorization::RelationRef;

            struct ComputedAdminRelation(UsersetExpr);

            impl Relation for ComputedAdminRelation {
                const REF: RelationRef = OrganizationAdminRelation::REF;

                fn expr(&self) -> UsersetExpr {
                    self.0.clone()
                }
            }

            model.define_relation(ComputedAdminRelation(UsersetExpr::ComputedUserset {
                relation: OrganizationMemberRelation::REF.into(),
            }));
        }
        let aggregate = membership();
        assert_eq!(
            model.derive(&aggregate).unwrap(),
            snapshot.derive(&aggregate).unwrap()
        );
        assert!(model.derive(&Organization::new()).unwrap().is_empty());
        model.define_relation(OrganizationAdminRelation);
        assert_eq!(snapshot.derive(&aggregate).unwrap().len(), 1);
        assert_eq!(model.derive(&aggregate).unwrap().len(), 2);
    }

    #[tokio::test]
    async fn reregistering_a_relation_replaces_sources_and_authorization_expression() {
        use super::OrganizationMemberDerivationHandlerError;
        use appletheia::application::authorization::{AuthorizationModel, RelationRef};
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };

        struct ReplacementMemberRelation(UsersetExpr);

        impl Relation for ReplacementMemberRelation {
            const REF: RelationRef = OrganizationMemberRelation::REF;

            fn expr(&self) -> UsersetExpr {
                self.0.clone()
            }
        }

        let calls = Arc::new(AtomicUsize::new(0));
        let handler_calls = Arc::clone(&calls);
        let expr = UsersetExpr::this::<
            OrganizationMembership,
            _,
            OrganizationMemberDerivationHandlerError,
        >(move |aggregate| {
            handler_calls.fetch_add(1, Ordering::SeqCst);
            let mut entries = RelationshipEntries::new();
            entries
                .insert::<Organization, User>(*aggregate.organization_id()?, *aggregate.user_id()?);
            Ok(entries)
        });
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(ReplacementMemberRelation(expr.clone()));
        model.define_relation(ReplacementMemberRelation(expr));
        let aggregate = membership();
        assert_eq!(model.derive(&aggregate).unwrap().len(), 1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        // Moving the source to another aggregate removes the previous source's handler.
        model.define_relation(ReplacementMemberRelation(UsersetExpr::this::<
            Organization,
            _,
            SerializedAggregateError,
        >(|_| {
            Ok(RelationshipEntries::new())
        })));
        assert!(model.derive(&aggregate).unwrap().is_empty());
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        model.define_relation(ReplacementMemberRelation(UsersetExpr::ComputedUserset {
            relation: OrganizationAdminRelation::REF.into(),
        }));
        assert!(model.derive(&aggregate).unwrap().is_empty());
        assert!(matches!(
            model.expr_for(&OrganizationMemberRelation::REF.into()).await.unwrap(),
            Some(UsersetExpr::ComputedUserset { relation })
                if relation == OrganizationAdminRelation::REF.into()
        ));
    }

    #[test]
    fn target_mismatch_is_rejected_before_persistence() {
        let mut model = InMemoryAuthorizationModel::new();
        {
            use appletheia::application::authorization::RelationRef;

            #[derive(Debug, thiserror::Error)]
            enum InvalidMemberDerivationHandlerError {
                #[error(transparent)]
                SerializedAggregate(#[from] SerializedAggregateError),
                #[error(transparent)]
                OrganizationMembership(#[from] OrganizationMembershipError),
            }

            struct InvalidMemberRelation(UsersetExpr);

            impl Relation for InvalidMemberRelation {
                const REF: RelationRef = OrganizationMemberRelation::REF;

                fn expr(&self) -> UsersetExpr {
                    self.0.clone()
                }
            }

            model.define_relation(InvalidMemberRelation(UsersetExpr::this::<
                OrganizationMembership,
                _,
                InvalidMemberDerivationHandlerError,
            >(|aggregate| {
                let mut entries = RelationshipEntries::new();
                entries.insert::<User, User>(*aggregate.user_id()?, *aggregate.user_id()?);
                Ok(entries)
            })));
        }
        assert!(matches!(
            model.derive(&membership()),
            Err(RelationshipDeriverError::RelationshipDerivation(
                RelationshipDerivationError::TargetMismatch
            ))
        ));
    }

    #[test]
    fn ownership_transfer_derives_only_the_current_owner() {
        use super::OrganizationOwnerRelation;
        use banking_iam_domain::{
            OrganizationCreation, OrganizationDisplayName, OrganizationHandle, OrganizationOwner,
        };

        let previous_owner = UserId::new();
        let next_owner = UserId::new();
        let mut organization = Organization::new();
        organization
            .create(OrganizationCreation {
                owner: OrganizationOwner::User(previous_owner),
                handle: OrganizationHandle::try_from("relation-test").unwrap(),
                display_name: OrganizationDisplayName::try_from("Relation Test").unwrap(),
                description: None,
                website_url: None,
                picture: None,
            })
            .unwrap();
        let mut model = InMemoryAuthorizationModel::new();
        define_iam_relations(&mut model);
        let snapshot = model.clone();
        let previous = snapshot.derive(&organization).unwrap();
        assert_eq!(previous.len(), 1);
        assert_eq!(
            previous[0].subject,
            RelationshipSubject::aggregate::<User>(previous_owner)
        );
        organization
            .transfer_ownership(OrganizationOwner::User(next_owner))
            .unwrap();
        let current = snapshot.derive(&organization).unwrap();
        assert_eq!(current.len(), 1);
        assert_eq!(current[0].relation, OrganizationOwnerRelation::REF.into());
        assert_eq!(
            current[0].subject,
            RelationshipSubject::aggregate::<User>(next_owner)
        );
    }

    #[test]
    fn instance_registration_captures_configuration_and_shared_services() {
        use appletheia::application::authorization::RelationRef;
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };

        #[derive(Debug, thiserror::Error)]
        enum ConfiguredMemberDerivationHandlerError {
            #[error(transparent)]
            SerializedAggregate(#[from] SerializedAggregateError),
            #[error(transparent)]
            OrganizationMembership(#[from] OrganizationMembershipError),
        }

        struct ConfiguredMemberRelation {
            enabled: bool,
            expression_calls: Arc<AtomicUsize>,
            derivation_calls: Arc<AtomicUsize>,
        }

        impl Relation for ConfiguredMemberRelation {
            const REF: RelationRef = OrganizationMemberRelation::REF;

            fn expr(&self) -> UsersetExpr {
                self.expression_calls.fetch_add(1, Ordering::SeqCst);
                let enabled = self.enabled;
                let calls = Arc::clone(&self.derivation_calls);
                UsersetExpr::this::<OrganizationMembership, _, ConfiguredMemberDerivationHandlerError>(
                    move |aggregate| {
                        calls.fetch_add(1, Ordering::SeqCst);
                        let mut entries = RelationshipEntries::new();
                        if enabled && aggregate.is_active()? {
                            entries.insert::<Organization, User>(
                                *aggregate.organization_id()?,
                                *aggregate.user_id()?,
                            );
                        }
                        Ok(entries)
                    },
                )
            }
        }

        let expression_calls = Arc::new(AtomicUsize::new(0));
        let derivation_calls = Arc::new(AtomicUsize::new(0));
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(ConfiguredMemberRelation {
            enabled: true,
            expression_calls: Arc::clone(&expression_calls),
            derivation_calls: Arc::clone(&derivation_calls),
        });
        let snapshot = model.clone();
        drop(model);
        let aggregate = membership();
        assert_eq!(snapshot.derive(&aggregate).unwrap().len(), 1);
        assert_eq!(snapshot.derive(&aggregate).unwrap().len(), 1);
        assert_eq!(expression_calls.load(Ordering::SeqCst), 1);
        assert_eq!(derivation_calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn serialized_aggregate_round_trip_preserves_state_without_pending_events() {
        use appletheia::application::aggregate::{
            SerializedAggregate, SerializedAggregateError, SerializedAggregateState,
        };

        let aggregate = membership();
        let serialized = SerializedAggregate::try_from_aggregate(&aggregate).unwrap();
        let restored = serialized
            .try_to_aggregate::<OrganizationMembership>()
            .unwrap();
        assert_eq!(restored.aggregate_id(), aggregate.aggregate_id());
        assert_eq!(restored.version(), aggregate.version());
        assert_eq!(restored.state(), aggregate.state());
        assert!(!aggregate.uncommitted_events().is_empty());
        assert!(restored.uncommitted_events().is_empty());
        assert!(matches!(
            serialized.try_to_aggregate::<Organization>(),
            Err(SerializedAggregateError::AggregateTypeMismatch { .. })
        ));

        let mut malformed = serialized.clone();
        malformed.state = Some(SerializedAggregateState::from(serde_json::json!({})));
        assert!(matches!(
            malformed.try_to_aggregate::<OrganizationMembership>(),
            Err(SerializedAggregateError::SerializedAggregateState(_))
        ));
        malformed.state = Some(SerializedAggregateState::from(serde_json::Value::Null));
        assert!(malformed.state.is_some());
        assert!(
            malformed
                .try_to_aggregate::<OrganizationMembership>()
                .is_err()
        );

        let empty = OrganizationMembership::new();
        let serialized_empty = SerializedAggregate::try_from_aggregate(&empty).unwrap();
        assert!(serialized_empty.state.is_none());
        let restored_empty = serialized_empty
            .try_to_aggregate::<OrganizationMembership>()
            .unwrap();
        assert_eq!(restored_empty.aggregate_id(), empty.aggregate_id());
        assert_eq!(restored_empty.version(), empty.version());
        assert!(restored_empty.state().is_none());
    }
}
