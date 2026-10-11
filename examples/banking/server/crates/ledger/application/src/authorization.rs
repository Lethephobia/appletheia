mod account;
mod currency;
mod currency_registrar_membership;
mod deposit;
mod organization;
mod organization_ledger_role_assignment;
mod token_binding;
mod transfer;
mod wallet_bookmark;
mod withdrawal;

pub use banking_iam_application::authorization::UserOwnerRelation;

pub use account::*;
pub use currency::*;
pub use currency_registrar_membership::*;
pub use deposit::*;
pub use organization::*;
pub use organization_ledger_role_assignment::*;
pub use token_binding::*;
pub use transfer::*;
pub use wallet_bookmark::*;
pub use withdrawal::*;

#[cfg(test)]
mod tests {
    use appletheia::application::aggregate::{AggregateRef, AggregateTypeOwned};
    use appletheia::application::authorization::{
        DefaultRelationshipResolver, InMemoryAuthorizationModel, Relation, RelationRefOwned,
        Relationship, RelationshipDeriver, RelationshipRequirement, RelationshipResolver,
        RelationshipResolverConfig, RelationshipStore, RelationshipStoreError, RelationshipSubject,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_application::authorization::{
        OrganizationAdminRelation, OrganizationMemberRelation, OrganizationOwnerRelation,
    };
    use banking_iam_domain::{
        Organization, OrganizationDisplayName, OrganizationHandle, OrganizationMembership,
        OrganizationOwner, User, UserId,
    };

    use super::{
        AccountOwnerRelation, AccountOwnershipManagerRelation, OrganizationFinanceManagerRelation,
        WalletBookmarkManagerRelation, WalletBookmarkOwnerRelation,
    };
    use banking_ledger_domain::account::{Account, AccountName, AccountOwner};
    use banking_ledger_domain::core::{
        SolanaAccountAddress, SolanaTokenAccountOwnerAddress, TokenOwnerAddress,
    };
    use banking_ledger_domain::currency::CurrencyId;
    use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkOwner};

    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    struct TestRelationshipStore(Vec<Relationship>);

    impl RelationshipStore for TestRelationshipStore {
        type Uow = TestUow;

        async fn replace(
            &self,
            _uow: &mut Self::Uow,
            _source: &AggregateRef,
            _relationships: &[Relationship],
        ) -> Result<(), RelationshipStoreError> {
            unreachable!("authorization checks do not replace relationships")
        }

        async fn read_targets_by_subject(
            &self,
            _uow: &mut Self::Uow,
            subject: &RelationshipSubject,
            relation: &RelationRefOwned,
        ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
            Ok(self
                .0
                .iter()
                .filter(|entry| &entry.subject == subject && &entry.relation == relation)
                .map(|entry| entry.target.clone())
                .collect())
        }

        async fn read_subjects_by_target(
            &self,
            _uow: &mut Self::Uow,
            target: &AggregateRef,
            relation: &RelationRefOwned,
            subject_aggregate_type: Option<&AggregateTypeOwned>,
        ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
            Ok(self
                .0
                .iter()
                .filter(|entry| {
                    &entry.target == target
                        && &entry.relation == relation
                        && subject_aggregate_type.is_none_or(|expected| match &entry.subject {
                            RelationshipSubject::Aggregate(aggregate)
                            | RelationshipSubject::AggregateSet { aggregate, .. } => {
                                &aggregate.aggregate_type == expected
                            }
                            RelationshipSubject::Wildcard { aggregate_type } => {
                                aggregate_type == expected
                            }
                        })
                })
                .map(|entry| entry.subject.clone())
                .collect())
        }
    }

