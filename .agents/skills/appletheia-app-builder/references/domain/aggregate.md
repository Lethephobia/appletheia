# Aggregate Guidelines

Use for aggregate boundaries, command methods, state transitions, and event application.

## Aggregate

### DO define command methods on the aggregate and call `append_event` from them

Keep write-side behavior inside the aggregate boundary.

good:
```rust
pub fn rename(&mut self, name: AccountName) -> Result<AccountRenameResult, AccountError> {
    if self.state_required()?.status.is_closed() {
        let reason = AccountRenameRejectionReason::Closed;
        self.append_event(AccountEventPayload::RenameRejected {
            name: name.clone(),
            reason,
        })?;
        return Ok(AccountRenameResult::Rejected { reason });
    }

    self.append_event(AccountEventPayload::Renamed { name })?;
    Ok(AccountRenameResult::Renamed)
}
```

bad:
```rust
aggregate.append_event(ExampleEventPayload::Renamed { name });
```

### DO build the event payload inside the command method

Construct the payload from the validated command input before you append it.

good:
```rust
pub fn issue(
    &mut self,
    organization_id: OrganizationId,
    invitee_id: UserId,
    issuer: OrganizationInvitationIssuer,
    expires_at: OrganizationInvitationExpiresAt,
) -> Result<(), OrganizationInvitationError> {
    if self.state().is_some() {
        let reason = OrganizationInvitationIssueRejectionReason::AlreadyIssued;
        self.append_event(OrganizationInvitationEventPayload::IssueRejected {
            id: OrganizationInvitationId::new(),
            organization_id,
            invitee_id,
            issuer,
            expires_at,
            reason,
        })?;
        return Ok(());
    }

    self.append_event(OrganizationInvitationEventPayload::Issued {
        id: OrganizationInvitationId::new(),
        organization_id,
        invitee_id,
        issuer,
        expires_at,
    })
}
```

bad:
```rust
pub fn open(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
    self.append_event(event)
}
```

### PREFER one command method to append one event

Keep a command method focused on a single domain fact. If a lifecycle event already contains the data needed for a relationship subject, prefer to carry that data in the primary event payload instead of emitting a second relationship-specific event.

good:
```rust
pub fn register(
    &mut self,
    username: Username,
) -> Result<RegisterUserResult, UserError> {
    if self.state().is_some() {
        return Err(UserError::AlreadyRegistered);
    }

    let id = UserId::new();
    self.append_event(UserEventPayload::Registered {
        id,
        username,
    })?;
    Ok(RegisterUserResult::Registered { user_id: id })
}
```

bad:
```rust
pub fn register(
    &mut self,
    username: Username,
) -> Result<(), UserError> {
    self.append_event(UserEventPayload::Registered {
        id: UserId::new(),
        username,
    })?;
    self.append_event(ExampleEventPayload::SomethingElse { owner: UserId::new() })
}
```

### DO model expected domain rejections as events and command results

If a command can be refused as a normal business outcome, append a rejection or failure event and
return a result value from the aggregate command method. Do not return `Err` for outcomes that
sagas, projections, or users should observe as persisted facts.

good:
```rust
pub enum ReserveFundsResult {
    Reserved,
    Rejected { reason: ReserveFundsRejectionReason },
}

pub fn reserve_funds(&mut self, amount: Money) -> Result<ReserveFundsResult, AccountError> {
    match self.state_required()?.status {
        AccountStatus::Active => {}
        AccountStatus::Frozen => {
            let reason = ReserveFundsRejectionReason::Frozen;
            self.append_event(AccountEventPayload::FundsReservationRejected { amount, reason })?;
            return Ok(ReserveFundsResult::Rejected { reason });
        }
        AccountStatus::Closed => {
            let reason = ReserveFundsRejectionReason::Closed;
            self.append_event(AccountEventPayload::FundsReservationRejected { amount, reason })?;
            return Ok(ReserveFundsResult::Rejected { reason });
        }
    }

    if self.available_balance()?.is_less_than(amount) {
        let reason = ReserveFundsRejectionReason::InsufficientAvailableBalance;
        self.append_event(AccountEventPayload::FundsReservationRejected { amount, reason })?;
        return Ok(ReserveFundsResult::Rejected { reason });
    }

    self.append_event(AccountEventPayload::FundsReserved { amount })?;
    Ok(ReserveFundsResult::Reserved)
}
```

