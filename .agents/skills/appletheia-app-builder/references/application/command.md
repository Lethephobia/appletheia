# Command Design

Use this reference for Appletheia command payloads, handlers, retryability, authorization, and
terminal failure behavior.

## Command payloads

### DO make a command describe one requested operation

Use domain value objects instead of transport primitives and include the aggregate identifier needed
to load the target.

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountFundsReserveCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
```

Do not put a saga name, instance ID, step, correlation ID, or causation ID in the payload. The
`CommandEnvelope` carries message metadata and `SagaCommandOrigin`.

### PREFER output that represents successful completion

Return identifiers or data the caller needs after a successful command. A failed operation belongs in
the handler's typed error, not in an `Output::Rejected` branch.

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountFundsReserveOutput;
```

## Handler boundary

### DO keep the handler transaction focused

A normal aggregate handler should:

1. Load or create the aggregate.
2. Perform application authorization and cross-aggregate lookup when required.
3. Call an aggregate command method.
4. Save the aggregate.
5. Return a successful output.

```rust
async fn handle(
    &self,
    command: &AccountFundsReserveCommand,
    context: &RequestContext,
    uow: &mut Uow,
) -> Result<AccountFundsReserveOutput, AccountFundsReserveCommandHandlerError> {
    let mut account = self.repository.find(command.account_id, uow).await?;
    account.reserve_funds(command.amount)?;
    self.repository.save(&mut account, context, uow).await?;
    Ok(AccountFundsReserveOutput)
}
```

Let `?` preserve the typed failure. Do not append a compensating failure event or save an otherwise
unchanged aggregate solely to report refusal.

### DO keep aggregate invariants in aggregate methods

The command handler may coordinate repositories, reference indexes, policies, and external services.
Rules that depend only on aggregate state belong in the aggregate. Map the aggregate error into the
handler error without changing its retryability.

### DO keep authorization in the application boundary

Resolve the current principal through the application's authorization abstraction. Domain aggregates
should not read `RequestContext.actor` or transport claims directly.

### DON'T use ambient request metadata as domain input

If issuer, actor, or provenance must be replayable, pass an explicit value object to the aggregate so
the emitted event contains it. A saga command's step remains in `SagaCommandOrigin`, not in the
payload or request context.

## Errors and retryability

### DO return typed errors for refused operations

Expected domain refusals are permanent handler errors unless retrying the same command can succeed
without another business action.

```rust
#[derive(Debug, Error)]
pub enum AccountFundsReserveCommandHandlerError {
    #[error(transparent)]
    AccountRepository(#[from] RepositoryError<Account>),

    #[error(transparent)]
    Account(#[from] AccountError),
}

impl Retryability for AccountFundsReserveCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::AccountRepository(error) => error.is_retryable(),
            Self::Account(_) => false,
        }
    }
}
```

Do not make every error retryable. Insufficient balance, invalid lifecycle state, duplicate domain
identity, malformed domain input, and failed authorization are normally permanent. Transient
database, network, or service availability errors may be retryable.

### DO distinguish a successful rejection decision from operation failure

The word "rejected" does not determine the model; the completed business action does.

- Rejecting a pending organization join request is a successful command. Persist a `Rejected` event
  because the request changed from pending to rejected.
- Failing to reserve funds is a failed command. Return an `AccountError`; do not persist a
  `FundsReserveRejected` event.

Apply the same test to `Declined`, `Denied`, `Failed`, and similar names: did the aggregate complete a
business transition, or did the requested operation fail to happen?

### DON'T encode command failure as a successful output

Avoid `Ok(Output::Rejected { reason })` for aggregate or application failures. It commits the handler
transaction and hides failure from command-worker retryability and terminal-failure routing.

If an API needs a client-friendly representation, map the typed command error at the transport
boundary.

## Transaction and worker behavior

### DO share the command worker across handlers

Construct the worker from shared dispatcher, subscriber, execution-store, failure-outbox, and
unit-of-work dependencies. Pass each handler by reference when starting its consumer instead of
storing the handler in the worker.

