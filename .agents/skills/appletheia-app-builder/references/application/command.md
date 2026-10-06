# Command Design

Use this reference for command data, application boundaries, and transactional execution.

### DO keep delivery metadata in the command envelope

Command fields express the requested business operation. `CommandEnvelope` and `SagaCommandOrigin`
already carry correlation, causation, saga identity, and step metadata; do not duplicate them in the
command payload. An actor or issuer ID that is part of the business operation may still be explicit
command data. Keep input minimal: supply the target ID and requested changes, and load authoritative
current values through the repository instead of asking callers to repeat them.

**Good**

```rust
pub struct AccountFundsReserveCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
}
```

**Bad**

```rust
pub struct AccountFundsReserveCommand {
    pub account_id: AccountId,
    pub amount: CurrencyAmount,
    pub saga_step: TransferSagaStep,
    pub causation_id: CausationId,
}
```

### PREFER structs for a single successful output shape

Return identifiers or data the caller needs after a successful command. A failed operation belongs in
the handler's typed error, not in an `Output::Rejected` branch.

**Good**

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountFundsReserveOutput {}
```

Use an empty named-field struct for an acknowledgement so JSON serialization produces `{}` rather
than `null`. Put returned identifiers or values directly in public struct fields, for example
`OrganizationCreateOutput { organization_id }`; avoid a single success enum variant that adds no distinction.
Implement `CommandOutput` and its replay representation consistently with neighboring outputs.

Use an enum when success genuinely has distinct shapes, as with Banking's `OidcCompleteOutput`
(token, exchange code, or identity linkage). Replay safety is a separate concern, described below.

**Bad**

```rust
pub enum AccountFundsReserveOutput {
    Reserved,
}
```

### DO let CommandOutput own a replay-safe representation

The dispatcher persists the output's `ReplayOutput` for idempotent replay. Implement this policy on
the output itself so every handler return path follows it. Use `CommandReplayOutput::Borrowed(self)`
only when the full output is safe to store. Return an owned replay DTO when the immediate response
contains tokens, exchange codes, or other credentials that must not be persisted and replayed.

**Good**

```rust
impl CommandOutput for OidcCompleteOutput {
    type ReplayOutput = OidcCompleteReplayOutput;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Owned(self.replay_safe_output())
    }
}
```

**Bad**

```rust
// Persists credentials from the immediate OIDC response.
impl CommandOutput for OidcCompleteOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
```

### DO coordinate cross-aggregate checks and authorization in the application layer

Declare access requirements through `CommandHandler::authorization_plan`; use application services
for additional policy or cross-aggregate checks. The aggregate enforces its own invariants without
querying another root or reading request-scoped authority. The dispatcher evaluates authorization
against `RequestContext.principal`; `actor` records provenance and is not a substitute for the
current principal. Do not rely on a persisted actor to recover request-scoped permissions.

Use aggregate repositories and their unique/reference indexes for write-side checks. Read models
may lag, so do not use a projection as the authority for a write invariant. Let the authorization
abstraction handle relationship checks rather than reimplementing them with a relationship store.

For example, organization creation looks up an existing handle owner before calling `create`.
The lookup enables a useful domain error; it does not replace the persisted uniqueness constraint
needed for concurrent requests. Keep save errors observable too.

**Good**

```rust
let unique_value = Self::handle_unique_value(&handle)?;
if self.organization_repository
    .find_by_unique_value(uow, OrganizationState::HANDLE_KEY, &unique_value)
    .await?
    .is_some()
{
    return Err(OrganizationError::HandleAlreadyTaken.into());
}

organization.create(owner, handle, display_name)?;
self.organization_repository.save(uow, request_context, &mut organization).await?;
```

**Bad**

```text
Organization::create -> read request claims -> query other organizations through a repository
```

### DO group required aggregate operations in one unit of work

For sign-in that creates a user and links an identity, Banking's OIDC handler performs:

**Good**

```rust
user.register()?;
user.link_identity(provider.clone(), subject.clone(), email)?;
self.user_repository.save(uow, request_context, &mut user).await?;
```

The events are recorded together; no intermediate save or separate registration command is needed.
An `Err` must prevent saving partial work. Database rollback does not undo an in-memory aggregate
already changed by `append_event`, so do not catch an error and later persist that partial instance.
Atomic persistence does not imply atomic delivery or projection of the two events.

Likewise, do not hide an aggregate operation behind a handler-side equality or lifecycle no-op.
Call the operation and propagate its outcome; an accepted command still needs its event for Saga
continuations. Coordinate follow-up commands in separate transactions through a saga rather than
turning a handler into a second workflow runner.

**Bad**

```text
register -> commit transaction A
link required identity -> transaction B fails
user remains partially registered
```

### DO classify retryability at the handler error boundary

Keep repository and external-service failures in handler errors rather than aggregate errors.
Implement `Retryability` there, delegating to nested infrastructure errors and classifying domain
failures by whether repeating the same command can help. Return failures as `Err`; see
[Aggregate Design](../domain/aggregate.md).

**Good**

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

**Bad**

```rust
impl Retryability for AccountFundsReserveCommandHandlerError {
    fn is_retryable(&self) -> bool {
        true
    }
}
```

### DO leave transaction and delivery control to the dispatcher and worker

Use `DefaultCommandDispatcher` for ordinary command execution, including synchronous entry points.
The handler uses the supplied unit of work, awaits required saves, and propagates failures. The
dispatcher handles commit/rollback and execution tracking; the worker handles delivery, attempt
limits, and terminal failure. Avoid bypassing these boundaries by calling a handler directly from
production entry points.

For saga-originated commands, the worker records terminal failure and enqueues its notification in
a separate transaction after handler rollback. A handler must not publish `CommandFailureEnvelope`
itself: an outbox write in its failed transaction would roll back too. Execution retries and outbox
publication retries are distinct. Saga compensation is a business reaction after terminal failure,
not a replacement for execution retry.

Database rollback cannot undo external effects. Use stable idempotency keys or another recovery
strategy for external operations that may run again.

**Good**

```text
Entry point -> dispatcher -> handler using supplied unit of work
  success -> commit
  error -> rollback
Worker -> retry if eligible; otherwise record terminal failure and its outbox notification
```

**Bad**

```text
Handler -> publish failure notification directly -> return Err
Database rollback -> notification has already escaped the transaction
```

### CONSIDER sharing a command worker when its configuration fits all consumers

A `DefaultCommandWorker` accepts a handler in `run_forever`, so compatible handlers can share its
dependencies. Each call creates its own consumer. Share only when retry settings and graceful-stop
behavior should also be shared; separate workers are appropriate when those requirements differ.

**Good**

```rust
let deposit_worker = Arc::clone(&worker);
tokio::spawn(async move {
    deposit_worker.run_forever(&account_deposit_handler).await
});

let reserve_worker = Arc::clone(&worker);
tokio::spawn(async move {
    reserve_worker.run_forever(&account_funds_reserve_handler).await
});
```

**Bad**

```text
Share one worker -> request independent shutdown of just one of its consumers via its shared stop flag
```