bad:
```rust
pub fn reserve_funds(&mut self, amount: Money) -> Result<(), AccountError> {
    if self.state_required()?.status.is_closed() {
        return Err(AccountError::Closed);
    }

    if self.available_balance()?.is_less_than(amount) {
        return Err(AccountError::InsufficientAvailableBalance);
    }

    self.append_event(AccountEventPayload::FundsReserved { amount })
}
```

### DO reject duplicate child additions in command methods, not apply methods

When an aggregate owns a child collection and a command would add a child that is already present,
detect the duplicate in the command method, append a rejection event, and return a rejected command
result. Keep `apply` as a direct state transition for the event fact. Do not hide duplicates in
`apply` with no-op checks such as `if !items.contains(...) { push(...) }`.

good:
```rust
pub fn link_identity(
    &mut self,
    identity: UserIdentity,
) -> Result<UserIdentityLinkResult, UserError> {
    if self.state_required()?.identities.contains(&identity) {
        let reason = UserIdentityLinkRejectionReason::AlreadyLinked;
        self.append_event(UserEventPayload::IdentityLinkRejected { identity, reason })?;
        return Ok(UserIdentityLinkResult::Rejected { reason });
    }

    self.append_event(UserEventPayload::IdentityLinked { identity })?;
    Ok(UserIdentityLinkResult::Linked)
}

fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
    match payload {
        UserEventPayload::IdentityLinked { identity } => {
            self.state_required_mut()?.identities.push(identity.clone());
        }
        UserEventPayload::IdentityLinkRejected { .. } => {}
    }

    Ok(())
}
```

bad:
```rust
fn apply(&mut self, payload: &UserEventPayload) -> Result<(), UserError> {
    match payload {
        UserEventPayload::IdentityLinked { identity } => {
            let state = self.state_required_mut()?;
            if !state.identities.contains(identity) {
                state.identities.push(identity.clone());
            }
        }
        UserEventPayload::IdentityLinkRejected { .. } => {}
    }

    Ok(())
}
```

### DO reserve aggregate errors for invalid or incomplete processing

Return `Err` when the command cannot be processed reliably or an invariant would be violated.
Use a rejection event for expected business denials such as insufficient funds, expired offers,
capacity limits, or already-consumed resources when those outcomes must drive projections or sagas.
`Err` is still appropriate for unexpected processing failures, missing required aggregate state,
serialization or conversion failures, arithmetic overflow, and internal invariant violations that
should roll back the command instead of being persisted as a business fact.

good:
```rust
pub fn available_balance(&self) -> Result<Money, AccountError> {
    let state = self.state_required()?;

    state.balance.try_sub(state.reserved_balance).map_err(|error| match error {
        MoneyError::InsufficientBalance => AccountError::InvalidReservedBalance,
        MoneyError::Overflow => AccountError::BalanceOverflow,
    })
}
```

good:
```rust
pub fn close(&mut self) -> Result<CloseAccountResult, AccountError> {
    if !self.state_required()?.balance.is_zero() {
        let reason = CloseAccountRejectionReason::BalanceRemaining;
        self.append_event(AccountEventPayload::CloseRejected { reason })?;
        return Ok(CloseAccountResult::Rejected { reason });
    }

    self.append_event(AccountEventPayload::Closed)?;
    Ok(CloseAccountResult::Closed)
}
```

