# Saga Design

Use this reference when an Appletheia application coordinates a workflow across aggregates.

## Workflow boundary

### DO use a saga only for cross-aggregate or multi-command coordination

Keep one aggregate's invariants inside that aggregate. A saga reacts to committed domain events,
dispatches commands, and reacts to terminal failures of commands it dispatched.

```text
committed event
  -> SagaEventWorker -> Saga::on_event -> command outbox
  -> CommandWorker -> success event or terminal CommandFailure
  -> SagaEventWorker or SagaCommandFailureWorker
```

The two saga workers have different inputs and contracts. Do not hide them behind one application
worker abstraction.

### DO declare runtime types on `Saga` and static metadata on `SagaSpec`

`Saga` owns the state, step, and error types used while handling a workflow. `SagaSpec` owns only
the descriptor that workers can inspect without constructing the saga.

```rust
impl SagaSpec for TransferSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("transfer"),
        START_EVENTS,
        SUBSCRIPTION,
    );
}

impl Saga for TransferSaga {
    type Spec = TransferSagaSpec;
    type State = TransferSagaState;
    type Step = TransferSagaStep;
    type Error = TransferSagaError;

    // ...
}
```

Use `S::State` and `S::Step` when generic code already has `S: Saga`. Do not route runtime types
through `S::Spec`; the spec is the stable descriptor boundary.

### DON'T use operation-failure events to drive a saga

If an aggregate refuses an operation, return a typed error. Do not append events such as
`FundsReserveRejected`, `CreateRejected`, or `CompleteRejected` merely so a saga can observe the
failure. Once a command becomes terminal, Appletheia durably publishes a `CommandFailureEnvelope`
for the originating saga.

A rejection or decline can still be a domain event when the rejection itself is the successful
business action. For example, `OrganizationJoinRequestEventPayload::Rejected` records that an
authorized actor rejected a pending request. That differs from a rejected attempt to execute a
command.

## Steps

### DO define the saga step as a user-owned serializable enum

`Saga::Step` identifies the logical command dispatch that produced a later event or terminal
failure. Give each dispatched operation one stable value.

```rust
use appletheia::application::saga::SagaStep;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferSagaStep {
    ReserveFunds,
    Deposit,
    ReleaseFunds,
    CommitFunds,
    CompensateDeposit,
    Complete,
    Fail,
}

impl SagaStep for TransferSagaStep {}
```

The framework serializes this value into `SagaCommandOrigin` and persists it with the dispatched
command. Do not maintain a parallel string step name or duplicate it in saga state.

### DO route follow-up events with the `causative_step` argument

`causative_step` is `None` only for a start event. An event caused by a saga-dispatched command is
routed with the step stored in that command's origin.

```rust
fn on_event(
    &self,
    instance: &mut SagaInstance<TransferSagaState, TransferSagaStep>,
    event: &EventEnvelope,
    causative_step: Option<TransferSagaStep>,
) -> Result<(), TransferSagaError> {
    if event.is_for_aggregate::<Account>() {
        let account_event = event.try_into_domain_event::<Account>()?;
        match account_event.payload() {
            AccountEventPayload::FundsReserved { .. }
                if causative_step == Some(TransferSagaStep::ReserveFunds) =>
            {
                let state = instance.state_required()?;
                instance.append_command(
                    CausationId::from(event.event_id),
                    TransferSagaStep::Deposit,
                    &AccountDepositCommand {
                        account_id: state.to_account_id,
                        amount: state.amount,
                    },
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}
```

Match both the event payload and causative step when the same event type can be produced by several
saga operations. Do not infer the previous operation from a custom state status.

### DON'T put saga steps in `RequestContext`

The step belongs to `SagaCommandOrigin`, not to caller identity or request metadata. Appletheia
persists the serialized step with the command and resolves it when routing its result.

## Dispatching commands

### DO append every saga command with causation and step

```rust
instance.append_command(
    CausationId::from(event.event_id),
    TransferSagaStep::ReserveFunds,
    &AccountFundsReserveCommand { account_id: from_account_id, amount },
)?;
```

From `on_event`, derive causation from `EventId`. From `on_command_failed`, derive it from
`CommandFailureId`.

