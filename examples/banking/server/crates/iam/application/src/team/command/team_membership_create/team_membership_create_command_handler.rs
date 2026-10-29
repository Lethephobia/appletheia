use super::{
    TeamMembershipCreateCommand, TeamMembershipCreateCommandHandlerError,
    TeamMembershipCreateOutput,
};
use crate::team::authorization::TeamManagerRelation;
use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::UniqueValue;
use banking_iam_domain::{
    Organization, OrganizationMembership, Team, TeamMembership, TeamMembershipState,
};

pub struct TeamMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamMembershipCreateCommand;
    type Output = TeamMembershipCreateOutput;
    type Error = TeamMembershipCreateCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Team,
            >(
                command.team_id,
                TeamManagerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let team = self
            .repository
            .read_shared::<Team>(uow, command.team_id)
            .await?;
        if !team.is_active()? {
            return Err(TeamMembershipCreateCommandHandlerError::TeamDeleted);
        }
        let organization_membership = self
            .repository
            .read_shared::<OrganizationMembership>(uow, command.organization_membership_id)
            .await?;
        if !organization_membership.is_active()? {
            return Err(TeamMembershipCreateCommandHandlerError::OrganizationMembershipDeleted);
        }
        if team.organization_id()? != organization_membership.organization_id()? {
            return Err(TeamMembershipCreateCommandHandlerError::OrganizationMismatch);
        }
        let organization = self
            .repository
            .read_shared::<Organization>(uow, team.organization_id()?)
            .await?;
        if !organization.is_active()? {
            return Err(TeamMembershipCreateCommandHandlerError::OrganizationDeleted);
        }
        let team_id = command.team_id.to_string();
        let organization_membership_id = command.organization_membership_id.to_string();
        let unique_value =
            UniqueValue::from_strings([team_id.as_str(), organization_membership_id.as_str()])?;
        if self
            .repository
            .find_shared_by_unique_value::<TeamMembership>(
                uow,
                TeamMembershipState::TEAM_ORGANIZATION_MEMBERSHIP_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(TeamMembershipCreateCommandHandlerError::AlreadyMember);
        }
        let mut membership = TeamMembership::new();
        membership.create(command.team_id, command.organization_membership_id)?;
        self.repository
            .save(uow, request_context, &mut membership)
            .await?;
        Ok(TeamMembershipCreateOutput {
            team_membership_id: membership.aggregate_id(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use appletheia::application::aggregate::{AggregateRef, SerializedAggregate};
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::{Aggregate, AggregateVersion, UniqueKey, UniqueValue};
    use banking_iam_domain::{
        Organization, OrganizationDisplayName, OrganizationHandle, OrganizationMembership,
        OrganizationOwner, Team, TeamMembership, TeamMembershipState, TeamName, UserId,
    };

    use super::{
        TeamMembershipCreateCommand, TeamMembershipCreateCommandHandler,
        TeamMembershipCreateCommandHandlerError,
    };
    use appletheia::application::command::CommandHandler;

    #[derive(Default)]
    struct TestUow {
        shared_reads: Vec<AggregateRef>,
        saved: Vec<SerializedAggregate>,
    }

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    struct TestRepository {
        aggregates: HashMap<AggregateRef, SerializedAggregate>,
        duplicate: Option<SerializedAggregate>,
        unique_value: UniqueValue,
    }

    impl Repository for TestRepository {
        type Uow = TestUow;

        async fn read<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            panic!("creation must use shared reads for existing aggregates")
        }

        async fn read_shared<A: Aggregate>(
            &self,
            uow: &mut Self::Uow,
            id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            let target = AggregateRef::from_id::<A>(id);
            uow.shared_reads.push(target.clone());
            Ok(self
                .aggregates
                .get(&target)
                .unwrap()
                .try_to_aggregate::<A>()
                .unwrap())
        }

        async fn read_at_version<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _id: A::Id,
            _at: AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            unreachable!()
        }

        async fn find_by_unique_value<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _key: UniqueKey,
            _value: &UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            panic!("duplicate lookup must use a shared lock")
        }

        async fn find_shared_by_unique_value<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            key: UniqueKey,
            value: &UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            assert_eq!(key, TeamMembershipState::TEAM_ORGANIZATION_MEMBERSHIP_KEY);
            assert_eq!(value, &self.unique_value);
            Ok(self
                .duplicate
                .as_ref()
                .map(|aggregate| aggregate.try_to_aggregate::<A>().unwrap()))
        }

        async fn save<A: Aggregate>(
            &self,
            uow: &mut Self::Uow,
            _context: &RequestContext,
            aggregate: &mut A,
        ) -> Result<(), RepositoryError<A>> {
            uow.saved
                .push(SerializedAggregate::try_from_aggregate(aggregate).unwrap());
            Ok(())
        }
    }

    #[tokio::test]
    async fn creation_checks_lifecycle_organization_and_uniqueness_before_saving() {
        for case in 0..6 {
            let mut organization = Organization::new();
            organization
                .create(
                    OrganizationOwner::User(UserId::new()),
                    OrganizationHandle::try_from("team-test").unwrap(),
                    OrganizationDisplayName::try_from("Team Test").unwrap(),
                )
                .unwrap();
            let mut team = Team::new();
            team.create(
                organization.aggregate_id(),
                TeamName::try_from("Finance").unwrap(),
            )
            .unwrap();
            let mut organization_membership = OrganizationMembership::new();
            let membership_organization_id = if case == 3 {
                Organization::new().aggregate_id()
            } else {
                organization.aggregate_id()
            };
            organization_membership
                .create(membership_organization_id, UserId::new())
                .unwrap();
            if case == 1 {
                team.delete().unwrap();
            }
            if case == 2 {
                organization_membership.delete().unwrap();
            }
            if case == 4 {
                organization.delete().unwrap();
            }
            let mut duplicate = TeamMembership::new();
            duplicate
                .create(team.aggregate_id(), organization_membership.aggregate_id())
                .unwrap();
            let unique_value = duplicate
                .unique_entries()
                .unwrap()
                .get(TeamMembershipState::TEAM_ORGANIZATION_MEMBERSHIP_KEY)
                .unwrap()
                .first()
                .unwrap()
                .clone();
            let repository = TestRepository {
                aggregates: HashMap::from([
                    (
                        AggregateRef::from_id::<Organization>(organization.aggregate_id()),
                        SerializedAggregate::try_from_aggregate(&organization).unwrap(),
                    ),
                    (
                        AggregateRef::from_id::<Team>(team.aggregate_id()),
                        SerializedAggregate::try_from_aggregate(&team).unwrap(),
                    ),
                    (
                        AggregateRef::from_id::<OrganizationMembership>(
                            organization_membership.aggregate_id(),
                        ),
                        SerializedAggregate::try_from_aggregate(&organization_membership).unwrap(),
                    ),
                ]),
                duplicate: (case == 5)
                    .then(|| SerializedAggregate::try_from_aggregate(&duplicate).unwrap()),
                unique_value,
            };
            let handler = TeamMembershipCreateCommandHandler::new(repository);
            let context = RequestContext::new(
                CorrelationId::from(MessageId::new().value()),
                MessageId::new(),
                Principal::System,
            )
            .unwrap();
            let mut uow = TestUow::default();
            let result = handler
                .handle(
                    &mut uow,
                    &context,
                    &TeamMembershipCreateCommand {
                        team_id: team.aggregate_id(),
                        organization_membership_id: organization_membership.aggregate_id(),
                    },
                )
                .await;
            match case {
                0 => {
                    let output = result.unwrap();
                    assert_eq!(
                        uow.shared_reads,
                        vec![
                            AggregateRef::from_id::<Team>(team.aggregate_id()),
                            AggregateRef::from_id::<OrganizationMembership>(
                                organization_membership.aggregate_id()
                            ),
                            AggregateRef::from_id::<Organization>(organization.aggregate_id())
                        ]
                    );
                    assert_eq!(uow.saved.len(), 1);
                    let saved = uow.saved[0].try_to_aggregate::<TeamMembership>().unwrap();
                    assert_eq!(saved.aggregate_id(), output.team_membership_id);
                    assert_eq!(saved.team_id().unwrap(), team.aggregate_id());
                    assert_eq!(
                        saved.organization_membership_id().unwrap(),
                        organization_membership.aggregate_id()
                    );
                }
                1 => assert!(matches!(
                    result,
                    Err(TeamMembershipCreateCommandHandlerError::TeamDeleted)
                )),
                2 => assert!(matches!(
                    result,
                    Err(TeamMembershipCreateCommandHandlerError::OrganizationMembershipDeleted)
                )),
                3 => assert!(matches!(
                    result,
                    Err(TeamMembershipCreateCommandHandlerError::OrganizationMismatch)
                )),
                4 => assert!(matches!(
                    result,
                    Err(TeamMembershipCreateCommandHandlerError::OrganizationDeleted)
                )),
                5 => assert!(matches!(
                    result,
                    Err(TeamMembershipCreateCommandHandlerError::AlreadyMember)
                )),
                _ => unreachable!(),
            }
            if case != 0 {
                assert!(uow.saved.is_empty());
            }
        }
    }
}