bad:
```rust
pub fn reserve_funds(&mut self, amount: Money) -> Result<ReserveFundsResult, AccountError> {
    if !self.state_required()?.balance.is_zero() {
        return Err(AccountError::InsufficientAvailableBalance);
    }

    self.append_event(AccountEventPayload::FundsReserved { amount })?;
    Ok(ReserveFundsResult::Reserved)
}
```

### PREFER command methods and events to align with top-level value object boundaries

If an aggregate state owns a top-level value object, prefer changing that value object through one aggregate command method and one event for the whole value object. Avoid adding attribute-specific command methods and events for fields nested inside that value object unless those fields have meaning outside the value object boundary.

good:
```rust
pub fn change_profile(
    &mut self,
    profile: OrganizationProfile,
) -> Result<OrganizationChangeProfileResult, OrganizationError> {
    if self.state_required()?.status.is_removed() {
        let reason = OrganizationChangeProfileRejectionReason::Removed;
        self.append_event(OrganizationEventPayload::ProfileChangeRejected {
            profile: profile.clone(),
            reason,
        })?;
        return Ok(OrganizationChangeProfileResult::Rejected { reason });
    }

    self.append_event(OrganizationEventPayload::ProfileChanged { profile })?;
    Ok(OrganizationChangeProfileResult::Changed)
}
```

bad:
```rust
pub fn change_display_name(
    &mut self,
    display_name: OrganizationDisplayName,
) -> Result<(), OrganizationError> {
    self.append_event(OrganizationEventPayload::DisplayNameChanged { display_name })
}
```

### DO allow attribute-level command methods and events when changing an entity inside the aggregate

If the aggregate owns an entity and the change targets one attribute of that entity, attribute-level command methods and events are acceptable. In that case the domain fact is usually about that specific attribute on that specific entity, not about replacing an enclosing value object.

good:
```rust
pub fn change_identity_email(
    &mut self,
    provider: UserIdentityProvider,
    subject: UserIdentitySubject,
    email: Option<Email>,
) -> Result<(), UserError> {
    let identity = self
        .state_required()?
        .identities
        .iter()
        .find(|identity| identity.matches(&provider, &subject))
        .ok_or(UserError::IdentityNotFound)?;

    if identity.email() == email.as_ref() {
        return Ok(());
    }

    self.append_event(UserEventPayload::IdentityEmailChanged {
        provider,
        subject,
        email,
    })
}
```

bad:
```rust
pub fn change_identity(
    &mut self,
    provider: UserIdentityProvider,
    subject: UserIdentitySubject,
    identity: UserIdentity,
) -> Result<(), UserError> {
    self.append_event(UserEventPayload::IdentityChanged {
        provider,
        subject,
        identity,
    })
}
```

### PREFER collection value objects when the aggregate treats the whole collection as one declared value

If a collection is supplied, stored, and replaced as one declared value, model it as a dedicated type instead of exposing the raw collection directly. This usually fits configuration-like inputs and top-level aggregate values that are changed in one step.

good:
```rust
pub fn configure_audiences(
    &mut self,
    audiences: AuthTokenAudiences,
) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::AudiencesConfigured { audiences })
}
```

bad:
```rust
pub fn add_audience(
    &mut self,
    audience: AuthTokenAudience,
) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::AudienceAdded { audience })
}
```

### PREFER raw collections when add/remove operations are the domain facts

If commands and events add or remove single items, keep the state as a raw collection and choose the collection type that matches the semantics. Prefer `Vec` when order matters and `BTreeSet` or `HashSet` when uniqueness matters.

good:
```rust
pub fn grant_role(
    &mut self,
    role: OrganizationRole,
) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::RoleGranted { role })
}
```

good:
```rust
pub struct ExampleState {
    roles: Vec<OrganizationRole>,
}
```

bad:
```rust
pub struct ExampleState {
    roles: OrganizationRoles,
}

pub fn grant_role(
    &mut self,
    role: OrganizationRole,
) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::RoleGranted { role })
}
```

### DON'T model a collection as a value object when the commands and events mutate it item by item

