# Saga Design

Use this reference when an Appletheia application coordinates a workflow across aggregates.
Verify the API in the target checkout when working against a different library version.

## Workflow boundary

### DO use a saga for cross-aggregate or multi-command coordination

Keep one aggregate's invariants inside that aggregate. A saga reacts to committed domain events,
appends commands, and optionally reacts to terminal failures of commands it dispatched.

```text
committed event
  -> SagaEventWorker -> SagaRunner -> selected callback -> command outbox
  -> CommandWorker -> committed event or terminal CommandFailureEnvelope
  -> SagaEventWorker or SagaCommandFailureWorker
```

The event and command-failure workers have different inputs and contracts. Keep them separate.

### DON'T use operation-failure events to drive a saga

Return a typed error when an operation is refused. Do not append `FundsReserveRejected`,
`CreateRejected`, or `CompleteRejected` merely to notify a saga. The command worker durably emits
`CommandFailureEnvelope` for a saga-owned command when execution becomes terminal.

Preserve genuine business facts: an authorized rejection of a pending join request is a successful
business action that can emit `Rejected`. Do not remove events based only on their names.

## Definition and registration

### DO implement `Saga::definition` with the staged builder

`Saga` declares `State`, `Step`, and `HandlerError`. Its only application callback-registration
method is `definition(&self)`. Construct the builder there; the worker supplies the saga, not a
builder. Use `add_start_step(...).on(...).handle(...)`, `add_step(...).on(...).handle(...)`, and
`add_failure_step(...).on(...).handle(...)`, then finish with `build().map_err(SagaError::from)`.

This Transfer excerpt illustrates the API using Banking types. It is not a complete transfer
workflow: register the remaining commit, release, compensation, and business-outcome paths too.

```rust
use appletheia::application::saga::{
    Saga, SagaDefinition, SagaDefinitionBuilder, SagaError, SagaName,
};

pub struct TransferSaga;

impl Saga for TransferSaga {
    type State = TransferSagaState;
    type Step = TransferSagaStep;
    type HandlerError = TransferSagaHandlerError;

    fn definition(
        &self,
    ) -> Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError> {
        SagaDefinitionBuilder::<Self::State, Self::Step, Self::HandlerError>::new(
            SagaName::new("transfer"),
        )
        .add_start_step(TransferSagaStep::ReserveFunds)
        .on::<Transfer>(TransferEventPayload::REQUESTED)
        .handle(|ctx, event| {
            if let TransferEventPayload::Requested {
                from_account_id, to_account_id, amount, ..
            } = event.payload()
            {
                ctx.set_state(TransferSagaState::new(
                    event.aggregate_id(),
                    *from_account_id,
                    *to_account_id,
                    *amount,
                ));
                ctx.append_command(&AccountFundsReserveCommand {
                    account_id: *from_account_id,
                    amount: *amount,
                })?;
            }
            Ok(())
        })
        .add_step(TransferSagaStep::Deposit)
        .on::<Account>(
            TransferSagaStep::ReserveFunds,
            AccountEventPayload::FUNDS_RESERVED,
        )
        .handle(|ctx, _event| {
            let state = ctx.state_required()?;
            let command = AccountDepositCommand {
                account_id: state.to_account_id,
                amount: state.amount,
            };
            ctx.append_command(&command)?;
            Ok(())
        })
        .add_failure_step(TransferSagaStep::ReleaseFunds)
        .on(TransferSagaStep::Deposit)
        .handle(|ctx, _failure| {
            let state = ctx.state_required()?;
            let command = AccountReservedFundsReleaseCommand {
                account_id: state.from_account_id,
                amount: state.amount,
            };
            ctx.append_command(&command)?;
            Ok(())
        })
        .build()
        .map_err(SagaError::from)
    }
}
```

`add_*_step` selects the step assigned to outgoing commands. `on` selects the input:

| Route | `on` arguments |
| --- | --- |
| Start | `on::<Aggregate>(event_name)` |
| Continuation | `on::<Aggregate>(caused_by, event_name)` |
| Terminal command failure | `on(caused_by)` |

The incoming `caused_by` step and outgoing step have different roles. `handle` completes the route
and returns the definition builder. Context and event types are inferred; do not add route aliases
or closure type annotations unless the actual compiler requires them.

There is no Saga spec, descriptor, separate start-event list, condition/trigger VO, or prepared
definition to declare. `SagaDefinition` is a concrete struct, not a trait. Do not copy Projector's
static descriptor design onto Saga.

### DO let the definition derive subscriptions and the runner execute callbacks

`on::<Aggregate>` creates the route's `EventSelector`. `SagaDefinition` stores only its name and
routes; `selectors()` returns a deduplicated vector for the event worker to retain when subscribing.
Register only events the workflow consumes, instead of maintaining another subscription list.

Definition construction rejects duplicate start selectors, duplicate `(selector, caused_by)`
continuations, and duplicate failure steps. Different outgoing steps do not distinguish otherwise
identical input conditions. Start and continuation routes may share an event selector.

For custom runner work, `find_event_route(event, caused_by)` and
`find_command_failure_route(caused_by)` return route references without executing them. Runner owns
ownership checks, deduplication, `SagaContext` construction, callback execution, and persistence.
Ordinary application sagas only register callbacks; do not perform those runner operations inside
`definition`.

### DO distinguish construction errors from handler errors

Name application callback errors for their role, for example `TransferSagaHandlerError`.
Wrap `SagaContextError` with a `#[from]` variant alongside relevant application errors:

```rust
use appletheia::application::saga::SagaContextError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferSagaHandlerError {
    #[error(transparent)]
    Context(#[from] SagaContextError),
}
```

