# Aggregate Guidelines

Use for aggregate boundaries, command methods, state transitions, and event application.

## Aggregate

### DO define command methods on the aggregate and call `append_event` from them

Keep write-side behavior inside the aggregate boundary.

good:
```rust
pub fn change_name(&mut self, name: OrganizationName) -> Result<(), OrganizationError> {
    self.ensure_not_removed()?;

    let current_name = self.state_required()?.name.clone();
    if current_name.eq(&name) {
        return Ok(());
    }

    self.append_event(OrganizationEventPayload::NameChanged { name })
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
        return Err(OrganizationInvitationError::AlreadyIssued);
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
) -> Result<(), UserError> {
    if self.state().is_some() {
        return Err(UserError::AlreadyRegistered);
    }

    self.append_event(UserEventPayload::Registered {
        id: UserId::new(),
        username,
    })
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

### PREFER command methods and events to align with top-level value object boundaries

If an aggregate state owns a top-level value object, prefer changing that value object through one aggregate command method and one event for the whole value object. Avoid adding attribute-specific command methods and events for fields nested inside that value object unless those fields have meaning outside the value object boundary.

good:
```rust
pub fn change_profile(
    &mut self,
    profile: OrganizationProfile,
) -> Result<(), OrganizationError> {
    self.ensure_not_removed()?;

    if self.state_required()?.profile == profile {
        return Ok(());
    }

    self.append_event(OrganizationEventPayload::ProfileChanged { profile })
}
```

bad:
```rust
pub fn change_display_name(
    &mut self,
    display_name: OrganizationDisplayName,
) -> Result<(), OrganizationError> {
    self.ensure_not_removed()?;

    if self
        .state_required()?
        .profile
        .display_name()
        .eq(&display_name)
    {
        return Ok(());
    }

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
    self.ensure_active()?;

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

### DO validate the request before you append an event

Reject invalid requests before any state change is recorded.

good:
```rust
pub fn reserve_funds(&mut self, amount: AccountBalance) -> Result<(), AccountError> {
    self.ensure_active_status()?;
    self.ensure_available_balance_at_least(amount, AccountError::InsufficientAvailableBalance)?;

    if amount.is_zero() {
        return Ok(());
    }

    self.append_event(AccountEventPayload::FundsReserved { amount })
}
```

bad:
```rust
pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Renamed { name })
}
```

### DO treat repeated updates to the same state as no-ops when repeating them is harmless

Return `Ok(())` instead of an error for idempotent requests.

good:
```rust
pub fn rename(&mut self, name: AccountName) -> Result<(), AccountError> {
    self.ensure_not_closed()?;

    if self.state().is_some_and(|state| state.name.eq(&name)) {
        return Ok(());
    }

    self.append_event(AccountEventPayload::Renamed { name })
}
```

bad:
```rust
if self.state().is_some_and(|state| state.name == name) {
    return Err(ExampleError::AlreadyRenamed);
}
```

### DO run validation before the no-op check

Do not let a repeated state hide a real validation failure.

good:
```rust
pub fn change_handle(&mut self, handle: OrganizationHandle) -> Result<(), OrganizationError> {
    self.ensure_not_removed()?;

    if self.state().is_some_and(|state| state.handle.eq(&handle)) {
        return Ok(());
    }

    self.append_event(OrganizationEventPayload::HandleChanged { handle })
}
```

bad:
```rust
if self.state().is_some_and(|state| state.name == name) {
    return Ok(());
}

self.ensure_opened()?;
```

### DO return an error instead of a no-op for one-shot methods that should only succeed once

Treat repeated create, open, or close calls as misuse.

good:
```rust
pub fn create(
    &mut self,
    handle: OrganizationHandle,
    name: OrganizationName,
) -> Result<(), OrganizationError> {
    if self.state().is_some() {
        return Err(OrganizationError::AlreadyCreated);
    }

    self.append_event(OrganizationEventPayload::Created {
        id: OrganizationId::new(),
        handle,
        name,
    })
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

### PREFER move validations shared by several command methods into private helper methods

Keep public methods focused on intent and share repeated checks through private helpers.

good:
```rust
fn ensure_not_removed(&self) -> Result<(), OrganizationError> {
    if self.state_required()?.status.is_removed() {
        return Err(OrganizationError::Removed);
    }

    Ok(())
}

pub fn change_name(&mut self, name: OrganizationName) -> Result<(), OrganizationError> {
    self.ensure_not_removed()?;
    self.append_event(OrganizationEventPayload::NameChanged { name })
}
```

bad:
```rust
pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    if self.state().is_none() {
        return Err(ExampleError::NotOpened);
    }

    self.append_event(ExampleEventPayload::Renamed { name })
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
        OrganizationEventPayload::Created { id, handle, name } => {
            self.state = Some(OrganizationState::new(
                *id,
                handle.clone(),
                name.clone(),
            ));
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
        OrganizationEventPayload::Created { id, handle, name } => {
            self.state = Some(OrganizationState::new(
                *id,
                handle.clone(),
                name.clone(),
            ));
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
            self.state = Some(ExampleState::new(id, name));
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
        OrganizationEventPayload::Created { id, handle, name } => {
            self.state = Some(OrganizationState::new(
                *id,
                handle.clone(),
                name.clone(),
            ));
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

### PREFER provide a constructor when the state has a default value

Use a constructor to capture default state in one place.

good:
```rust
impl OrganizationState {
    pub(super) fn new(
        id: OrganizationId,
        handle: OrganizationHandle,
        name: OrganizationName,
    ) -> Self {
        Self {
            id,
            status: OrganizationStatus::Active,
            handle,
            name,
        }
    }
}
```

bad:
```rust
pub(super) struct OrganizationState {
    pub(super) id: OrganizationId,
    pub(super) handle: OrganizationHandle,
    pub(super) name: OrganizationName,
    pub(super) status: OrganizationStatus,
}
```

### AVOID define methods other than constructors on `AggregateState`, and keep logic out of it

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