Avoid wrapping a collection in a value object when the surrounding API still talks in terms of individual inserts and removals. That split usually makes the state shape and the event model drift apart.

bad:
```rust
pub fn grant_role(&mut self, roles: OrganizationRoles) -> Result<(), OrganizationError> {
    self.append_event(OrganizationEventPayload::RolesReplaced { roles })
}
```

good:
```rust
pub fn grant_role(&mut self, role: OrganizationRole) -> Result<(), OrganizationError> {
    self.append_event(OrganizationEventPayload::RoleGranted { role })
}
```

### DO validate the request before you append an event

Append a rejection event for expected business denials before any success event is recorded. Keep
unexpected processing errors as `Err`.

good:
```rust
pub fn reserve_funds(&mut self, amount: Money) -> Result<ReserveFundsResult, AccountError> {
    if self.available_balance()?.is_less_than(amount) {
        let reason = ReserveFundsRejectionReason::InsufficientAvailableBalance;
        self.append_event(AccountEventPayload::FundsReservationRejected { amount, reason })?;
        return Ok(ReserveFundsResult::Rejected { reason });
    }

    self.append_event(AccountEventPayload::FundsReserved { amount })?;
    Ok(ReserveFundsResult::Reserved)
}
```

bad:
```rust
pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Renamed { name })
}
```

### DO append success events even when the resulting state is unchanged

If a command is accepted, append the corresponding success event and return the success result even
when replaying that event leaves the aggregate state unchanged. This keeps command acceptance
observable for sagas and projections, and prevents workflows from stalling while they wait for an
accepted command event that was never persisted. Use command idempotency to suppress duplicate
command messages; do not hide accepted commands inside aggregate no-ops.

good:
```rust
pub fn change_name(&mut self, name: AccountName) -> Result<AccountNameChangeResult, AccountError> {
    if self.state_required()?.status.is_closed() {
        let reason = AccountNameChangeRejectionReason::Closed;
        self.append_event(AccountEventPayload::NameChangeRejected {
            name: name.clone(),
            reason,
        })?;
        return Ok(AccountNameChangeResult::Rejected { reason });
    }

    self.append_event(AccountEventPayload::NameChanged { name })?;
    Ok(AccountNameChangeResult::Changed)
}
```

bad:
```rust
if self.state().is_some_and(|state| state.name == name) {
    return Ok(AccountNameChangeResult::Changed);
}
```

### DO run business rejection checks before success events

Do not let an already-matching state hide a business rejection that should be persisted. Validate
the command first, then append the success event.

good:
```rust
pub fn change_name(&mut self, name: AccountName) -> Result<AccountNameChangeResult, AccountError> {
    if self.state_required()?.status.is_closed() {
        let reason = AccountNameChangeRejectionReason::Closed;
        self.append_event(AccountEventPayload::NameChangeRejected {
            name: name.clone(),
            reason,
        })?;
        return Ok(AccountNameChangeResult::Rejected { reason });
    }

    self.append_event(AccountEventPayload::NameChanged { name })?;
    Ok(AccountNameChangeResult::Changed)
}
```

bad:
```rust
if self.state().is_some_and(|state| state.name == name) {
    return Ok(AccountNameChangeResult::Changed);
}

if self.state_required()?.status.is_closed() {
    let reason = AccountNameChangeRejectionReason::Closed;
    self.append_event(AccountEventPayload::NameChangeRejected { name, reason })?;
    return Ok(AccountNameChangeResult::Rejected { reason });
}
```

### DO reject repeated one-shot methods when repetition is a business outcome

When repetition is observable domain behavior, append a rejection event instead of silently treating
it as a success event. Use `Err` only when the repeated call is command misuse that should roll back and not
be projected.

