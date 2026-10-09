# Aggregate Design

Use this reference for aggregate behavior and event replay. Examples use Banking types and omit
unrelated declarations.

### DO enforce aggregate invariants in aggregate methods

Keep rules that depend on the aggregate's own state inside its methods. Command handlers call those
methods instead of reproducing their checks or appending domain events directly. This keeps every
caller subject to the same invariants. Build the event payload from validated domain arguments inside
the method; accepting an arbitrary `EventPayload` would let the caller choose the transition.
Reference other aggregate roots by ID rather than passing or storing their instances.

**Good**

```rust
impl Account {
    pub fn reserve_funds(&mut self, amount: CurrencyAmount) -> Result<(), AccountError> {
        match self.state_required()?.status {
            AccountStatus::Active => {}
            AccountStatus::Frozen => {
                return Err(AccountError::Frozen);
            }
            AccountStatus::Closed => {
                return Err(AccountError::Closed);
            }
        }

        if self.available_balance()? < amount {
            return Err(AccountError::InsufficientAvailableBalance);
        }

        self.append_event(AccountEventPayload::FundsReserved { amount })
    }
}
```

**Bad**

```rust
// In the handler: bypass aggregate behavior.
if account.available_balance()? >= command.amount {
    account.append_event(AccountEventPayload::FundsReserved { amount: command.amount })?;
}
```

### DO return operation failures as errors

Return `Err` when the requested operation fails, including an expected domain refusal. Do not return
failure as a successful result or append a domain event solely to notify callers of that failure.
This lets command execution handle rollback, retryability, and terminal-failure notification.
The error may contain a dedicated reason type when that helps express the failure.

Distinguish operation failure from a successful business decision to reject or decline something.
Rejecting a pending join request changes its domain state and can legitimately emit `Rejected`.
Decide from the meaning of the operation, not the event's name.

**Good**

```rust
// The requested funds reservation did not happen.
return Err(AccountError::InsufficientAvailableBalance);
```

**Bad**

```rust
// Report a failed reservation as a successful result.
return Ok(AccountFundsReserveResult::Rejected { reason });
```

```rust
// Or append an event solely to notify callers of the failed reservation.
self.append_event(AccountEventPayload::FundsReserveRejected { reason })?;
Ok(())
```

### DO validate before appending a success event

Validate whether the operation is allowed and whether it preserves business invariants in the
aggregate method, before calling `append_event`, including duplicate-child checks. Do not defer these checks to `AggregateApply::apply`
or repeat them there. Returning an error afterward does not automatically undo changes to the
in-memory aggregate or remove events already added to its pending change set.

**Good**

```rust
pub fn change_name(&mut self, name: AccountName) -> Result<(), AccountError> {
    let state = self.state_required()?;
    if state.status.is_closed() {
        return Err(AccountError::Closed);
    }
    self.append_event(AccountEventPayload::NameChanged { name })
}
```

**Bad**

```rust
// The command method appends without checking whether the operation is allowed.
self.append_event(AccountEventPayload::NameChanged { name })

// The check is incorrectly deferred to AggregateApply::apply.
AccountEventPayload::NameChanged { name } => {
    if self.state_required()?.status.is_closed() {
        return Err(AccountError::Closed);
    }
    self.state_required_mut()?.name = name.clone();
}
```

### DO append an event even when the requested value is unchanged

For an accepted aggregate operation, append its event even when the requested value equals the
current value. Do not return `Ok(())` without an event merely because the values match. A saga may
be waiting for that command's resulting event; silently treating the operation as a no-op can leave
the workflow waiting indefinitely.

This applies after validating the operation. It does not turn a refused operation into success or
replace the framework's command deduplication.

**Good**

```rust
pub fn change_name(&mut self, name: AccountName) -> Result<(), AccountError> {
    if self.state_required()?.status.is_closed() {
        return Err(AccountError::Closed);
    }

    self.append_event(AccountEventPayload::NameChanged { name })
}
```

**Bad**

```rust
pub fn change_name(&mut self, name: AccountName) -> Result<(), AccountError> {
    let state = self.state_required()?;
    if state.status.is_closed() {
        return Err(AccountError::Closed);
    }
    if state.name == name {
        return Ok(());
    }

    self.append_event(AccountEventPayload::NameChanged { name })
}
```

### DO make event application the source of state changes

Command methods validate and call `append_event`; `AggregateApply::apply` reflects the already
validated event in state. It does not decide whether the operation should be accepted or revalidate
business invariants. Structural state-access errors, such as missing initialized state, can still
be propagated. It must reproduce the same state during live execution and replay. Avoid reading the clock,
performing I/O, or appending further events in `apply`; capture required values in the original event.
Match variants explicitly so a new event requires a deliberate replay decision.

