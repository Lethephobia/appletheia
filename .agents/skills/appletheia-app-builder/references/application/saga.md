# Saga Design

Use this reference for workflows that react to committed events and coordinate commands across
transactions. For command execution and failure publication, see [Command Design](command.md).

### DO distinguish the triggering step from the outgoing step

Build each route with `SagaRouteBuilder::new(step)` and pass the completed route to
`SagaDefinitionBuilder::add_route` in `Saga::definition(&self)`. The constructor's step identifies
outgoing commands; `caused_by` identifies the preceding step. An event route without `caused_by`
starts an instance. Subscriptions are derived from `event_selectors()` and
`command_failure_selectors()`; failure subscriptions filter by command name.

When a handler reads state fields without establishing the state type, annotate the state binding
(e.g. `let state: &TransferSagaState = ctx.state_required()?;`) so the independent route can infer it.

| Route | Input |
| --- | --- |
| Start | `on::<Aggregate>(event_name).handle(handler)` |
| Continuation | `on::<Aggregate>(event_name).caused_by(step).handle(handler)` |
| Command failure | `on_command_failed::<Command>(step).handle(handler)` |

Define a step enum with `#[saga_step]`; it derives `Copy`, equality, serde, and `SagaStep`, using
adjacently tagged snake-case JSON (`type` / `data`) by default. Use `#[derive(SagaStep)]` when managing
the other derives and serialization yourself. Variants may contain `Copy` values; these values take
part in equality and route matching, so keep changing workflow progress in SagaState.

A step identifies a command-dispatch stage, not a persisted current position. One route may append
multiple commands, all carrying its outgoing step. Failure routes match both the originating step and `Command::NAME`. Their handlers receive
the decoded command, so different command types can have separate failure routes on the same step. Duplicate input conditions cannot be distinguished merely by changing the outgoing step.

This continuation excerpt handles the reservation's result and dispatches a deposit. A complete
workflow must also register its start and required outcome paths.

**Good**

```rust
builder.add_route(
    SagaRouteBuilder::new(TransferSagaStep::Deposit)
        .on::<Account>(AccountEventPayload::FUNDS_RESERVED)
        .caused_by(TransferSagaStep::ReserveFunds)
        .handle(|ctx, _event| {
            let state: &TransferSagaState = ctx.state_required()?;
            let command = AccountDepositCommand {
                account_id: state.to_account_id,
                amount: state.amount,
            };
            ctx.append_command(&command)?;
            Ok(())
        }),
)
```

**Bad**

```rust
// Deposit cannot have caused the reservation result this route waits for.
SagaRouteBuilder::new(TransferSagaStep::Deposit)
    .on::<Account>(AccountEventPayload::FUNDS_RESERVED)
    .caused_by(TransferSagaStep::Deposit)
```

### DO keep definition construction free of execution side effects

`Saga` declares `State`, `Step`, and `HandlerError`; `definition` returns
`Result<SagaDefinition<'_, Self::State, Self::Step, Self::HandlerError>, SagaError>`.
Finish the builder with `build().map_err(SagaError::from)`.

Pass `&saga` to the event and command-failure workers' `run_forever` methods. Each builds a definition
once at startup, so running both builds it twice. Register callbacks there rather than executing
business operations during construction.

Callbacks are synchronous `Fn` closures and may borrow injected services through `&self`, subject to
`Send + Sync`. They receive a typed event or the decoded command that failed. Use commands for asynchronous
work. Define a handler error with `From<EventEnvelopeError>`, `From<CommandFailureEnvelopeError>`, and conversions for the context errors
used by callbacks; these are execution errors, separate from definition-construction errors.

**Good**

```rust
builder
    .add_route(
        SagaRouteBuilder::new(TransferSagaStep::ReserveFunds)
            .on::<Transfer>(TransferEventPayload::REQUESTED)
            .handle(|ctx, event| {
                if let TransferEventPayload::Requested {
                    from_account_id,
                    to_account_id,
                    amount,
                    ..
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
            }),
    )
    .build()
    .map_err(SagaError::from)
```

**Bad**

```text
Worker startup -> definition() -> perform business operation immediately -> return routes
Second worker startup -> definition() -> perform the same business operation again
```