    #[tokio::test]
    async fn organization_operations_require_owner_while_membership_remains_available() {
        let owner = UserId::new();
        let member = UserId::new();
        let stranger = UserId::new();
        let mut organization = Organization::new();
        organization
            .create(
                OrganizationOwner::User(owner),
                OrganizationHandle::try_from("owner-only").unwrap(),
                OrganizationDisplayName::try_from("Owner Only").unwrap(),
            )
            .unwrap();
        let mut membership = OrganizationMembership::new();
        membership
            .create(organization.aggregate_id(), member)
            .unwrap();
        let mut account = Account::new();
        account
            .open(
                AccountOwner::Organization(organization.aggregate_id()),
                AccountName::try_from("Checking").unwrap(),
                CurrencyId::new(),
            )
            .unwrap();
        let mut personal_account = Account::new();
        personal_account
            .open(
                AccountOwner::User(owner),
                AccountName::try_from("Personal").unwrap(),
                CurrencyId::new(),
            )
            .unwrap();
        let mut bookmark = WalletBookmark::new();
        bookmark
            .register(
                WalletBookmarkOwner::Organization(organization.aggregate_id()),
                TokenOwnerAddress::Solana(SolanaTokenAccountOwnerAddress::new(
                    SolanaAccountAddress::from_bytes([2; 32]),
                )),
            )
            .unwrap();
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(OrganizationOwnerRelation);
        model.define_relation(OrganizationMemberRelation);
        model.define_relation(AccountOwnerRelation);
        model.define_relation(AccountOwnershipManagerRelation);
        model.define_relation(WalletBookmarkOwnerRelation);
        model.define_relation(WalletBookmarkManagerRelation);
        model.define_relation(OrganizationAdminRelation);
        model.define_relation(OrganizationFinanceManagerRelation);
        let mut relationships = model.derive(&organization).unwrap();
        relationships.extend(model.derive(&membership).unwrap());
        relationships.extend(model.derive(&account).unwrap());
        relationships.extend(model.derive(&personal_account).unwrap());
        relationships.extend(model.derive(&bookmark).unwrap());
        let resolver = DefaultRelationshipResolver::new(
            TestRelationshipStore(relationships),
            model,
            RelationshipResolverConfig::default(),
        );
        let mut uow = TestUow;
        assert!(
            resolver
                .satisfies(
                    &mut uow,
                    &AggregateRef::from_id::<Organization>(organization.aggregate_id()),
                    &RelationshipRequirement::check::<Account>(
                        account.aggregate_id(),
                        AccountOwnerRelation::REF
                    )
                )
                .await
                .unwrap()
        );
        assert!(
            !resolver
                .satisfies(
                    &mut uow,
                    &AggregateRef::from_id::<User>(owner),
                    &RelationshipRequirement::check::<Account>(
                        account.aggregate_id(),
                        AccountOwnerRelation::REF
                    )
                )
                .await
                .unwrap()
        );
        for (subject, expected) in [
            (
                AggregateRef::from_id::<Organization>(organization.aggregate_id()),
                true,
            ),
            (AggregateRef::from_id::<User>(owner), false),
        ] {
            assert_eq!(
                resolver
                    .satisfies(
                        &mut uow,
                        &subject,
                        &RelationshipRequirement::check::<WalletBookmark>(
                            bookmark.aggregate_id(),
                            WalletBookmarkOwnerRelation::REF,
                        ),
                    )
                    .await
                    .unwrap(),
                expected,
            );
        }
        for (user_id, is_owner, is_member) in [
            (owner, true, true),
            (member, false, true),
            (stranger, false, false),
        ] {
            let subject = AggregateRef::from_id::<User>(user_id);
            for requirement in [
                RelationshipRequirement::check::<Account>(
                    personal_account.aggregate_id(),
                    AccountOwnerRelation::REF,
                ),
                RelationshipRequirement::check::<Account>(
                    personal_account.aggregate_id(),
                    AccountOwnershipManagerRelation::REF,
                ),
                RelationshipRequirement::check::<WalletBookmark>(
                    bookmark.aggregate_id(),
                    WalletBookmarkManagerRelation::REF,
                ),
                RelationshipRequirement::check::<Organization>(
                    organization.aggregate_id(),
                    OrganizationOwnerRelation::REF,
                ),
                RelationshipRequirement::check::<Account>(
                    account.aggregate_id(),
                    AccountOwnershipManagerRelation::REF,
                ),
            ] {
                assert_eq!(
                    resolver
                        .satisfies(&mut uow, &subject, &requirement)
                        .await
                        .unwrap(),
                    is_owner
                );
            }
            assert_eq!(
                resolver
                    .satisfies(
                        &mut uow,
                        &subject,
                        &RelationshipRequirement::check::<Organization>(
                            organization.aggregate_id(),
                            OrganizationMemberRelation::REF
                        )
                    )
                    .await
                    .unwrap(),
                is_member
            );
        }
    }

