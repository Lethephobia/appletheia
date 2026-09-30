# Design

Guidelines for public API shape, macro contracts, and compatibility.

## Library Boundaries

### DO keep the repository generic and reusable

Keep reusable crates focused on interfaces, default implementations, and supporting infrastructure.

**Good**

```text
Library: generic event replay contract
Banking: account lifecycle rules
```

**Bad**

```text
Library event replay hardcodes which Banking account statuses permit withdrawals
```

### DON'T add application-specific implementations or definitions to library crates

Let downstream applications own business and domain concepts.

**Good**

```text
Define OrganizationMemberRelation in the Banking application
```

**Bad**

```text
Define OrganizationMemberRelation in appletheia-application
```

### PREFER abstractions that stay easy to swap out

Favor extensibility over one-off app logic.

**Good**

```rust
pub struct Service<R: Repository<Account>> {
    repository: R,
}
```

**Bad**

```rust
pub struct Service {
    repository: PgAccountRepository, // Couples reusable orchestration to one backend.
}
```

### AVOID assuming downstream application behavior lives in this repository

Keep downstream concerns out of the library surface.

**Good**

```text
A library contract accepts downstream aggregate implementations
```

**Bad**

```text
A library contract imports banking_iam_domain::User to function
```

### CONSIDER making library boundaries explicit when behavior varies

Use traits and adapters when a concern needs to vary by application.

**Good**

```text
Vary persistence through a store trait implemented by infrastructure adapters
```

**Bad**

```text
Branch on application names inside the library to choose persistence behavior
```

## Facade Crates

### DO keep the top-level `appletheia` crate as a thin feature-gated facade

Re-export the subcrates behind feature flags instead of adding implementation logic to the facade.

**Good**

```rust
#[cfg(feature = "domain")]
pub use appletheia_domain as domain;
```

**Bad**

```rust
// In the facade crate:
impl AggregateReplayEngine {
    pub fn replay(&mut self) { /* replay algorithm */ }
}
```

### PREFER feature flags for the domain, application, infrastructure, and macro surfaces

Let downstream users opt into the parts they need without pulling the whole workspace surface into every build.

**Good**

```rust
#[cfg(feature = "domain")]
pub use appletheia_domain as domain;

#[cfg(feature = "infrastructure")]
pub use appletheia_infrastructure as infrastructure;
```

**Bad**

```toml
[features]
domain = ["dep:appletheia-domain", "dep:appletheia-infrastructure"] # Unrelated backend dependency.
```

### DON'T put concrete library behavior in the facade crate

Keep the facade focused on wiring and re-exports.

**Good**

```text
Facade re-exports domain types; domain crate implements replay
```

**Bad**

```text
Facade implements replay while domain crate is only a type container
```

## Public Surface

### DO treat macro expansion as part of the public contract

Keep the generated API stable unless you are intentionally making a breaking change.

**Good**

```text
Change generated name() signature -> check downstream calls and identify the API break
```

**Bad**

```text
Change generated name() signature -> classify as private because the method is macro-generated
```

### DO check whether a change alters trait signatures, public types, event names, or serialized shapes

Treat those changes as contract changes and review them explicitly.

**Good**

```text
Before renaming a serialized variant, compare old/new JSON and identify affected consumers
```

**Bad**

```text
Rename a serialized variant as an internal cleanup without checking wire compatibility
```

### PREFER serialize JSON-facing enums as adjacently tagged `snake_case`

When an enum is serialized to JSON, prefer `#[serde(tag = "type", content = "data", rename_all = "snake_case")]` so the wire shape stays explicit and remains compatible with future tuple variants.

**Good**

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ExampleStatus {
    Active,
    Removed,
}
```

**Bad**

```rust
#[derive(Serialize, Deserialize)]
pub enum ExampleStatus {
    Active,
    Removed,
}
```

### DON'T change a public contract casually

If a breaking change is necessary, make it deliberate and visible.

**Good**

```text
Change trait signature deliberately -> document break -> update callers
```

**Bad**

```text
Change trait signature under a formatting-only commit
```

### PREFER additive changes over breaking ones

Add new types, methods, or fields when you can preserve the old contract.

**Good**

```text
Add an optional capability without changing existing implementors when feasible
```

**Bad**

```text
Add a required method to a public trait for a capability existing users do not need
```

### PREFER dedicated collection types only when the public API treats the collection as one named value

If a collection is configured, validated, serialized, and passed around as one declared value, a dedicated wrapper type can make the contract clearer. This fits public surfaces such as audience lists, issuer URL lists, and other named collections with collection-level rules.

**Good**

```rust
pub struct AuthTokenAudiences(Vec<AuthTokenAudience>);
```

**Bad**

```rust
pub struct UnrelatedValues(Vec<String>); // A wrapper with no named domain meaning or rules.
```

### AVOID introducing collection wrappers for APIs whose domain facts are item-by-item mutations

If the public API is fundamentally about adding, removing, or toggling single items, prefer exposing a raw collection in state and modeling the per-item operation explicitly. In those cases a wrapper often makes the contract look more aggregate-wide than the behavior really is.

**Good**

```text
An item-oriented API exposes explicit grant_role(role) and revoke_role(role) operations
```

**Bad**

```text
Wrap roles and expose only replace_all_roles just to perform a single grant operation
```

### CONSIDER the raw collection semantics before adding a wrapper

When a wrapper is unnecessary, still choose the raw collection deliberately. Prefer `Vec` for ordered declarations and `BTreeSet` or `HashSet` for uniqueness-driven sets.

**Good**

```rust
pub struct OrderedDeclarations(Vec<Declaration>);