```rust
let config = CommandWorkerConfig {
    lease_duration: CommandExecutionLeaseDuration::default(),
    retry_options: CommandExecutionRetryOptions {
        max_attempts: CommandExecutionMaxAttempts::default(),
    },
};
let worker = Arc::new(DefaultCommandWorker::new(dependencies, config));

let deposit_worker = Arc::clone(&worker);
tokio::spawn(async move {
    deposit_worker.run_forever(&account_deposit_handler).await
});

let reserve_worker = Arc::clone(&worker);
tokio::spawn(async move {
    reserve_worker.run_forever(&account_funds_reserve_handler).await
});
```

Each call derives both its consumer group and subscription selector from `H::Command::NAME`. The
worker's config and graceful-stop flag are shared by every handler consumer running on that worker.
The lease duration controls abandoned-execution recovery, while retry options control whether
another handler attempt remains available.

### DO rely on rollback for handler errors

When a handler returns `Err`, aggregate events, repository writes, and ordinary outbox writes in that
unit of work roll back. Therefore terminal command failure cannot be published from the failed handler
transaction.

The command worker owns the durable failure boundary:

```text
dispatch handler
  -> Ok: commit domain changes and ack
  -> Err(retryable) with attempts remaining: roll back, release lease, nack
  -> Err(non-retryable or exhausted): roll back, mark failed, enqueue CommandFailure, ack
```

Command-outbox publication retry and command-execution retry are separate concerns.

### DO let `Retryability` drive the retry decision

The worker has the attempt count obtained when command execution begins and compares it with the
configured maximum. Handler code should only classify its error; it must not count attempts, sleep,
nack, or release execution leases.

### DON'T construct or publish `CommandFailureEnvelope` in a handler

The worker creates a new `CommandFailureId`, reuses the persisted `failed_at`, and publishes the
notification when the command is terminal. For saga-originated commands the envelope includes the
original `SagaCommandOrigin`; the saga failure worker uses it to route the failure.

## Cross-aggregate validation

### DO use reference indexes or repositories for application-level uniqueness

An aggregate cannot enforce facts owned by another aggregate. Perform the lookup before the target
mutation and return a typed non-retryable error when a conflicting owner exists.

```rust
if self.handle_index.owner_of(&command.handle, uow).await?.is_some() {
    return Err(OrganizationCreateCommandHandlerError::HandleAlreadyTaken);
}

let mut organization = Organization::new();
organization.create(command.organization_id, command.handle.clone())?;
self.repository.save(&mut organization, context, uow).await?;
```

Do not create an empty aggregate stream or append a `CreateRejected` event to record this lookup
failure.

### PREFER one durable owner for each invariant

If a unique value is reserved through a dedicated aggregate or registry, command that owner first and
coordinate later work with a saga. Keep compensation explicit for terminal failures.

## Idempotency and side effects

### DO dispatch through `DefaultCommandDispatcher`

Use the standard dispatcher even for direct dispatch so command execution, idempotency, unit-of-work,
and output persistence behave consistently. Do not invent a no-op command execution store for an
alternate path.

### DO keep external effects behind retry-aware application abstractions

Calls to object storage, identity providers, blockchains, email, or other services must declare
whether their failures are retryable. Prefer idempotency keys derived from stable message identity for
effects that may be invoked again.

### DON'T report success before durable state is ready

Return `Ok` only after the aggregate and required outbox work have been recorded in the handler unit
of work. Do not use an output variant to mask partial work.

## Testing

### DO test both retryability classes

For each handler, cover at least:

- successful state change and emitted event;
- permanent aggregate or policy failure with no emitted event;
- retryable infrastructure failure;
- mapping from nested errors to `Retryability`;
- rollback of domain and outbox changes on `Err`.

For saga-originated commands, also test that a terminal failure is routed to the saga with the
original step. That worker-level behavior does not belong in the handler unit test.