`SagaDefinitionError` contains validation errors. `SagaDefinitionBuilderError` wraps it, and
`SagaError` wraps the builder error. None of these construction errors has a handler-error type
parameter. Execution errors are held in `SagaRunnerError<HandlerError>::Route` through
`SagaRouteError<HandlerError>`. Envelope decoding and event-name validation belong to the route
adapter, not to the application's typed event callback.

## Steps and commands

### DO define the step as a user-owned serializable enum

Implement `SagaStep` for a `Copy` enum with stable serialized variants, using
`#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]` and
`#[serde(rename_all = "snake_case")]` for a fieldless step enum.

The step identifies a command-dispatch stage; it is not a persisted current position or a unique
route ID. Multiple routes may emit commands at the same step. One route may emit several commands,
including different command kinds, but all receive that route's step. Choose stages accordingly.
Even a route that emits no command declares a step; it is used only when appending commands.

The framework stores the step in each command's `SagaCommandOrigin`. Do not maintain a parallel
string step name, put steps in `RequestContext`, or duplicate a linear route position in state.

### DO append commands through `SagaContext`

Use `ctx.append_command(&command)` or, when defaults are insufficient,
`ctx.append_command_with_options(&command, options)`. Do not pass a step or causation ID. Context
adds the selected route's step, saga origin, and the input event ID or failure ID as causation.
Each command receives its own message ID. There is no per-command step override.

Appending queues uncommitted commands. Runner persists state, processed-input records, and command
outbox entries in one unit of work. Callback or outbox errors roll back that work. Application code
must not push dispatched-command records manually or invoke external side effects directly.

## Start and continuation semantics

### DO account for ownership-first routing and start-once behavior

Runner first resolves the input's causation command against saved saga dispatches. An owned command
selects a continuation by its saved step, even when the event also has a start route. No matching
continuation yields `NoMatchingRoute`; it never falls back to the start route.

Without an owned command, the input must match a start route. If the saga name and correlation ID
already identify an instance, Runner returns `AlreadyStarted` and the worker ACKs without rerunning
the start callback or changing state. This also applies to redelivery of the original start event.
Only a missing instance is created. Do not rely on repeated external start events as a continuation
mechanism; correlation alone does not authorize a continuation.

Start-once prevents committed reinitialization. It is not an exactly-once callback guarantee:
failed transactions can execute again. In PostgreSQL, competing new instances cannot overwrite the
same correlation; the losing transaction rolls back and can retry to observe `AlreadyStarted`.

### DO keep only workflow data and necessary business guards in state

Store identifiers, values required by later commands, and independent facts needed to join parallel
branches. A domain completion command or event is still valid, but Saga has no shared completion
status, `complete()`, or `is_completed()` API. Do not add empty routes merely to end a saga.

Successful handlers preserve queued commands. A later owned input can still run its matching
continuation or failure handler. When business rules require ignoring such an input, guard on
state and return `Ok(())`; Runner still records and acknowledges it. Framework input deduplication
handles redelivery, so do not invent consumed flags to replace it.

## Terminal failures

### DO register failure routes only when the workflow needs a reaction

Use `add_failure_step(outgoing_step).on(caused_by).handle(...)` for compensation, recording a
business failure, or another required reaction. No command generic is needed. Do not register
`handle(|_ctx, _failure| Ok(()))` just to consume a notification.

An owned terminal failure with no matching route is recorded as processed and acknowledged with
`NoMatchingRoute`. It does not change state or dispatch commands. Redelivery and republication
remain deduplicated. Errors from a registered callback still roll back and are retryable.

When one step dispatches several command kinds that require different failure policies, inspect
the validated `failure.command_name` inside that step's callback. Step matching selects the route;
it does not guarantee one command kind per step.

### DO leave execution retries and failure publication to the command worker

The command worker retries retryable errors while attempts remain. Terminal notifications follow
a non-retryable error or exhausted attempts, after handler rollback, through a separate durable
failure boundary and outbox. Do not reproduce that retry loop or publish `CommandFailureEnvelope`
from a saga callback or command handler. A business recovery that dispatches a new command is a
separate workflow decision.

Command-failure subscriptions use the saga name and saved command ownership. They are not domain
events, and require no event selector in the application's definition.

## Workers and dependencies

### DO pass the saga to reusable workers and build definitions at startup

Inject dependencies into the saga, then pass `&saga` to the event and command-failure workers'
`run_forever` methods. Each worker calls `saga.definition()` once before subscribing, keeps that
definition for the run, and reports construction errors before consuming messages. Running both
workers constructs two definitions; keep `definition` deterministic and free of side effects.

Callbacks are synchronous `Fn` closures returning `Result<(), HandlerError>`. They receive
`&mut SagaContext` and a typed `&Event<Aggregate::Id, Aggregate::EventPayload>` or
`&CommandFailureEnvelope`. They may borrow injected services through `&self`; do not force a clone,
`Arc`, or `'static` capture merely to register a route. Captured references must satisfy the closure's
`Send + Sync` bounds. Use commands for asynchronous I/O rather than async route callbacks.

Reuse a worker across sagas when they share the subscriber and runner. Each `run_forever` call
creates its own consumer. A shared worker's stop flag affects all its consumers. If spawning tasks,
arrange task ownership separately from the borrowing permitted inside a definition. When joining
the two worker futures, convert their different error types to a common application error.

### DON'T load or mutate aggregates directly from a saga

Coordinate through commands to preserve authorization, idempotency, command-execution tracking,
retry policy, and the outbox boundary. Do not rely on ambient request-scoped authority surviving
asynchronous delivery. Carry actor or issuer identifiers as domain data only when later operations
need them, rather than persisting the whole `RequestContext` in saga state.