```rust
instance.append_command(
    CausationId::from(failure.failure_id),
    TransferSagaStep::ReleaseFunds,
    &AccountReservedFundsReleaseCommand { account_id, amount },
)?;
```

This preserves the causal chain while each new command receives its own message ID.

### PREFER `append_command` over `append_command_with_options`

Use explicit options only when they differ from the defaults. Keep the argument order consistent:

```rust
instance.append_command_with_options(
    CausationId::from(event.event_id),
    TransferSagaStep::Complete,
    &TransferCompleteCommand { transfer_id },
    command_options,
)?;
```

Appending only adds an uncommitted command. The framework creates the dispatched-command record
from the persisted envelope and its `SagaCommandOrigin`; application code must not push one manually.

## Command failures

### DO handle compensation from `on_command_failed`

The method receives the terminal failure and the exact causative step that dispatched the command.
Branch on that step; command-name and step selectors are unnecessary in application code.

```rust
fn on_command_failed(
    &self,
    instance: &mut SagaInstance<TransferSagaState, TransferSagaStep>,
    failure: &CommandFailureEnvelope,
    causative_step: TransferSagaStep,
) -> Result<(), TransferSagaError> {
    match causative_step {
        TransferSagaStep::Deposit => {
            let state = instance.state_required()?;
            instance.append_command(
                CausationId::from(failure.failure_id),
                TransferSagaStep::ReleaseFunds,
                &AccountReservedFundsReleaseCommand {
                    account_id: state.from_account_id,
                    amount: state.amount,
                },
            )?;
        }
        _ => instance.complete(),
    }
    Ok(())
}
```

Every saga must implement `on_command_failed` explicitly. If a saga needs no compensation policy,
call `instance.complete()` so the terminal decision remains visible in application code.

### DO treat `CommandFailureEnvelope` as a terminal notification

The command worker retries retryable errors while attempts remain. It emits the notification only
after a non-retryable error or exhausted attempts. Handler changes have already rolled back; the
worker records `failed_at` in its own durable boundary and publishes through the failure outbox.

Do not retry inside the saga and do not construct a failure envelope in a command handler.

### DON'T subscribe to command failures as domain events

Event subscriptions select start and continuation events. Command failures are routed separately by
the saga name in `SagaCommandOrigin`, then correlated with the persisted dispatched command. There is
no application `CommandFailureSelector` or failure event payload to register.

## Saga state and lifecycle

### DO keep saga state to workflow data and readiness facts

Store only identifiers, values needed by later commands, and independent facts needed to join
parallel branches.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransferSagaState {
    pub transfer_id: TransferId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
}

impl SagaState for TransferSagaState {}
```

For parallel work, booleans or result slots may be appropriate because they represent independent
observations. A linear `Pending -> Reserving -> Depositing -> Completing` status duplicates the
framework step and should not be stored.

### DO let `SagaInstance` own completion status

Call `instance.complete()` when the workflow should no longer handle events or command failures.
This method also discards uncommitted commands. Keep the business outcome in domain events instead
of duplicating successful or failed status in the saga instance or user-defined state.

```rust
match transfer_event.payload() {
    TransferEventPayload::Completed
        if causative_step == Some(TransferSagaStep::Complete) => instance.complete(),
    TransferEventPayload::Failed { .. }
        if causative_step == Some(TransferSagaStep::Fail) => instance.complete(),
    _ => {}
}
```

### DO rely on framework persistence for duplicate delivery

Appletheia records processed events and processed command failures with uniqueness constraints. Saga
logic should remain deterministic, but it should not add ad hoc consumed flags or synthetic state to
replace the framework idempotency boundary.

## Subscriptions and boundaries

### DO keep event subscriptions explicit and narrow

Subscribe only to event payloads the saga starts from or actually consumes. Do not subscribe to every
event of an aggregate and do not include removed operation-rejection events.

### DON'T load or mutate aggregates directly from a saga

A saga coordinates through commands. Direct aggregate access bypasses command authorization,
idempotency, retryability, command execution tracking, and the command outbox.

### DON'T put request-scoped authority in saga state

If a later command needs an actor or issuer as domain data, include the required value in the start
event or saga state explicitly. Do not depend on an ambient `RequestContext` surviving asynchronous
delivery.