    #[tokio::test]
    async fn registrar_members_can_manage_only_their_registrar_while_active() {
        use super::{
            CurrencyRegistrarMemberRelation, CurrencyRegistrarMembershipManagerRelation,
            CurrencyRegistrarMembershipRegistrarRelation,
        };
        use banking_ledger_domain::{
            CurrencyRegistrar, CurrencyRegistrarId, CurrencyRegistrarMembership,
        };

        let registrar_id = CurrencyRegistrarId::new();
        let member_id = UserId::new();
        let other_member_id = UserId::new();
        let stranger_id = UserId::new();
        let mut membership = CurrencyRegistrarMembership::new();
        membership.create(registrar_id, member_id).unwrap();
        let mut target = CurrencyRegistrarMembership::new();
        target.create(registrar_id, UserId::new()).unwrap();
        let mut other_membership = CurrencyRegistrarMembership::new();
        other_membership
            .create(CurrencyRegistrarId::new(), other_member_id)
            .unwrap();
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(CurrencyRegistrarMemberRelation);
        model.define_relation(CurrencyRegistrarMembershipManagerRelation);
        model.define_relation(CurrencyRegistrarMembershipRegistrarRelation);
        for removed in [false, true] {
            if removed {
                membership.remove().unwrap();
            }
            let mut relationships = model.derive(&membership).unwrap();
            relationships.extend(model.derive(&target).unwrap());
            relationships.extend(model.derive(&other_membership).unwrap());
            let resolver = DefaultRelationshipResolver::new(
                TestRelationshipStore(relationships),
                model.clone(),
                RelationshipResolverConfig::default(),
            );
            for user_id in [member_id, other_member_id, stranger_id] {
                let subject = AggregateRef::from_id::<User>(user_id);
                for requirement in [
                    RelationshipRequirement::check::<CurrencyRegistrar>(
                        registrar_id,
                        CurrencyRegistrarMemberRelation::REF,
                    ),
                    RelationshipRequirement::check::<CurrencyRegistrarMembership>(
                        target.aggregate_id(),
                        CurrencyRegistrarMembershipManagerRelation::REF,
                    ),
                ] {
                    assert_eq!(
                        resolver
                            .satisfies(&mut TestUow, &subject, &requirement)
                            .await
                            .unwrap(),
                        user_id == member_id && !removed
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn ledger_roles_are_scoped_and_follow_assignee_and_assignment_lifecycles() {
        use super::{
            AccountManagerRelation, AccountTransactorRelation, OrganizationFinanceManagerRelation,
            OrganizationTreasurerRelation,
        };
        use banking_iam_application::authorization::{
            OrganizationMembershipUserRelation, TeamMemberRelation, TeamOrganizationRelation,
        };
        use banking_iam_domain::{OrganizationId, Team, TeamMembership, TeamName};
        use banking_ledger_domain::{
            OrganizationLedgerRole, OrganizationLedgerRoleAssignee,
            OrganizationLedgerRoleAssignment,
        };

        use banking_iam_application::authorization::OrganizationAdminRelation;

        let admin_id = UserId::new();
        let owner_id = UserId::new();
        let member_id = UserId::new();
        let outsider_id = UserId::new();
        let mut organization = Organization::new();
        organization
            .create(
                OrganizationOwner::User(owner_id),
                OrganizationHandle::try_from("ledger-roles").unwrap(),
                OrganizationDisplayName::try_from("Ledger Roles").unwrap(),
            )
            .unwrap();
        let mut admin = OrganizationMembership::new();
        admin.create(organization.aggregate_id(), admin_id).unwrap();
        admin.promote_to_admin().unwrap();
        let mut member = OrganizationMembership::new();
        member
            .create(organization.aggregate_id(), member_id)
            .unwrap();
        let mut team = Team::new();
        team.create(
            organization.aggregate_id(),
            TeamName::try_from("Finance").unwrap(),
        )
        .unwrap();
        let mut team_member = TeamMembership::new();
        team_member
            .create(team.aggregate_id(), member.aggregate_id())
            .unwrap();
        let mut finance = OrganizationLedgerRoleAssignment::new();
        finance
            .assign(
                organization.aggregate_id(),
                OrganizationLedgerRoleAssignee::OrganizationMembership(member.aggregate_id()),
                OrganizationLedgerRole::FinanceManager,
            )
            .unwrap();
        let mut treasury = OrganizationLedgerRoleAssignment::new();
        treasury
            .assign(
                organization.aggregate_id(),
                OrganizationLedgerRoleAssignee::Team(team.aggregate_id()),
                OrganizationLedgerRole::Treasurer,
            )
            .unwrap();
        let mut account = Account::new();
        account
            .open(
                AccountOwner::Organization(organization.aggregate_id()),
                AccountName::try_from("Shared").unwrap(),
                CurrencyId::new(),
            )
            .unwrap();
        let mut personal = Account::new();
        personal
            .open(
                AccountOwner::User(member_id),
                AccountName::try_from("Personal").unwrap(),
                CurrencyId::new(),
            )
            .unwrap();
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(OrganizationOwnerRelation);
        model.define_relation(OrganizationAdminRelation);
        model.define_relation(OrganizationMemberRelation);
        model.define_relation(OrganizationMembershipUserRelation);
        model.define_relation(TeamMemberRelation);
        model.define_relation(TeamOrganizationRelation);
        model.define_relation(OrganizationFinanceManagerRelation);
        model.define_relation(OrganizationTreasurerRelation);
        model.define_relation(AccountManagerRelation);
        model.define_relation(AccountOwnerRelation);
        model.define_relation(AccountTransactorRelation);
        for phase in 0..5 {
            if phase == 1 {
                finance.revoke().unwrap();
                admin.demote_from_admin().unwrap();
            }
            if phase == 2 {
                member.remove().unwrap();
            }
            if phase == 4 {
                team.remove().unwrap();
            }
            let mut relationships = model.derive(&organization).unwrap();
            relationships.extend(model.derive(&member).unwrap());
            relationships.extend(model.derive(&admin).unwrap());
            relationships.extend(model.derive(&team).unwrap());
            relationships.extend(model.derive(&team_member).unwrap());
            relationships.extend(model.derive(&finance).unwrap());
            relationships.extend(model.derive(&treasury).unwrap());
            relationships.extend(model.derive(&account).unwrap());
            relationships.extend(model.derive(&personal).unwrap());
            if phase >= 3 {
                let mut rejoined = OrganizationMembership::new();
                rejoined
                    .create(organization.aggregate_id(), member_id)
                    .unwrap();
                relationships.extend(model.derive(&rejoined).unwrap());
            }
            let resolver = DefaultRelationshipResolver::new(
                TestRelationshipStore(relationships),
                model.clone(),
                RelationshipResolverConfig::default(),
            );
            for user_id in [owner_id, admin_id, member_id, outsider_id] {
                for (index, (requirement, allowed)) in [
                    (
                        RelationshipRequirement::check::<Organization>(
                            organization.aggregate_id(),
                            OrganizationFinanceManagerRelation::REF,
                        ),
                        phase == 0,
                    ),
                    (
                        RelationshipRequirement::check::<Organization>(
                            organization.aggregate_id(),
                            OrganizationTreasurerRelation::REF,
                        ),
                        phase < 2,
                    ),
                    (
                        RelationshipRequirement::check::<Account>(
                            account.aggregate_id(),
                            AccountManagerRelation::REF,
                        ),
                        phase == 0,
                    ),
                    (
                        RelationshipRequirement::check::<Account>(
                            account.aggregate_id(),
                            AccountTransactorRelation::REF,
                        ),
                        phase < 2,
                    ),
                    (
                        RelationshipRequirement::check::<Account>(
                            personal.aggregate_id(),
                            AccountTransactorRelation::REF,
                        ),
                        true,
                    ),
                    (
                        RelationshipRequirement::check::<Organization>(
                            OrganizationId::new(),
                            OrganizationTreasurerRelation::REF,
                        ),
                        false,
                    ),
                ]
                .into_iter()
                .enumerate()
                {
                    assert_eq!(
                        resolver
                            .satisfies(
                                &mut TestUow,
                                &AggregateRef::from_id::<User>(user_id),
                                &requirement
                            )
                            .await
                            .unwrap(),
                        (index < 4 && (user_id == owner_id || (user_id == admin_id && phase == 0)))
                            || (user_id == member_id && allowed),
                        "phase={phase}, requirement={requirement:?}"
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn role_assignment_management_follows_organization_admin_rights() {
        use super::{
            OrganizationLedgerRoleAssignmentManagerRelation,
            OrganizationLedgerRoleAssignmentOrganizationRelation,
        };
        use banking_iam_application::authorization::OrganizationAdminRelation;
        use banking_ledger_domain::{
            OrganizationLedgerRole, OrganizationLedgerRoleAssignee,
            OrganizationLedgerRoleAssignment,
        };

        let owner_id = UserId::new();
        let admin_id = UserId::new();
        let stranger_id = UserId::new();
        let mut organization = Organization::new();
        organization
            .create(
                OrganizationOwner::User(owner_id),
                OrganizationHandle::try_from("role-admin").unwrap(),
                OrganizationDisplayName::try_from("Role Admin").unwrap(),
            )
            .unwrap();
        let mut admin = OrganizationMembership::new();
        admin.create(organization.aggregate_id(), admin_id).unwrap();
        admin.promote_to_admin().unwrap();
        let mut assignment = OrganizationLedgerRoleAssignment::new();
        assignment
            .assign(
                organization.aggregate_id(),
                OrganizationLedgerRoleAssignee::OrganizationMembership(admin.aggregate_id()),
                OrganizationLedgerRole::Treasurer,
            )
            .unwrap();
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(OrganizationOwnerRelation);
        model.define_relation(OrganizationAdminRelation);
        model.define_relation(OrganizationLedgerRoleAssignmentOrganizationRelation);
        model.define_relation(OrganizationLedgerRoleAssignmentManagerRelation);
        for demoted in [false, true] {
            if demoted {
                admin.demote_from_admin().unwrap();
            }
            let mut relationships = model.derive(&organization).unwrap();
            relationships.extend(model.derive(&admin).unwrap());
            relationships.extend(model.derive(&assignment).unwrap());
            let resolver = DefaultRelationshipResolver::new(
                TestRelationshipStore(relationships),
                model.clone(),
                RelationshipResolverConfig::default(),
            );
            for user_id in [owner_id, admin_id, stranger_id] {
                for requirement in [
                    RelationshipRequirement::check::<Organization>(
                        organization.aggregate_id(),
                        OrganizationAdminRelation::REF,
                    ),
                    RelationshipRequirement::check::<OrganizationLedgerRoleAssignment>(
                        assignment.aggregate_id(),
                        OrganizationLedgerRoleAssignmentManagerRelation::REF,
                    ),
                ] {
                    assert_eq!(
                        resolver
                            .satisfies(
                                &mut TestUow,
                                &AggregateRef::from_id::<User>(user_id),
                                &requirement
                            )
                            .await
                            .unwrap(),
                        user_id == owner_id || (user_id == admin_id && !demoted)
                    );
                }
            }
        }
    }
}