### DO dispatch workflow commands through SagaContext

`ctx.append_command(&command)` adds the route's step, saga origin, and current input's causation ID.
The runner saves queued commands, saga state, and processed-input records in one transaction.

Direct publication or direct aggregate mutation from a callback bypasses this dispatch bookkeeping
and the command handler's authorization, execution tracking, and transaction boundary. Keep each
saga centered on a business workflow; react to its actual lifecycle facts instead of inventing
parent-aggregate events solely to launch commands for another aggregate.

Do not persist `RequestContext` or request-scoped authority in saga state. Carry an actor or issuer ID
as explicit business data only when later commands need it; command authorization remains the
application's responsibility.

**Good**

```rust
ctx.append_command(&AccountDepositCommand { account_id, amount })?;
```

**Bad**

```text
Callback -> publish deposit command directly -> no saved saga dispatch ownership
Deposit result -> runner cannot match it to the saga's continuation
```

### DO design continuations around command ownership, not correlation alone

The runner first looks for a saved command matching the event's causation ID. An owned command
selects a continuation by its saved step, even if the event also matches a start route. A missing
continuation does not fall back to starting the saga.

Without an owned command, a start route creates an instance per `(saga_name, start_event_id)`.
Different start events may share a correlation ID and create separate instances, including events
emitted by the same command. Correlation groups related work; it does not identify an instance.
Redelivery of the same start event yields `AlreadyStarted` once its instance exists. Processing is
also deduplicated by `(saga_name, event_id)` within the transaction. Start-once does not mean callbacks
execute exactly once: rolled-back processing may retry. Owned command results still follow the
continuation path rather than creating another instance, even if they match a start selector.

**Good**

```text
Saga dispatches ReserveFunds -> resulting event references that command -> continuation
Distinct unowned start events + same correlation -> separate instances
Same start event redelivered -> AlreadyStarted
```

**Bad**

```text
Unrelated event shares correlation ID -> assume it continues the existing saga
```

### DO track readiness explicitly when parallel branches must join

Step routing identifies which command caused an input; it does not prove that all parallel commands
have finished. Store only the workflow's required pending IDs or branch facts and dispatch the next
command when the join condition is met. Avoid copying entire aggregate state or adding a linear
phase machine solely to repeat checks already provided by ownership-based routing.

Framework deduplication prevents processing the same input repeatedly. It does not replace a
business guard against completing the join twice through different valid inputs.

**Good**

```text
Dispatch closure commands for accounts A and B -> record pending {A, B}
A result -> record A handled -> B still pending -> do not dispatch final command
B result -> no pending accounts -> dispatch the final command once
```

**Bad**

```text
Dispatch closure commands for accounts A and B
First result from closure step -> assume both accounts are handled -> dispatch final command
```

### DO register failure reactions only when the workflow needs them

Use failure routes for compensation or another business reaction to terminal command failure.
An owned failure without a matching route is recorded and acknowledged; no empty handler is needed.
The command worker owns execution retries and terminal notification, as described in
[Command Design](command.md).

Saga has no framework completion operation. A later owned input may still run a matching route.
Store business facts and guard callbacks when the workflow must suppress a later reaction; framework
deduplication handles repeated inputs. Domain completion events and commands remain valid.

**Good**

```rust
builder.add_route(
    SagaRouteBuilder::new(TransferSagaStep::ReleaseFunds)
        .on_command_failed::<AccountDepositCommand>(TransferSagaStep::Deposit)
        .handle(|ctx, _command| {
            let state: &TransferSagaState = ctx.state_required()?;
            let command = AccountReservedFundsReleaseCommand {
                account_id: state.from_account_id,
                amount: state.amount,
            };
            ctx.append_command(&command)?;
            Ok(())
        }),
)
```

**Bad**

```rust
// Added solely to consume the final notification or mark the saga finished.
builder.add_route(
    SagaRouteBuilder::new(TransferSagaStep::Succeed)
        .on_command_failed::<TransferSucceedCommand>(TransferSagaStep::Succeed)
        .handle(|_ctx, _command| Ok(())),
)
```
