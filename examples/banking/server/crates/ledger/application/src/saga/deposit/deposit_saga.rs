use appletheia::application::saga::SagaError;
use appletheia::application::saga::{Saga, SagaDefinition, SagaDefinitionBuilder, SagaName};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::deposit::{Deposit, DepositEventPayload, DepositFailureReason};

use super::{DepositSagaHandlerError, DepositSagaState, DepositSagaStep};
use crate::command::{AccountDepositCommand, DepositCompleteCommand, DepositFailCommand};

/// Coordinates the deposit flow.
pub struct DepositSaga;

impl Saga for DepositSaga {
    type State = DepositSagaState;
    type Step = DepositSagaStep;
    type HandlerError = DepositSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(SagaName::new(
            "deposit",
        ))
        .add_start_step(DepositSagaStep::Deposit)
        .on::<Deposit>(DepositEventPayload::SETTLEMENT_VERIFIED)
        .handle(|ctx, deposit_event| {
            if let DepositEventPayload::SettlementVerified {
                account_id, amount, ..
            } = deposit_event.payload()
            {
                ctx.set_state(DepositSagaState::new(
                    deposit_event.aggregate_id(),
                    *account_id,
                    *amount,
                ));
                ctx.append_command(&AccountDepositCommand {
                    account_id: *account_id,
                    amount: *amount,
                })?;
            }
            Ok(())
        })
        .add_step(DepositSagaStep::Complete)
        .on::<Account>(DepositSagaStep::Deposit, AccountEventPayload::DEPOSITED)
        .handle(|ctx, _account_event| {
            let state = ctx.state_required_mut()?;
            let deposit_id = state.deposit_id;

            ctx.append_command(&DepositCompleteCommand { deposit_id })?;
            Ok(())
        })
        .add_failure_step(DepositSagaStep::Fail)
        .on(DepositSagaStep::Deposit)
        .handle(|ctx, _failure| {
            let state = ctx.state_required_mut()?;
            let deposit_id = state.deposit_id;
            ctx.append_command(&DepositFailCommand {
                deposit_id,
                reason: DepositFailureReason::AccountDepositRejected,
            })?;
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}