good:
```rust
pub fn create(
    &mut self,
    handle: OrganizationHandle,
    name: OrganizationName,
) -> Result<OrganizationCreateResult, OrganizationError> {
    if self.state().is_some() {
        let reason = OrganizationCreateRejectionReason::AlreadyCreated;
        self.append_event(OrganizationEventPayload::CreateRejected { handle, name, reason })?;
        return Ok(OrganizationCreateResult::Rejected { reason });
    }

    let id = OrganizationId::new();
    self.append_event(OrganizationEventPayload::Created {
        id,
        handle,
        name,
    })?;
    Ok(OrganizationCreateResult::Created { organization_id: id })
}
```

bad:
```rust
pub fn open(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    if self.state().is_some_and(|state| state.name == name) {
        return Ok(());
    }

    self.append_event(ExampleEventPayload::Opened {
        id: ExampleId::new(),
        name,
    })
}
```

### DO generate the aggregate's own `AggregateId` inside the aggregate's own command method

Keep identity creation within the aggregate boundary.

good:
```rust
pub fn create(
    &mut self,
    organization_id: OrganizationId,
    user_id: UserId,
) -> Result<(), OrganizationMembershipError> {
    self.append_event(OrganizationMembershipEventPayload::Created {
        id: OrganizationMembershipId::new(),
        organization_id,
        user_id,
    })
}
```

bad:
```rust
pub fn open(&mut self, id: ExampleId, name: ExampleName) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Opened { id, name })
}
```

### PREFER expose state attributes and computed values through getters

Use read-only accessors when callers need the current state or a derived value.

good:
```rust
pub fn available_balance(&self) -> Result<AccountBalance, AccountError> {
    let state = self.state_required()?;

    state
        .balance
        .try_sub(state.reserved_balance)
        .map_err(|error| match error {
            AccountBalanceError::InsufficientBalance => AccountError::InvalidReservedBalance,
            AccountBalanceError::BalanceOverflow => AccountError::BalanceOverflow,
        })
}
```

bad:
```rust
let name = aggregate.state.name.clone();
```

### DON'T give the aggregate any fields other than `core`

Keep aggregate data inside `AggregateCore` and the aggregate state.

bad:
```rust
pub struct ExampleAggregate {
    core: AggregateCore<ExampleState, ExampleEventPayload>,
    name: ExampleName,
}
```

good:
```rust
pub struct Organization {
    core: AggregateCore<OrganizationState, OrganizationEventPayload>,
}
```

### DON'T define trigger-only command methods or events on the aggregate that creates another aggregate

Put the creation command on the aggregate that is actually being created.

bad:
```rust
impl Parent {
    pub fn request_open_child(&mut self, name: ChildName) -> Result<(), ParentError> {
        self.append_event(ParentEventPayload::ChildOpenRequested { name })
    }
}

impl Child {
    pub fn open(&mut self, name: ChildName) -> Result<(), ChildError> {
        self.append_event(ChildEventPayload::Opened {
            id: ChildId::new(),
            name,
        })
    }
}
```

good:
```rust
impl OrganizationMembership {
    pub fn create(
        &mut self,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), OrganizationMembershipError> {
        self.append_event(OrganizationMembershipEventPayload::Created {
            id: OrganizationMembershipId::new(),
            organization_id,
            user_id,
        })
    }
}
```

### DON'T reference other aggregates directly

Pass aggregate identifiers instead of aggregate instances.

bad:
```rust
pub fn transfer_to(
    &mut self,
    target: ExampleAggregate,
    amount: Money,
) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Transferred {
        target_id: target.id(),
        amount,
    })
}
```

good:
```rust
pub fn issue(
    &mut self,
    organization_id: OrganizationId,
    invitee_id: UserId,
    issuer: OrganizationInvitationIssuer,
    expires_at: OrganizationInvitationExpiresAt,
) -> Result<(), OrganizationInvitationError> {
    self.append_event(OrganizationInvitationEventPayload::Issued {
        id: OrganizationInvitationId::new(),
        organization_id,
        invitee_id,
        issuer,
        expires_at,
    })
}
```