pub struct UniqueNames(BTreeSet<Name>);
```

**Bad**

```rust
pub struct OrderedDeclarations(HashSet<Declaration>); // Loses declaration order.
```

### AVOID renaming public types or event names when an additive path exists

Prefer compatibility-preserving changes before removing old names.

**Good**

```text
When compatibility is required, introduce the replacement while retaining a migration path
```

**Bad**

```text
Rename a published event discriminator without considering stored events or subscribers
```

### CONSIDER deprecating before removing

Use a transition period when downstream crates need time to migrate.

**Good**

```text
Give downstream users a supported replacement and a deprecation period when needed
```

**Bad**

```text
Remove a widely used public method without a replacement or migration notice
```

## Macros

### DO update fixtures when generated code changes

Regenerate or refresh the expected output alongside the macro change.

**Good**

```text
Intentional expansion change -> review and update expected expansion -> rerun tests
```

**Bad**

```text
Intentional expansion change -> leave stale expected output and disable the failing assertion
```

### DON'T verify expansion behavior in downstream crates

Keep macro expansion tests in `appletheia-macros` so the contract lives with the implementation.

**Good**

```text
Macro crate: expansion assertions
Application crate: behavior of generated types
```

**Bad**

```text
Application crate duplicates the macro crate's token-by-token assertions
```

### PREFER keeping macro error messages stable when practical

Stable errors make generated APIs easier to use and easier to test.

**Good**

```text
Keep a useful diagnostic stable while changing only the implementation
```

**Bad**

```text
Rephrase unchanged diagnostics on every refactor and churn compile-fail expectations
```

### AVOID making the generated surface larger than the API needs

Keep the macro surface minimal so the generated contract stays easy to understand.

**Good**

```text
Generate the required trait implementation and documented conveniences
```

**Bad**

```text
Generate public accessors for every internal field regardless of the intended contract
```

### CONSIDER the smallest macro surface that still expresses the API clearly

Prefer a smaller expansion when it keeps the contract easier to reason about.

**Good**

```text
One documented configuration option controls the needed generated behavior
```

**Bad**

```text
Add several synonymous options that generate identical behavior
```

## Compatibility

### DO update docs, examples, and tests together when a break is unavoidable

Keep the release story aligned with the API change.

**Good**

```text
Rename required builder method -> update docs, example calls, and contract tests
```

**Bad**

```text
Rename required builder method -> ship examples that no longer compile
```

### DO verify replay and snapshot restore against the current model after event-sourced changes

Keep tests and examples aligned with the current design, even when that means rewriting fixture
event shapes.

**Good**

```text
Change state representation -> verify event replay and current snapshot restoration
```

**Bad**

```text
Change state representation -> check only that the constructor succeeds
```

### DON'T keep compatibility scaffolding in example crates when it obscures the intended design

Examples should show the current model directly instead of carrying migrations, upcasters, or
legacy payload branches.

**Good**

```text
Update Banking payloads and tests to demonstrate the current model directly
```

**Bad**

```text
Keep LegacyUserState and old fixture upcasters in an example that does not teach migration
```

### PREFER reviewing contract impact before merge

Be explicit about what changed, but optimize examples for clarity over historical compatibility.

**Good**

```text
PR explains changed trait signatures, wire shapes, and required caller changes
```

**Bad**

```text
PR calls a serialized-shape change an implementation detail
```

### PREFER preserving semver in library crates, but allow example fixtures to break when the design improves

Library APIs still need compatibility review. Example domains and their serialized payloads do not
need legacy-preserving glue.

**Good**

```text
Review downstream library compatibility; rewrite example-only payload fixtures for the new design
```

**Bad**

```text
Require production migration scaffolding solely to preserve unused Banking fixture data
```

### AVOID adding migrations or upcasters to examples unless the example is specifically about migration

Keep example event payloads simple and current.

**Good**

```text
Migration tutorial: show an upcaster intentionally
Ordinary Banking example: show current payloads
```

**Bad**

```text
Add an upcaster to every example event just to keep outdated test data
```

### CONSIDER versioned payloads only for real library or production contracts

Do not add versioning noise to examples just to preserve old fixture data.

**Good**

```text
Version a payload when deployed consumers or persisted production events require coexistence
```

**Bad**

```text
Add V1/V2 variants solely to retain obsolete, unused example fixtures
```
