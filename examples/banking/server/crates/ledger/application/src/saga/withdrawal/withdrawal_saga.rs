use appletheia::application::saga::SagaError;
use appletheia::application::saga::{
    Saga, SagaContext, SagaDefinition, SagaDefinitionBuilder, SagaName,
};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalEventPayload, WithdrawalFailureReason,
};

use super::{WithdrawalSagaHandlerError, WithdrawalSagaState, WithdrawalSagaStep};
use crate::command::{
    AccountFundsReserveCommand, AccountReservedFundsCommitCommand,
    AccountReservedFundsReleaseCommand, WithdrawalCompleteCommand, WithdrawalFailCommand,
    WithdrawalSettlementExecuteCommand,
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
        .add_start_step(WithdrawalSagaStep::ReserveFunds)
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
        })
        .add_step(WithdrawalSagaStep::CommitFunds)
        .on::<Withdrawal>(
            WithdrawalSagaStep::ExecuteSettlement,
            WithdrawalEventPayload::SETTLEMENT_EXECUTED,
        )
        .handle(|ctx, _withdrawal_event| {
            let state = ctx.state_required_mut()?;
            let account_id = state.account_id;
            let amount = state.amount;
            ctx.append_command(&AccountReservedFundsCommitCommand { account_id, amount })?;
            Ok(())
        })
        .add_step(WithdrawalSagaStep::ExecuteSettlement)
        .on::<Account>(
            WithdrawalSagaStep::ReserveFunds,
            AccountEventPayload::FUNDS_RESERVED,
        )
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let withdrawal_id = state.withdrawal_id;
            ctx.append_command(&WithdrawalSettlementExecuteCommand { withdrawal_id })?;
            Ok(())
        })
        .add_step(WithdrawalSagaStep::Fail)
        .on::<Account>(
            WithdrawalSagaStep::ReleaseFunds,
            AccountEventPayload::RESERVED_FUNDS_RELEASED,
        )
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let withdrawal_id = state.withdrawal_id;
            ctx.append_command(&WithdrawalFailCommand {
                withdrawal_id,
                reason: WithdrawalFailureReason::SettlementExecuteRejected,
            })?;
            Ok(())
        })
        .add_step(WithdrawalSagaStep::Complete)
        .on::<Account>(
            WithdrawalSagaStep::CommitFunds,
            AccountEventPayload::RESERVED_FUNDS_COMMITTED,
        )
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let withdrawal_id = state.withdrawal_id;
            ctx.append_command(&WithdrawalCompleteCommand { withdrawal_id })?;
            Ok(())
        })
        .add_failure_step(WithdrawalSagaStep::Fail)
        .on(WithdrawalSagaStep::ReserveFunds)
        .handle(|ctx, _failure| {
            self.append_fail_after_failure(ctx, WithdrawalFailureReason::FundsReserveRejected)?;
            Ok(())
        })
        .add_failure_step(WithdrawalSagaStep::ReleaseFunds)
        .on(WithdrawalSagaStep::ExecuteSettlement)
        .handle(|ctx, _failure| {
            let state = ctx.state_required_mut()?;
            let account_id = state.account_id;
            let amount = state.amount;
            ctx.append_command(&AccountReservedFundsReleaseCommand { account_id, amount })?;
            Ok(())
        })
        .add_failure_step(WithdrawalSagaStep::Fail)
        .on(WithdrawalSagaStep::ReleaseFunds)
        .handle(|ctx, _failure| {
            self.append_fail_after_failure(
                ctx,
                WithdrawalFailureReason::ReservedFundsReleaseRejected,
            )?;
            Ok(())
        })
        .add_failure_step(WithdrawalSagaStep::Fail)
        .on(WithdrawalSagaStep::CommitFunds)
        .handle(|ctx, _failure| {
            self.append_fail_after_failure(
                ctx,
                WithdrawalFailureReason::ReservedFundsCommitRejected,
            )?;
            Ok(())
        })
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
        let state = ctx.state_required_mut()?;
        let withdrawal_id = state.withdrawal_id;
        ctx.append_command(&WithdrawalFailCommand {
            withdrawal_id,
            reason,
        })?;
        Ok(())
    }
}