## AggregateApply

### DON'T put validation in `apply`

Keep validation in command methods, not in event replay.

bad:
```rust
fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
    if self.state_required()?.status.is_removed() {
        return Err(OrganizationError::Removed);
    }

    match payload {
        OrganizationEventPayload::NameChanged { name } => {
            self.state_required_mut()?.name = name.clone();
            Ok(())
        }
        OrganizationEventPayload::Created {
            id,
            handle,
            name,
        } => {
            self.state = Some(OrganizationState {
                id: *id,
                handle: handle.clone(),
                name: name.clone(),
                status: OrganizationStatus::Active,
            });
            Ok(())
        }
        _ => Ok(()),
    }
}
```

good:
```rust
fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
    match payload {
        OrganizationEventPayload::Created {
            id,
            handle,
            name,
        } => {
            self.state = Some(OrganizationState {
                id: *id,
                handle: handle.clone(),
                name: name.clone(),
                status: OrganizationStatus::Active,
            });
            Ok(())
        }
        OrganizationEventPayload::HandleChanged { handle } => {
            self.state_required_mut()?.handle = handle.clone();
            Ok(())
        }
        OrganizationEventPayload::NameChanged { name } => {
            self.state_required_mut()?.name = name.clone();
            Ok(())
        }
        OrganizationEventPayload::Removed => {
            self.state_required_mut()?.status = OrganizationStatus::Removed;
            Ok(())
        }
    }
}
```

### DON'T skip events that `apply` receives

If an event cannot be applied, return an error so compatibility problems stay visible.

bad:
```rust
fn apply(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
    match event {
        ExampleEventPayload::Renamed { name } => {
            if let Some(state) = self.state_mut() {
                state.name = name;
            }

            Ok(())
        }
        ExampleEventPayload::Opened { id, name } => {
            self.state = Some(ExampleState {
                id: *id,
                name: name.clone(),
                status: ExampleStatus::Active,
            });
            Ok(())
        }
    }
}
```

good:
```rust
fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
    match payload {
        OrganizationEventPayload::NameChanged { name } => {
            let state = self.state_required_mut()?;
            state.name = name.clone();
            Ok(())
        }
        OrganizationEventPayload::Created {
            id,
            handle,
            name,
        } => {
            self.state = Some(OrganizationState {
                id: *id,
                handle: handle.clone(),
                name: name.clone(),
                status: OrganizationStatus::Active,
            });
            Ok(())
        }
        OrganizationEventPayload::HandleChanged { handle } => {
            self.state_required_mut()?.handle = handle.clone();
            Ok(())
        }
        OrganizationEventPayload::Removed => {
            self.state_required_mut()?.status = OrganizationStatus::Removed;
            Ok(())
        }
    }
}
```

## AggregateState

### PREFER keep `AggregateState` fields `pub(super)` or `pub(crate)` at most

Limit field visibility to the aggregate module or its parent when possible.

good:
```rust
pub(super) struct OrganizationState {
    pub(super) id: OrganizationId,
    pub(super) status: OrganizationStatus,
    pub(super) handle: OrganizationHandle,
    pub(super) name: OrganizationName,
}
```

bad:
```rust
pub struct OrganizationState {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub name: OrganizationName,
    pub status: OrganizationStatus,
}
```

### DON'T use initial events as state snapshots

Keep initial event payloads focused on the fact that happened and the values decided by that fact.
Do not add fields only because the aggregate state needs a default value. When a value is fully
implied by the event variant, initialize it in `apply`.

good:
```rust
pub fn create(
    &mut self,
    handle: OrganizationHandle,
    name: OrganizationName,
) -> Result<(), OrganizationError> {
    self.append_event(OrganizationEventPayload::Created {
        id: OrganizationId::new(),
        handle,
        name,
    })
}

fn apply(&mut self, payload: &OrganizationEventPayload) -> Result<(), OrganizationError> {
    match payload {
        OrganizationEventPayload::Created { id, handle, name } => {
            self.state = Some(OrganizationState {
                id: *id,
                handle: handle.clone(),
                name: name.clone(),
                status: OrganizationStatus::Active,
            });
        }
        OrganizationEventPayload::Removed => {
            self.state_required_mut()?.status = OrganizationStatus::Removed;
        }
    }

    Ok(())
}
```

