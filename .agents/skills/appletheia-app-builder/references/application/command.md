# Command Guidelines

Use for command payloads, command handlers, authorization, validation, and orchestration.

## Command

### DO model commands as intent-only messages

Commands should describe what the caller wants to do, not how the application will do it.

good:
```rust
pub struct OrganizationChangeNameCommand {
    pub organization_id: OrganizationId,
    pub name: OrganizationName,
}
```

bad:
```rust
pub struct OrganizationChangeNameCommand {
    pub organization: Organization,
    pub name: String,
    pub principal: Principal,
}
```

### DO keep command payloads serializable and transport-friendly

Use value objects and ids that can move across process boundaries cleanly.

good:
```rust
pub struct AccountOpenCommand {
    pub owner_id: UserId,
    pub currency_definition_id: CurrencyDefinitionId,
}
```

bad:
```rust
pub struct AccountOpenCommand {
    pub owner: AccountOwner,
    pub currency_definition: CurrencyDefinition,
}
```

### DO keep command input minimal

Include only the data that is necessary to express the intent.

good:
```rust
pub struct OrganizationRemoveCommand {
    pub organization_id: OrganizationId,
}
```

bad:
```rust
pub struct OrganizationRemoveCommand {
    pub organization_id: OrganizationId,
    pub organization_name: OrganizationName,
    pub organization_handle: OrganizationHandle,
}
```

### DO keep principal and actor out of command payloads

Security context belongs in `RequestContext`, not in the command struct.

good:
```rust
pub struct CurrencyDefinitionDefineCommand {
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
}
```

bad:
```rust
pub struct CurrencyDefinitionDefineCommand {
    pub principal: Principal,
    pub symbol: CurrencySymbol,
    pub name: CurrencyName,
    pub decimals: CurrencyDecimals,
}
```

### PREFER one command per user intent

Split commands when they represent different business actions, especially when authorization differs.
If the domain object is edited as a coupled set of fields, a patch command can still be a good fit.

good:
```rust
pub struct OrganizationChangeNameCommand;
pub struct OrganizationChangeHandleCommand;
pub struct OrganizationRemoveCommand;
```

bad:
```rust
pub struct OrganizationUpdateCommand {
    pub name: Option<OrganizationName>,
    pub handle: Option<OrganizationHandle>,
}
```

### CONSIDER `FieldPatch` for coupled fields

Use `FieldPatch` when several fields of one conceptual object are expected to change together and omitted fields mean "leave unchanged".
This fits user profile or settings surfaces better than splitting every field into its own command.

good:
```rust
pub struct UserProfileEditCommand {
    pub username: FieldPatch<Username>,
    pub display_name: FieldPatch<UserDisplayName>,
    pub bio: FieldPatch<Option<UserBio>>,
}
```

bad:
```rust
pub struct OrganizationPatchCommand {
    pub name: FieldPatch<OrganizationName>,
    pub handle: FieldPatch<OrganizationHandle>,
}
```

## CommandHandler

### DO load the aggregate, invoke its command method, and save the result

Keep state transitions inside the aggregate boundary.

good:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_name(command.name)?;
repository.save(uow, &organization).await?;
```

bad:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.state_mut().name = command.name;
repository.save(uow, &organization).await?;
```

### DON'T touch `RequestContext.actor` in command handlers

The default command dispatcher already authorizes commands with `principal`.
Use `actor` only when a workflow explicitly needs provenance or persistence context, not for routine authorization decisions.

good:
```rust
let _principal = request_context.principal.clone();
```

bad:
```rust
let actor = &request_context.actor;
```

### DON'T mutate aggregate state directly in the handler

The handler should orchestrate, not reimplement domain logic.

bad:
```rust
let mut account = repository.find_by_id(uow, command.account_id).await?;
account.state_mut().name = command.name;
```

good:
```rust
let mut account = repository.find_by_id(uow, command.account_id).await?;
account.rename(command.name)?;
```

### DO keep cross-aggregate validation in the handler when the rule cannot live inside one aggregate

Use the handler for lookups that span multiple aggregates or read models.

good:
```rust
let organization = organization_repository.find_by_id(uow, command.organization_id).await?;
if organization.is_removed() {
    return Err(OrganizationChangeNameCommandHandlerError::OrganizationRemoved);
}
```

bad:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.ensure_not_removed()?;
```

### DON'T duplicate aggregate-owned validation in the handler

If the aggregate command method already enforces a rule, let the aggregate own that failure path.
Reserve handler-side checks for rules that need other aggregates or read models.

good:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_name(command.name)?;
```

bad:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
if organization.is_removed() {
    return Err(OrganizationChangeNameCommandHandlerError::OrganizationRemoved);
}

organization.change_name(command.name)?;
```

### DON'T orchestrate multi-aggregate workflows directly in the handler

Use a saga when one command needs to emit follow-up commands for another aggregate.

bad:
```rust
let mut invitation = invitation_repository.find_by_id(uow, command.invitation_id).await?;
invitation.accept()?;

let mut membership = membership_repository.find_by_id(uow, command.membership_id).await?;
membership.create()?;
```

good:
```rust
let mut invitation = invitation_repository.find_by_id(uow, command.invitation_id).await?;
invitation.accept()?;
```

### DON'T depend on read model stores or relationship stores in command handlers

Command handlers should work through aggregate repositories and domain methods.
If a workflow needs read model data or relationship graph queries, move that concern to a separate query path or workflow service.

good:
```rust
let mut organization = organization_repository.find_by_id(uow, command.organization_id).await?;
organization.change_name(command.name)?;
```

bad:
```rust
let members = relationship_store.read_subjects_by_aggregate(...).await?;
let summary = read_model_store.find_by_organization_id(...).await?;
```

### DO map domain errors into handler errors

Return application-specific errors from the handler boundary.

good:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_handle(command.handle)?;
```

bad:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_handle(command.handle).unwrap();
```

### PREFER handlers to return outputs from persisted ids or resulting state

Return what the caller needs to continue, not extra read-model data.

good:
```rust
Ok(OrganizationRemoveOutput {
    organization_id: command.organization_id,
})
```

bad:
```rust
Ok(OrganizationRemoveOutput {
    organization: repository.find_by_id(uow, command.organization_id).await?,
})
```

### PREFER one unit of work per handler

Keep the transaction boundary aligned with the command boundary unless a workflow explicitly needs more.

good:
```rust
let mut uow = repository.begin().await?;
// load -> authorize -> mutate -> save
uow.commit().await?;
```

bad:
```rust
let mut uow1 = repository.begin().await?;
let mut uow2 = repository.begin().await?;
```

### DON'T hide one-shot domain failures in the handler

If the aggregate rejects a repeated create, open, approve, or accept call, let that failure surface.

good:
```rust
organization.remove()?;
```

bad:
```rust
if organization.is_removed() {
    return Ok(());
}

organization.remove()?;
```
