use appletheia::application::saga::SagaError;
use appletheia::application::saga::{
    Saga, SagaContext, SagaDefinition, SagaDefinitionBuilder, SagaName, SagaRouteBuilder,
};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalEventPayload, WithdrawalFailureReason,
};

use super::{WithdrawalSagaHandlerError, WithdrawalSagaState, WithdrawalSagaStep};
use crate::command::{
    AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
    AccountReservedFundsReleaseCommand, WithdrawalFailCommand, WithdrawalSettlementExecuteCommand,
    WithdrawalSucceedCommand,
};

/// Coordinates the withdrawal flow.
pub struct WithdrawalSaga;

impl Saga for WithdrawalSaga {
    type State = WithdrawalSagaState;
    type Step = WithdrawalSagaStep;
    type HandlerError = WithdrawalSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "withdrawal",
        ))
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::ReserveFunds)
                .on::<Withdrawal>(WithdrawalEventPayload::REQUESTED)
                .handle(|ctx, withdrawal_event| {
                    if let WithdrawalEventPayload::Requested {
                        account_id, amount, ..
                    } = withdrawal_event.payload()
                    {
                        ctx.set_state(WithdrawalSagaState::new(
                            withdrawal_event.aggregate_id(),
                            *account_id,
                            *amount,
                        ));
                        ctx.append_command(&AccountFundsReserveCommand {
                            account_id: *account_id,
                            amount: *amount,
                        })?;
                    }
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::CommitFunds)
                .on::<Withdrawal>(WithdrawalEventPayload::SETTLEMENT_EXECUTED)
                .caused_by(WithdrawalSagaStep::ExecuteSettlement)
                .handle(|ctx, _withdrawal_event| {
                    let state: &mut WithdrawalSagaState = ctx.state_required_mut()?;
                    let account_id = state.account_id;
                    let amount = state.amount;
                    ctx.append_command(&AccountReservedFundsCommitCommand { account_id, amount })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::ExecuteSettlement)
                .on::<Account>(AccountEventPayload::FUNDS_RESERVED)
                .caused_by(WithdrawalSagaStep::ReserveFunds)
                .handle(|ctx, _account_event| {
                    let state: &mut WithdrawalSagaState = ctx.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    ctx.append_command(&WithdrawalSettlementExecuteCommand { withdrawal_id })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::Fail)
                .on::<Account>(AccountEventPayload::RESERVED_FUNDS_RELEASED)
                .caused_by(WithdrawalSagaStep::ReleaseFunds)
                .handle(|ctx, _account_event| {
                    let state: &mut WithdrawalSagaState = ctx.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    ctx.append_command(&WithdrawalFailCommand {
                        withdrawal_id,
                        reason: WithdrawalFailureReason::SettlementExecuteRejected,
                    })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::Succeed)
                .on::<Account>(AccountEventPayload::RESERVED_FUNDS_COMMITTED)
                .caused_by(WithdrawalSagaStep::CommitFunds)
                .handle(|ctx, _account_event| {
                    let state: &mut WithdrawalSagaState = ctx.state_required_mut()?;
                    let withdrawal_id = state.withdrawal_id;
                    ctx.append_command(&WithdrawalSucceedCommand { withdrawal_id })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::Fail)
                .on_command_failed::<AccountFundsReserveCommand>(WithdrawalSagaStep::ReserveFunds)
                .handle(|ctx, _command| {
                    self.append_fail_after_failure(
                        ctx,
                        WithdrawalFailureReason::FundsReserveRejected,
                    )?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::ReleaseFunds)
                .on_command_failed::<WithdrawalSettlementExecuteCommand>(
                    WithdrawalSagaStep::ExecuteSettlement,
                )
                .handle(|ctx, _command| {
                    let state: &mut WithdrawalSagaState = ctx.state_required_mut()?;
                    let account_id = state.account_id;
                    let amount = state.amount;
                    ctx.append_command(&AccountReservedFundsReleaseCommand { account_id, amount })?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::Fail)
                .on_command_failed::<AccountReservedFundsReleaseCommand>(
                    WithdrawalSagaStep::ReleaseFunds,
                )
                .handle(|ctx, _command| {
                    self.append_fail_after_failure(
                        ctx,
                        WithdrawalFailureReason::ReservedFundsReleaseRejected,
                    )?;
                    Ok(())
                }),
        )
        .add_route(
            SagaRouteBuilder::new(WithdrawalSagaStep::Fail)
                .on_command_failed::<AccountReservedFundsCommitCommand>(
                    WithdrawalSagaStep::CommitFunds,
                )
                .handle(|ctx, _command| {
                    self.append_fail_after_failure(
                        ctx,
                        WithdrawalFailureReason::ReservedFundsCommitRejected,
                    )?;
                    Ok(())
                }),
        )
        .build()
        .map_err(SagaError::from)
    }
}

impl WithdrawalSaga {
    fn append_fail_after_failure(
        &self,
        ctx: &mut SagaContext<'_, WithdrawalSagaState, WithdrawalSagaStep>,
        reason: WithdrawalFailureReason,
    ) -> Result<(), WithdrawalSagaHandlerError> {
        let state: &mut WithdrawalSagaState = ctx.state_required_mut()?;
        let withdrawal_id = state.withdrawal_id;
        ctx.append_command(&WithdrawalFailCommand {
            withdrawal_id,
            reason,
        })?;
        Ok(())
    }
}