bad:
```rust
pub enum OrganizationEventPayload {
    Created {
        id: OrganizationId,
        handle: OrganizationHandle,
        name: OrganizationName,
        status: OrganizationStatus,
        member_count: u32,
    },
}
```

### AVOID define methods on `AggregateState`, and keep logic out of it

Keep `AggregateState` as a data container and update it directly from `AggregateApply`.

bad:
```rust
impl OrganizationState {
    pub fn change_name(&mut self, name: OrganizationName) {
        self.name = name;
    }
}
```

good:
```rust
impl AggregateApply<OrganizationEventPayload, OrganizationError> for Organization {
    fn apply(
        &mut self,
        payload: &OrganizationEventPayload,
    ) -> Result<(), OrganizationError> {
        if let OrganizationEventPayload::NameChanged { name } = payload {
            self.state_required_mut()?.name = name.clone();
        }

        Ok(())
    }
}
```

### PREFER define value objects

Prefer domain-specific types over primitive fields when the meaning matters.

good:
```rust
pub(super) struct OrganizationState {
    pub(super) handle: OrganizationHandle,
    pub(super) name: OrganizationName,
    pub(super) status: OrganizationStatus,
}
```

bad:
```rust
pub(super) struct OrganizationState {
    pub(super) handle: String,
    pub(super) name: String,
    pub(super) status: i64,
}
```

### PREFER serialize enum value objects as adjacently tagged JSON

When a value object is an enum and it is serialized to JSON, prefer `#[serde(tag = "type", content = "data", rename_all = "snake_case")]` so the wire shape stays explicit and compatible with future tuple variants.

good:
```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationStatus {
    Active,
    Removed,
}
```

bad:
```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationStatus {
    Active,
    Removed,
}
```

### AVOID use floating-point types such as `f64` in `AggregateState`

Use a fixed-point representation and keep the decimal precision explicit.

bad:
```rust
pub(super) struct AccountState {
    pub(super) balance: f64,
}
```

good:
```rust
pub(super) struct AccountState {
    pub(super) balance: AccountBalance,
    pub(super) reserved_balance: AccountBalance,
}
```

## EventPayload

### PREFER model `EventPayload` as an enum

Use enum variants to represent distinct facts instead of collapsing them into a struct with optional fields.

good:
```rust
#[event_payload(error = OrganizationEventPayloadError)]
pub enum OrganizationEventPayload {
    Created {
        id: OrganizationId,
        handle: OrganizationHandle,
        name: OrganizationName,
    },
    HandleChanged {
        handle: OrganizationHandle,
    },
    NameChanged {
        name: OrganizationName,
    },
    Removed,
}
```

bad:
```rust
#[derive(Serialize, Deserialize)]
pub struct OrganizationEventPayload {
    pub kind: String,
    pub id: Option<OrganizationId>,
    pub handle: Option<OrganizationHandle>,
    pub name: Option<OrganizationName>,
}
```

### DO use past participles for variant names

Make variants read as facts about what already happened.

good:
```rust
pub enum OrganizationInvitationEventPayload {
    Issued {
        id: OrganizationInvitationId,
        organization_id: OrganizationId,
        invitee_id: UserId,
        issuer: OrganizationInvitationIssuer,
        expires_at: OrganizationInvitationExpiresAt,
    },
    Accepted {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
    Declined {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
    Canceled {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
}
```

bad:
```rust
pub enum OrganizationInvitationEventPayload {
    Issue {
        id: OrganizationInvitationId,
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
    Accept {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
    Decline {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
    Cancel {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
}
```