Use `state_required()` for behavior that requires initialization and `state_required_mut()` when
applying changes to initialized state, rather than unwrapping optional state.

Keep aggregate State as a data structure: do not add business logic, state-transition methods, or
setters to it. Validate operations in aggregate methods and assign State fields directly inside
`AggregateApply::apply`, rather than delegating changes to methods on State. Prefer `pub(super)`
fields (or `pub(crate)` when needed) and read-only aggregate accessors for external callers. Direct
assignment in `apply` does not mean exposing publicly mutable State fields.

Do not silently skip an event because required state or a child is missing. Propagate a structural
error instead. Likewise, do not hide duplicate-child events with a conditional insert in `apply`;
validate duplicate additions in the aggregate method.

**Good**

```rust
// Command method:
self.append_event(AccountEventPayload::NameChanged { name })

// Corresponding arm in AggregateApply::apply:
AccountEventPayload::NameChanged { name } => {
    self.state_required_mut()?.name = name.clone();
}
```

**Bad**

```rust
// Command method changes state without recording how to replay it.
self.state_required_mut()?.name = name;
Ok(())
```

```rust
// Delegate the transition to a setter on State instead of assigning the field in apply.
AccountEventPayload::NameChanged { name } => {
    self.state_required_mut()?.set_name(name.clone());
}
```

```text
Apply an event -> fetch today's external profile -> reconstruct different state on each replay
```

### DO model child entities as identity-bearing state

Keep child entities as structs containing their identity and attributes, without constructors,
getters, mutation methods, or business validation. Their identity may be a composite of value
objects, such as a user identity's provider and subject. Validate operations in the aggregate
before appending events; construct entities with struct literals and assign their fields directly
in `apply`. Value objects still enforce their own value constraints.

Expose only shared references to entities held by an aggregate. Public entity fields must not
allow callers to mutate the aggregate's stored state outside event application.

**Good**

```rust
pub struct UserIdentity {
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}

// In User::apply, after the operation has been validated before event append:
UserEventPayload::IdentityLinked { provider, subject, email } => {
    self.state_required_mut()?.identities.push(UserIdentity {
        provider: provider.clone(),
        subject: subject.clone(),
        email: email.clone(),
    });
}

// Aggregate accessors return &UserIdentity or &[UserIdentity], never mutable references.
```

**Bad**

```rust
// Entity methods obscure state construction and updates during event application.
let identity = UserIdentity::new(provider, subject, email)?;
identity.set_email(new_email)?;

// Exposing mutable aggregate-owned entities permits unrecorded changes.
pub fn identities_mut(&mut self) -> &mut Vec<UserIdentity> {
    &mut self.state.identities
}
```

### DO keep initial events focused on facts rather than copying State

Include values decided by the creation operation. Initialize values implied by the event variant
explicitly in `apply`; do not add status, empty collections, or other defaults merely to serialize a
complete State. Keep this initialization visible instead of hiding it in a State constructor.
A value chosen by the caller still belongs in the event even if it happens to equal a default.

**Good**

```rust
// Registered carries no data because these initial values are implied by registration.
UserEventPayload::Registered => self.set_state(Some(UserState {
    identities: Vec::new(),
    username: None,
    display_name: None,
    bio: None,
    picture: None,
    status: UserStatus::Active,
})),
```

**Bad**

```rust
Registered {
    identities: Vec<UserIdentity>,
    username: Option<Username>,
    display_name: Option<UserDisplayName>,
    bio: Option<UserBio>,
    picture: Option<UserPictureRef>,
    status: UserStatus,
}, // All these values are already fixed by the meaning of Registered.
```

### DO obtain the root's own ID from the aggregate

`Aggregate::new()` generates the ID before the first event; read it through `aggregate_id()`.
Do not generate another ID in a creation method or duplicate the root's own ID in State or its
EventPayload. The event metadata carries it, and a typed saga event exposes `event.aggregate_id()`.
Handlers can return that ID in their output. IDs of other roots remain normal domain data.

**Good**

```rust
let mut user = User::new();
let user_id = user.aggregate_id();
user.register()?;
// After saving, the handler may include user_id in its output.
```

**Bad**

```rust
pub fn register(&mut self) -> Result<(), UserError> {
    let user_id = UserId::new(); // Competes with the ID already held by AggregateCore.
    self.append_event(UserEventPayload::Registered { user_id })
}
```

### DO derive unique and reference entries from aggregate state

Define unique and reference entries from the state produced by replay, using the existing state
macros or contracts. Repository persistence maintains these indexes alongside aggregate changes.
Do not introduce a second mutable source of truth for an indexed value. When persistence code needs
the entries, call `aggregate.unique_entries()` and `aggregate.reference_entries()`; the aggregate
supplies its ID to State's callbacks. If a callback needs the root ID, use its `aggregate_id` argument
rather than adding the ID to State.

