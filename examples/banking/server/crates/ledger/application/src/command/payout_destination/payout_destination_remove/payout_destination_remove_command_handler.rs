use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::payout_destination::{PayoutDestination, PayoutDestinationRemoveResult};

use super::{
    PayoutDestinationRemoveCommand, PayoutDestinationRemoveCommandHandlerError,
    PayoutDestinationRemoveOutput,
};
use crate::authorization::PayoutDestinationRemoverRelation;

/// Handles `PayoutDestinationRemoveCommand`.
pub struct PayoutDestinationRemoveCommandHandler<PDR>
where
    PDR: Repository<PayoutDestination>,
{
    payout_destination_repository: PDR,
}

impl<PDR> PayoutDestinationRemoveCommandHandler<PDR>
where
    PDR: Repository<PayoutDestination>,
{
    pub fn new(payout_destination_repository: PDR) -> Self {
        Self {
            payout_destination_repository,
        }
    }
}

impl<PDR> CommandHandler for PayoutDestinationRemoveCommandHandler<PDR>
where
    PDR: Repository<PayoutDestination>,
{
    type Command = PayoutDestinationRemoveCommand;
    type Output = PayoutDestinationRemoveOutput;
    type ReplayOutput = PayoutDestinationRemoveOutput;
    type Error = PayoutDestinationRemoveCommandHandlerError;
    type Uow = PDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                PayoutDestination,
            >(
                command.payout_destination_id,
                PayoutDestinationRemoverRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let mut payout_destination = self
            .payout_destination_repository
            .read(uow, command.payout_destination_id)
            .await?;

        let result = payout_destination.remove()?;

        self.payout_destination_repository
            .save(uow, request_context, &mut payout_destination)
            .await?;

        let output = match result {
            PayoutDestinationRemoveResult::Removed => PayoutDestinationRemoveOutput::Removed,
            PayoutDestinationRemoveResult::Rejected { reason } => {
                PayoutDestinationRemoveOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
