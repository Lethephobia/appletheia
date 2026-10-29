use super::{
    OrganizationLedgerRoleAssignmentAssignCommand,
    OrganizationLedgerRoleAssignmentAssignCommandHandlerError,
    OrganizationLedgerRoleAssignmentAssignOutput,
};
use appletheia::application::{
    authorization::{AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement},
    command::CommandHandler,
    repository::Repository,
    request_context::RequestContext,
};
use appletheia::domain::Aggregate;
use banking_iam_application::organization::authorization::OrganizationAdminRelation;
use banking_iam_domain::{Organization, OrganizationMembership, Team};
use banking_ledger_domain::OrganizationLedgerRoleAssignment;
use banking_ledger_domain::{
    OrganizationLedgerRoleAssignee, OrganizationLedgerRoleAssignmentState,
};

pub struct OrganizationLedgerRoleAssignmentAssignCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationLedgerRoleAssignmentAssignCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for OrganizationLedgerRoleAssignmentAssignCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationLedgerRoleAssignmentAssignCommand;
    type Output = OrganizationLedgerRoleAssignmentAssignOutput;
    type Error = OrganizationLedgerRoleAssignmentAssignCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Organization,
            >(
                command.organization_id,
                OrganizationAdminRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let organization = self
            .repository
            .read_shared::<Organization>(uow, command.organization_id)
            .await?;
        if !organization.is_active()? {
            return Err(
                OrganizationLedgerRoleAssignmentAssignCommandHandlerError::OrganizationInactive,
            );
        }
        match command.assignee {
            OrganizationLedgerRoleAssignee::OrganizationMembership(id) => {
                let membership = self
                    .repository
                    .read_shared::<OrganizationMembership>(uow, id)
                    .await?;
                if !membership.is_active()? {
                    return Err(
                        OrganizationLedgerRoleAssignmentAssignCommandHandlerError::AssigneeInactive,
                    );
                }
                if membership.organization_id()? != command.organization_id {
                    return Err(OrganizationLedgerRoleAssignmentAssignCommandHandlerError::OrganizationMismatch);
                }
            }
            OrganizationLedgerRoleAssignee::Team(id) => {
                let team = self.repository.read_shared::<Team>(uow, id).await?;
                if !team.is_active()? {
                    return Err(
                        OrganizationLedgerRoleAssignmentAssignCommandHandlerError::AssigneeInactive,
                    );
                }
                if team.organization_id()? != command.organization_id {
                    return Err(OrganizationLedgerRoleAssignmentAssignCommandHandlerError::OrganizationMismatch);
                }
            }
        }
        let unique_value = command
            .assignee
            .assignment_unique_value(command.organization_id, command.role)?;
        if self
            .repository
            .find_shared_by_unique_value::<OrganizationLedgerRoleAssignment>(
                uow,
                OrganizationLedgerRoleAssignmentState::ORGANIZATION_ASSIGNEE_ROLE_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(OrganizationLedgerRoleAssignmentAssignCommandHandlerError::AlreadyAssigned);
        }
        let mut assignment = OrganizationLedgerRoleAssignment::new();
        assignment.assign(command.organization_id, command.assignee, command.role)?;
        self.repository
            .save(uow, request_context, &mut assignment)
            .await?;
        Ok(OrganizationLedgerRoleAssignmentAssignOutput {
            organization_ledger_role_assignment_id: assignment.aggregate_id(),
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
        OrganizationOwner, Team, TeamName, UserId,
    };

    use super::{
        OrganizationLedgerRoleAssignmentAssignCommand,
        OrganizationLedgerRoleAssignmentAssignCommandHandler,
        OrganizationLedgerRoleAssignmentAssignCommandHandlerError,
    };
    use appletheia::application::command::CommandHandler;
    use banking_ledger_domain::{
        OrganizationLedgerRole, OrganizationLedgerRoleAssignee, OrganizationLedgerRoleAssignment,
        OrganizationLedgerRoleAssignmentState,
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
            assert_eq!(
                key,
                OrganizationLedgerRoleAssignmentState::ORGANIZATION_ASSIGNEE_ROLE_KEY
            );
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
    async fn assign_validates_both_assignee_kinds_and_uses_shared_reads() {
        for to_team in [false, true] {
            for case in 0..5 {
                let mut organization = Organization::new();
                organization
                    .create(
                        OrganizationOwner::User(UserId::new()),
                        OrganizationHandle::try_from("roles").unwrap(),
                        OrganizationDisplayName::try_from("Roles").unwrap(),
                    )
                    .unwrap();
                let assignee_organization_id = if case == 2 {
                    Organization::new().aggregate_id()
                } else {
                    organization.aggregate_id()
                };
                let mut membership = OrganizationMembership::new();
                membership
                    .create(assignee_organization_id, UserId::new())
                    .unwrap();
                let mut team = Team::new();
                team.create(
                    assignee_organization_id,
                    TeamName::try_from("Finance").unwrap(),
                )
                .unwrap();
                if case == 1 {
                    membership.delete().unwrap();
                    team.delete().unwrap();
                }
                if case == 3 {
                    organization.delete().unwrap();
                }
                let assignee = if to_team {
                    OrganizationLedgerRoleAssignee::Team(team.aggregate_id())
                } else {
                    OrganizationLedgerRoleAssignee::OrganizationMembership(
                        membership.aggregate_id(),
                    )
                };
                let role = OrganizationLedgerRole::Treasurer;
                let command = OrganizationLedgerRoleAssignmentAssignCommand {
                    organization_id: organization.aggregate_id(),
                    assignee,
                    role,
                };
                let duplicate = if case == 4 {
                    let mut assignment = OrganizationLedgerRoleAssignment::new();
                    assignment
                        .assign(command.organization_id, assignee, role)
                        .unwrap();
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
                            AggregateRef::from_id::<OrganizationMembership>(
                                membership.aggregate_id(),
                            ),
                            SerializedAggregate::try_from_aggregate(&membership).unwrap(),
                        ),
                        (
                            AggregateRef::from_id::<Team>(team.aggregate_id()),
                            SerializedAggregate::try_from_aggregate(&team).unwrap(),
                        ),
                    ]),
                    duplicate,
                    unique_value: assignee
                        .assignment_unique_value(command.organization_id, role)
                        .unwrap(),
                };
                let handler = OrganizationLedgerRoleAssignmentAssignCommandHandler::new(repository);
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
                        assert_eq!(uow.shared_reads.len(), 2);
                        let saved = uow.saved[0].try_to_aggregate::<OrganizationLedgerRoleAssignment>().unwrap();
                        assert_eq!(saved.aggregate_id(), output.organization_ledger_role_assignment_id);
                        assert_eq!(saved.assignee().unwrap(), assignee);
                        assert_eq!(saved.role().unwrap(), role);
                    }
                    1 => assert!(matches!(result, Err(OrganizationLedgerRoleAssignmentAssignCommandHandlerError::AssigneeInactive))),
                    2 => assert!(matches!(result, Err(OrganizationLedgerRoleAssignmentAssignCommandHandlerError::OrganizationMismatch))),
                    3 => assert!(matches!(result, Err(OrganizationLedgerRoleAssignmentAssignCommandHandlerError::OrganizationInactive))),
                    _ => assert!(matches!(result, Err(OrganizationLedgerRoleAssignmentAssignCommandHandlerError::AlreadyAssigned))),
                }
                if case != 0 {
                    assert!(uow.saved.is_empty());
                }
            }
        }
    }
}
