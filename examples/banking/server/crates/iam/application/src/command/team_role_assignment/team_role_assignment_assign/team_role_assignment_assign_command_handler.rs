use super::{
    TeamRoleAssignmentAssignCommand, TeamRoleAssignmentAssignCommandHandlerError,
    TeamRoleAssignmentAssignOutput,
};
use crate::authorization::TeamManagerRelation;
use appletheia::application::{
    authorization::{AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement},
    command::CommandHandler,
    repository::Repository,
    request_context::RequestContext,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::TeamRoleAssignment;
use banking_iam_domain::{Organization, OrganizationMembership, Team, TeamMembership};
use banking_iam_domain::{TeamRoleAssignee, TeamRoleAssignmentState};

pub struct TeamRoleAssignmentAssignCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> TeamRoleAssignmentAssignCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for TeamRoleAssignmentAssignCommandHandler<R>
where
    R: Repository,
{
    type Command = TeamRoleAssignmentAssignCommand;
    type Output = TeamRoleAssignmentAssignOutput;
    type Error = TeamRoleAssignmentAssignCommandHandlerError;
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
            return Err(TeamRoleAssignmentAssignCommandHandlerError::TeamInactive);
        }
        match command.assignee {
            TeamRoleAssignee::TeamMembership(id) => {
                let membership = self
                    .repository
                    .read_shared::<TeamMembership>(uow, id)
                    .await?;
                if !membership.is_active()? {
                    return Err(TeamRoleAssignmentAssignCommandHandlerError::AssigneeInactive);
                }
                if membership.team_id()? != command.team_id {
                    return Err(TeamRoleAssignmentAssignCommandHandlerError::TeamMismatch);
                }
                let organization = self
                    .repository
                    .read_shared::<Organization>(uow, team.organization_id()?)
                    .await?;
                if !organization.is_active()? {
                    return Err(TeamRoleAssignmentAssignCommandHandlerError::OrganizationInactive);
                }
                let organization_membership = self
                    .repository
                    .read_shared::<OrganizationMembership>(
                        uow,
                        membership.organization_membership_id()?,
                    )
                    .await?;
                if !organization_membership.is_active()? {
                    return Err(TeamRoleAssignmentAssignCommandHandlerError::AssigneeInactive);
                }
                if organization_membership.organization_id()? != team.organization_id()? {
                    return Err(TeamRoleAssignmentAssignCommandHandlerError::OrganizationMismatch);
                }
            }
        }
        let unique_value = command
            .assignee
            .assignment_unique_value(command.team_id, command.role)?;
        if self
            .repository
            .find_shared_by_unique_value::<TeamRoleAssignment>(
                uow,
                TeamRoleAssignmentState::TEAM_ASSIGNEE_ROLE_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(TeamRoleAssignmentAssignCommandHandlerError::AlreadyAssigned);
        }
        let mut assignment = TeamRoleAssignment::new();
        assignment.assign(command.team_id, command.assignee, command.role)?;
        self.repository
            .save(uow, request_context, &mut assignment)
            .await?;
        Ok(TeamRoleAssignmentAssignOutput {
            team_role_assignment_id: assignment.aggregate_id(),
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
        OrganizationOwner, Team, TeamMembership, TeamName, UserId,
    };

    use super::{
        TeamRoleAssignmentAssignCommand, TeamRoleAssignmentAssignCommandHandler,
        TeamRoleAssignmentAssignCommandHandlerError,
    };
    use appletheia::application::command::CommandHandler;
    use banking_iam_domain::{
        TeamRole, TeamRoleAssignee, TeamRoleAssignment, TeamRoleAssignmentState,
    };

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
            assert_eq!(key, TeamRoleAssignmentState::TEAM_ASSIGNEE_ROLE_KEY);
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
    async fn assignment_validates_scope_membership_and_uniqueness_with_shared_reads() {
        for case in 0..8 {
            let mut organization = Organization::new();
            organization
                .create(
                    OrganizationOwner::User(UserId::new()),
                    OrganizationHandle::try_from("roles").unwrap(),
                    OrganizationDisplayName::try_from("Roles").unwrap(),
                )
                .unwrap();
            let mut membership = OrganizationMembership::new();
            membership
                .create(
                    if case == 7 {
                        Organization::new().aggregate_id()
                    } else {
                        organization.aggregate_id()
                    },
                    UserId::new(),
                )
                .unwrap();
            let mut team = Team::new();
            team.create(
                organization.aggregate_id(),
                TeamName::try_from("Operations").unwrap(),
            )
            .unwrap();
            let mut team_membership = TeamMembership::new();
            team_membership
                .create(
                    if case == 2 {
                        Team::new().aggregate_id()
                    } else {
                        team.aggregate_id()
                    },
                    membership.aggregate_id(),
                )
                .unwrap();
            if case == 1 {
                team_membership.remove().unwrap();
            }
            if case == 3 {
                team.remove().unwrap();
            }
            if case == 5 {
                membership.remove().unwrap();
            }
            if case == 6 {
                organization.remove().unwrap();
            }
            let assignee = TeamRoleAssignee::TeamMembership(team_membership.aggregate_id());
            let role = TeamRole::Manager;
            let command = TeamRoleAssignmentAssignCommand {
                team_id: team.aggregate_id(),
                assignee,
                role,
            };
            let duplicate = if case == 4 {
                let mut assignment = TeamRoleAssignment::new();
                assignment.assign(command.team_id, assignee, role).unwrap();
                Some(SerializedAggregate::try_from_aggregate(&assignment).unwrap())
            } else {
                None
            };
            let repository = TestRepository {
                aggregates: HashMap::from([
                    (
                        AggregateRef::from_id::<Organization>(organization.aggregate_id()),
                        SerializedAggregate::try_from_aggregate(&organization).unwrap(),
                    ),
                    (
                        AggregateRef::from_id::<OrganizationMembership>(membership.aggregate_id()),
                        SerializedAggregate::try_from_aggregate(&membership).unwrap(),
                    ),
                    (
                        AggregateRef::from_id::<Team>(team.aggregate_id()),
                        SerializedAggregate::try_from_aggregate(&team).unwrap(),
                    ),
                    (
                        AggregateRef::from_id::<TeamMembership>(team_membership.aggregate_id()),
                        SerializedAggregate::try_from_aggregate(&team_membership).unwrap(),
                    ),
                ]),
                duplicate,
                unique_value: assignee
                    .assignment_unique_value(command.team_id, role)
                    .unwrap(),
            };
            let handler = TeamRoleAssignmentAssignCommandHandler::new(repository);
            let context = RequestContext::new(
                CorrelationId::from(MessageId::new().value()),
                MessageId::new(),
                Principal::System,
            )
            .unwrap();
            let mut uow = TestUow::default();
            let result = handler.handle(&mut uow, &context, &command).await;
            match case {
                0 => {
                    let output = result.unwrap();
                    assert_eq!(uow.shared_reads.len(), 4);
                    assert_eq!(uow.saved.len(), 1);
                    let saved = uow.saved[0]
                        .try_to_aggregate::<TeamRoleAssignment>()
                        .unwrap();
                    assert_eq!(saved.aggregate_id(), output.team_role_assignment_id);
                    assert_eq!(saved.assignee().unwrap(), assignee);
                    assert_eq!(saved.role().unwrap(), role);
                }
                1 => assert!(matches!(
                    result,
                    Err(TeamRoleAssignmentAssignCommandHandlerError::AssigneeInactive)
                )),
                2 => assert!(matches!(
                    result,
                    Err(TeamRoleAssignmentAssignCommandHandlerError::TeamMismatch)
                )),
                3 => assert!(matches!(
                    result,
                    Err(TeamRoleAssignmentAssignCommandHandlerError::TeamInactive)
                )),
                4 => assert!(matches!(
                    result,
                    Err(TeamRoleAssignmentAssignCommandHandlerError::AlreadyAssigned)
                )),
                5 => assert!(matches!(
                    result,
                    Err(TeamRoleAssignmentAssignCommandHandlerError::AssigneeInactive)
                )),
                6 => assert!(matches!(
                    result,
                    Err(TeamRoleAssignmentAssignCommandHandlerError::OrganizationInactive)
                )),
                7 => assert!(matches!(
                    result,
                    Err(TeamRoleAssignmentAssignCommandHandlerError::OrganizationMismatch)
                )),
                _ => unreachable!(),
            }
            if case != 0 {
                assert!(uow.saved.is_empty());
            }
        }
    }
}
