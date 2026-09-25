# Aggregate Design

Use this reference for Appletheia aggregate boundaries, command methods, state transitions, events,
and domain errors.

## Aggregate boundary

### DO start from the aggregate's state, events, and behavior

Before designing handlers or sagas, identify:

- the state whose invariants must change atomically;
- the successful business facts worth replaying;
- the operations that can produce those facts;
- the typed reasons an operation can fail.

An aggregate is the consistency boundary. Keep behavior that needs several aggregate states in an
application service or saga rather than injecting repositories into the aggregate.

### DO give the aggregate one stable identity

Use a dedicated `AggregateId` value object. Child objects inside the boundary have their own domain
identities when useful, but they are loaded and persisted through the root.

### PREFER small aggregate state

Persist only the values required to enforce invariants or answer future commands. Read-model display
data, request metadata, saga progress, and infrastructure timestamps do not belong in aggregate state.

## Command methods

### DO express behavior as methods on the aggregate

```rust
impl Account {
    pub fn reserve_funds(&mut self, amount: CurrencyAmount) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                return Err(AccountError::FundsReserveRejected(
                    AccountFundsReserveRejectionReason::Frozen,
                ));
            }
            AccountStatus::Closed => {
                return Err(AccountError::FundsReserveRejected(
                    AccountFundsReserveRejectionReason::Closed,
                ));
            }
        }

        if self.available_balance()? < amount {
            return Err(AccountError::FundsReserveRejected(
                AccountFundsReserveRejectionReason::InsufficientAvailableBalance,
            ));
        }

        self.append_event(AccountEventPayload::FundsReserved { amount })
    }
}
```

The method validates current state, returns a typed error on refusal, and appends only the successful
event. Do not let a handler reproduce these state rules.

### DO return typed errors for operations that do not happen

An attempted state change that is refused is an error, even when refusal is expected in the business
domain. Preserve structured reasons in the aggregate error so the application and transport layers
can map them without parsing strings.

```rust
#[derive(Debug, Error)]
pub enum AccountError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<AccountId>),

    #[error(transparent)]
    State(#[from] AccountStateError),

    #[error("account funds reservation rejected: {0:?}")]
    FundsReserveRejected(AccountFundsReserveRejectionReason),
}
```

Using `Rejected` in an error variant is fine. The removed pattern is an operation-failure
`EventPayload::...Rejected` event.

### DON'T append an event for an operation failure

Avoid event variants such as:

- `FundsReserveRejected` when no funds were reserved;
- `CreateRejected` when an aggregate was not created;
- `NameChangeRejected` when the name did not change;
- `CompleteRejected` when completion did not happen.

Those attempts do not change aggregate state and do not belong in the aggregate stream. Returning
`Err` lets the application classify retryability and lets the command worker notify an originating
saga after terminal failure.

### DO preserve meaningful rejection and decline events

Keep a rejection event when rejection is itself the requested state transition.

```rust
pub enum OrganizationJoinRequestEventPayload {
    Submitted { organization_id: OrganizationId, user_id: UserId },
    Approved,
    Rejected { reason: OrganizationJoinRequestRejectionReason },
    Cancelled,
}
```

Here a `RejectOrganizationJoinRequest` command succeeds by moving a pending request to rejected. The
event is a durable business fact. The same reasoning applies to an invitation being declined.

Ask: "Did the command successfully perform a rejection decision, or was its requested operation
rejected?" Persist only the former.

### DO validate before appending a success event

Run every invariant check before `append_event`. Once an event is appended it is part of the pending
change set and should not be undone by application code.

```rust
pub fn change_name(&mut self, name: AccountName) -> Result<(), AccountError> {
    let state = self.state_required()?;
    if state.status.is_closed() {
        return Err(AccountError::NameChangeRejected(
            AccountNameChangeRejectionReason::AccountClosed,
        ));
    }
    if state.name == name {
        return Ok(());
    }

    self.append_event(AccountEventPayload::NameChanged { name })
}
```

If an already-satisfied request is intentionally idempotent, return `Ok(())` without an event. If it
is a domain violation, return the typed error. Make that choice explicit per operation.

### DON'T use constructor misuse as a business rejection

Calling a one-shot `create` or `open` method twice on the same initialized instance is aggregate API
misuse. Return a structural error such as `AlreadyOpened`; do not append a `CreateRejected` event.

## Events

### DO name events as completed facts

Use past-tense facts such as `Opened`, `FundsReserved`, `NameChanged`, `Closed`, `Approved`, or
`Rejected`. Event payloads contain all domain data needed to replay the transition.

