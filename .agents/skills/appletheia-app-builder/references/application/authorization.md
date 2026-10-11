# Authorization Design

Use this reference for relationship derivation, permission inheritance, explicit grants, and membership lifecycle boundaries.

## Relationship Structure

### PREFER TupleToUserset for permissions inherited through structural relationships

Persist the structural edge to the related aggregate, then express which of its relations to follow
in `UsersetExpr::TupleToUserset`. Examples include an Account's owning Organization and a
RoleAssignment's Team. This keeps the relationship data independent of the permission being evaluated.

`ComputedUserset` selects another relation on the same target; `TupleToUserset` follows a relation
to another aggregate. An `AggregateSet` subject identifies a particular aggregate and relation as
the grantee. Although some checks can be expressed either way, do not replace structural edges with
permission-specific AggregateSet entries merely to avoid the traversal expression.

**Good**

```rust
// In AccountOwnerRelation's derivation: preserve the actual owner.
entries.insert::<Account, Organization>(account.aggregate_id(), organization_id);

// In AccountManagerRelation's expression: follow that owner to its permission.
UsersetExpr::TupleToUserset {
    tupleset_relation: AccountOwnerRelation::REF.into(),
    computed_userset: OrganizationFinanceManagerRelation::REF.into(),
}
```

**Bad**

```rust
// Derive a separate permission-specific edge from the same ownership field.
// Repeat this again for Treasurer, ownership management, and other permissions.
entries.insert_entry(RelationshipEntry::from_parts(
    AggregateRef::from_id::<Account>(account.aggregate_id()),
    RelationshipSubject::aggregate_set::<Organization>(
        organization_id,
        OrganizationFinanceManagerRelation::REF,
    ),
));
```

### PREFER AggregateSet when the grantee is explicitly a related group of subjects

Use `RelationshipSubject::aggregate_set` when a grant names subjects through a specific relation,
such as a Team's members or the user of an exact OrganizationMembership. The aggregate ID and
relation together express who receives the grant. Derive the entry from the aggregate that owns
the grant, such as a RoleAssignment; that source can differ from the relationship's target.

Do not expand the group's current users into persisted direct grants. Such copies would need
synchronization on every membership change and could retain access after a departure.

**Good**

```rust
// Inside the active FinanceManager assignment derivation, for a Team assignee.
entries.insert_entry(RelationshipEntry::from_parts(
    AggregateRef::from_id::<Organization>(assignment.organization_id()?),
    RelationshipSubject::aggregate_set::<Team>(team_id, TeamMemberRelation::REF),
));
```

**Bad**

```rust
// Snapshot current members as individual grants; later departures do not remove them.
for user_id in current_team_user_ids {
    entries.insert::<Organization, User>(assignment.organization_id()?, user_id);
}
```

## Permission Boundaries

### PREFER capability relations that encapsulate the permitted ownership and role paths

Keep ownership relations structural: an Organization-owned Account points to the Organization,
not directly to its owner User. Compose user-facing capabilities such as Manager, Transactor, or
OwnershipManager from those edges. Have commands require the relevant capability instead of
repeating its inheritance alternatives in every AuthorizationPlan.

Choose each capability's inheritance deliberately. For example, AccountManager includes the
personal Owner and Organization FinanceManager; AccountOwnershipManager includes the personal
Owner and Organization Owner. The latter does not include all Admins or FinanceManagers.
Membership relations still describe actual membership; a management capability does not imply membership.

**Good**

```rust
// AccountManagerRelation includes the direct owner and the organization role.
UsersetExpr::Union(vec![
    UsersetExpr::ComputedUserset {
        relation: AccountOwnerRelation::REF.into(),
    },
    UsersetExpr::TupleToUserset {
        tupleset_relation: AccountOwnerRelation::REF.into(),
        computed_userset: OrganizationFinanceManagerRelation::REF.into(),
    },
])

// Account metadata commands require that capability once.
AuthorizationPlan::OnlyPrincipals(vec![
    PrincipalRequirement::AuthenticatedWithRelationship(
        RelationshipRequirement::check::<Account>(account_id, AccountManagerRelation::REF),
    ),
])
```

**Bad**

```rust
// AccountManager already includes Owner; each command repeats the same alternative.
AuthorizationPlan::OnlyPrincipals(vec![
    PrincipalRequirement::AuthenticatedWithRelationship(
        RelationshipRequirement::check::<Account>(account_id, AccountOwnerRelation::REF),
    ),
    PrincipalRequirement::AuthenticatedWithRelationship(
        RelationshipRequirement::check::<Account>(account_id, AccountManagerRelation::REF),
    ),
])
```

### DO bind membership-scoped grants to the membership lifecycle

When a grant must end on departure and must not revive on rejoining, identify its assignee by the
specific Membership ID rather than only the User ID. Keep the corresponding user relation
conditional on active membership. An inactive assignment derives no grant, and an active assignment
to a removed membership resolves to no user even before asynchronous cleanup removes that assignment.

For Team grants, preserve both lifecycle boundaries: TeamMembership must remain active and its
referenced OrganizationMembership must remain active. Rejoining with a new ID must not reconnect
an old assignment. Validate scope compatibility when granting; merely using a typed Membership ID
does not prove that it belongs to the target Organization or Team.

**Good**

```rust
// Assignment points to the exact joining lifecycle.
RelationshipSubject::aggregate_set::<OrganizationMembership>(
    membership_id,
    OrganizationMembershipUserRelation::REF,
)

// OrganizationMembershipUserRelation derives a user only while this membership is active.
if membership.is_active()? {
    entries.insert::<OrganizationMembership, User>(
        membership.aggregate_id(),
        *membership.user_id()?,
    );
}
```

**Bad**

```rust
// A membership-scoped role is persisted directly against the user.
// Removing the membership leaves this independent grant effective.
entries.insert::<Organization, User>(organization_id, user_id);
```