For example, Banking's organization handle is declared as a unique field on `OrganizationState`.
Cross-aggregate lookups belong in the application layer; see [Command Design](../application/command.md).

**Good**

```text
Organization events -> current OrganizationState.handle -> unique entry maintained on save
```

```rust
let unique_entries = aggregate.unique_entries()?;
let reference_entries = aggregate.reference_entries()?;
```

**Bad**

```text
Rename organization -> update only a separate handle index -> replay still yields the old handle
```

```rust
// Persistence code bypasses the aggregate's ID/state boundary.
let entries = aggregate.state_required()?.unique_entries(aggregate.aggregate_id().value())?;
```

### DO create child entities through separate events

Use a separate event to create a child entity, including when it is created alongside the root.
This gives initial and later child creation the same replay path. When the use case requires both,
the command handler calls both aggregate operations and saves them in one unit of work.

Banking's `Registered` creates the user with no identities; `IdentityLinked` creates a child.
This rule concerns entity lifecycles, not whether event fields use a reusable DTO or value object.

**Good**

```rust
pub enum UserEventPayload {
    Registered,
    IdentityLinked {
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    },
    // Other variants omitted.
}
```

**Bad**

```rust
pub enum UserEventPayload {
    Registered { identities: Vec<UserIdentityData> },
    IdentityLinked { identity: UserIdentityData },
    // Initial and later entity creation now need separate replay paths.
}
```

### PREFER updates that respect value-object and entity boundaries

Replace a value object as a whole when its fields form one domain value. Avoid commands and events
for its internal fields merely because they are independently stored. An entity has its own identity
and lifecycle, so an event may update one of its attributes while identifying that entity.

Banking changes a `UserPictureRef` as one value and an identity's email by provider/subject. Child
identifiers must remain meaningful if the collection order changes; do not use a vector position as
identity.

A collection wrapper does not require whole-collection replacement. When elements represent
independent grants, expose aggregate operations such as `grant_role` and `revoke_role`, and record
`RoleGranted` / `RoleRevoked`. A plain `BTreeSet<OrganizationRole>` can store the roles: validate in the aggregate before
appending the event, then insert or remove the element directly in `apply`.
When a membership creation command accepts initial roles, create the membership with an empty
set and call `grant_role` for each requested role in the command handler before saving once.
Do not fold the grants into the aggregate's creation method or its `Created` event.

**Good**

```rust
// Event variants (excerpt):
PictureSet {
    picture: Option<UserPictureRef>,
    old_picture: Option<UserPictureRef>,
},
IdentityEmailSet {
    provider: UserIdentityProvider,
    subject: UserIdentitySubject,
    email: Option<Email>,
},
```

```rust
membership.grant_role(OrganizationRole::Admin)?;
membership.revoke_role(OrganizationRole::Treasurer)?;
```

**Bad**

```rust
// Replacing every role obscures an operation intended only to grant one permission.
membership.change_roles(roles_copied_from_an_old_screen)?;

// A position does not identify the same child after reordering.
IdentityEmailSet { index: usize, email: Option<Email> },
```

```text
Change one field inside a picture reference -> leave the remaining reference fields inconsistent
```

### PREFER adjacent tags for enum value objects serialized as JSON

Follow Banking's `#[serde(tag = "type", content = "data", rename_all = "snake_case")]` convention for
domain enum VOs so discriminants and variant data have an explicit, consistent shape. This is a
wire-format convention. The `#[saga_step]` macro also uses adjacent tags by default; a deliberately
customized serialization contract can use different settings.

**Good**

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Removed,
}
```

**Bad**

```rust
// Introduces a different JSON shape for an equivalent domain status.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Removed,
}
```

### DO represent monetary quantities with explicit precision

Use exact numeric VOs with checked arithmetic for balances and quantities. Banking's `CurrencyAmount`
stores `u128` smallest units, with decimal precision represented separately. Do not use floating-point
state for financial quantities that require exact replay and comparison.

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
    pub amount: f64,
}
```

### DO verify that emitted events reconstruct the resulting state

When testing aggregate behavior, compare replayed state with the state reached by the command
methods. A successful return alone does not detect state mutations that bypass events. For refused
operations, verify the concrete error and that no event was added by the failed operation.

**Good**

```text
Run register and link_identity -> collect events -> replay on a fresh root -> compare state
Refuse duplicate identity -> assert IdentityAlreadyLinked and unchanged pending-event count
```

**Bad**

```text
Run register and link_identity -> assert Ok only -> never verify replay
```