```rust
#[event_payload(error = AccountEventPayloadError)]
pub enum AccountEventPayload {
    Opened {
        owner: AccountOwner,
        name: AccountName,
        currency_id: CurrencyId,
    },
    FundsReserved { amount: CurrencyAmount },
    NameChanged { name: AccountName },
    Closed,
}
```

Do not include handler errors, repository errors, retryability, request context, saga step, or API
response details in the domain event.

### DO keep event application deterministic and exhaustive

`AggregateApply` reconstructs state from payload alone. It performs no I/O, reads no clock, and emits
no further events.

```rust
impl AggregateApply<AccountEventPayload, AccountError> for Account {
    fn apply(&mut self, payload: &AccountEventPayload) -> Result<(), AccountError> {
        match payload {
            AccountEventPayload::Opened { owner, name, currency_id } => {
                self.set_state(Some(AccountState::new(
                    owner.clone(),
                    name.clone(),
                    *currency_id,
                )));
            }
            AccountEventPayload::FundsReserved { amount } => {
                self.state_required_mut()?.balance.reserve(*amount)?;
            }
            AccountEventPayload::NameChanged { name } => {
                self.state_required_mut()?.name = name.clone();
            }
            AccountEventPayload::Closed => {
                self.state_required_mut()?.status = AccountStatus::Closed;
            }
        }
        Ok(())
    }
}
```

Do not add a wildcard arm to hide a missing transition. Exhaustive matching makes schema evolution
visible at compile time.

### DON'T mutate state outside event application

Aggregate command methods call `append_event`; the apply implementation owns state mutation. Direct
mutation before appending makes live execution diverge from replay.

## State and value objects

### DO use dedicated value objects for domain concepts

Validate names, money, currency, URLs, and identifiers at construction. Store valid values in events
and state so replay does not repeat transport validation.

### DO use `state_required` and `state_required_mut`

Use the shared aggregate helpers instead of unwrapping optional state. Their typed errors preserve the
difference between an uninitialized aggregate and a domain refusal.

### PREFER domain status only when it is a real aggregate fact

An `AccountStatus::Frozen` or `JoinRequestStatus::Rejected` can be essential to future invariants.
Keep it. Do not add statuses for handler execution, command retries, saga steps, or persistence
bookkeeping.

## Uniqueness and references

### DO derive unique and reference entries from current state

Implement Appletheia's unique/reference entry contracts from materialized aggregate state. These
indexes are persistence aids for domain ownership and lookup, not alternative mutable state.

### DON'T enforce cross-aggregate uniqueness inside one aggregate

The aggregate cannot know whether another root owns a handle, external identity, or currency code.
Let the application query the reference/unique index and return a typed handler error, or model a
dedicated owner aggregate and coordinate it through a saga.

Do not initialize a losing aggregate merely to store a rejection event.

## Child entities and collections

### DO enforce collection invariants at the root

The aggregate root decides whether a child can be added, removed, or changed. Return a typed error for
duplicates or invalid lifecycle state and append a success event only when the collection changes.

```rust
pub fn link_identity(&mut self, identity: ExternalIdentity) -> Result<(), UserError> {
    if self.state_required()?.identities.contains(&identity) {
        return Err(UserError::IdentityAlreadyLinked(identity));
    }

    self.append_event(UserEventPayload::IdentityLinked { identity })
}
```

### PREFER event payloads that identify the affected child

Include the child's stable ID and the values needed for deterministic apply. Avoid positional indexes
whose meaning changes as a collection is reordered.

## Error boundaries

### DO separate aggregate errors from application errors

Aggregate errors cover invalid aggregate usage, state access, value transitions, and invariant
refusals. Repository, authorization, network, and external-service failures belong in handler errors.
The handler error implements `Retryability`; aggregate domain failures normally map to
non-retryable.

### DON'T add retryability to domain events

Retryability describes execution failure and may depend on infrastructure. It is not a historical
fact about aggregate state.

## Testing

### DO test behavior and replay separately

For every operation, cover:

- valid state produces the expected success event;
- each refused state returns the expected typed error and no uncommitted event;
- intentional idempotent no-op returns success without an event;
- applying each event yields the expected state;
- replaying the event sequence reconstructs the same state;
- unique and reference entries reflect the reconstructed state.

For a meaningful rejection action, test that the reject command emits its `Rejected` fact and replay
changes status. For an operation failure, assert that no rejection event exists.
