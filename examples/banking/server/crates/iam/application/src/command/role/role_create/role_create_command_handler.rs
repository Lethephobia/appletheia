use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_domain::Role;

use super::{RoleCreateCommand, RoleCreateCommandHandlerError, RoleCreateOutput};

/// Handles `RoleCreateCommand`.
pub struct RoleCreateCommandHandler<RR>
where
    RR: Repository<Role>,
{
    role_repository: RR,
}

impl<RR> RoleCreateCommandHandler<RR>
where
    RR: Repository<Role>,
{
    pub fn new(role_repository: RR) -> Self {
        Self { role_repository }
    }
}

impl<RR> CommandHandler for RoleCreateCommandHandler<RR>
where
    RR: Repository<Role>,
{
    type Command = RoleCreateCommand;
    type Output = RoleCreateOutput;
    type ReplayOutput = RoleCreateOutput;
    type Error = RoleCreateCommandHandlerError;
    type Uow = RR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let mut role = Role::default();
        role.create(command.name)?;

        self.role_repository
            .save(uow, request_context, &mut role)
            .await?;

        let role_id = role
            .aggregate_id()
            .ok_or(RoleCreateCommandHandlerError::MissingRoleId)?;
        let output = RoleCreateOutput::new(role_id);

        Ok(CommandHandled::same(output))
    }
}
