# Design

Guidelines for public API shape, macro contracts, and compatibility.

## Library Boundaries

### DO keep the repository generic and reusable

Keep reusable crates focused on interfaces, default implementations, and supporting infrastructure.

### DON'T add application-specific implementations or definitions to library crates

Let downstream applications own business and domain concepts.

### PREFER abstractions that stay easy to swap out

Favor extensibility over one-off app logic.

### AVOID assuming downstream application behavior lives in this repository

Keep downstream concerns out of the library surface.

### CONSIDER making library boundaries explicit when behavior varies

Use traits and adapters when a concern needs to vary by application.

## Facade Crates

### DO keep the top-level `appletheia` crate as a thin feature-gated facade

Re-export the subcrates behind feature flags instead of adding implementation logic to the facade.

### PREFER feature flags for the domain, application, infrastructure, and macro surfaces

Let downstream users opt into the parts they need without pulling the whole workspace surface into every build.

### DON'T put concrete library behavior in the facade crate

Keep the facade focused on wiring and re-exports.

## Public Surface

### DO treat macro expansion as part of the public contract

Keep the generated API stable unless you are intentionally making a breaking change.

good:
```rust
// The generated trait method stays part of the contract.
```

bad:
```rust
// Generated output changes without any compatibility review.
```

### DO check whether a change alters trait signatures, public types, event names, or serialized shapes

Treat those changes as contract changes and review them explicitly.

### DON'T change a public contract casually

If a breaking change is necessary, make it deliberate and visible.

### PREFER additive changes over breaking ones

Add new types, methods, or fields when you can preserve the old contract.

### AVOID renaming public types or event names when an additive path exists

Prefer compatibility-preserving changes before removing old names.

### CONSIDER deprecating before removing

Use a transition period when downstream crates need time to migrate.

## Macros

### DO update fixtures when generated code changes

Regenerate or refresh the expected output alongside the macro change.

### DON'T verify expansion behavior in downstream crates

Keep macro expansion tests in `appletheia-macros` so the contract lives with the implementation.

### PREFER keeping macro error messages stable when practical

Stable errors make generated APIs easier to use and easier to test.

### AVOID making the generated surface larger than the API needs

Keep the macro surface minimal so the generated contract stays easy to understand.

### CONSIDER the smallest macro surface that still expresses the API clearly

Prefer a smaller expansion when it keeps the contract easier to reason about.

## Compatibility

### DO update docs, examples, and tests together when a break is unavoidable

Keep the release story aligned with the API change.

### DO verify replay and snapshot restore after event-sourced changes

Make sure persisted data still loads correctly after the change.

### DON'T assume old serialized data will continue to work forever

Persisted shapes need a compatibility strategy when they evolve.

### PREFER reviewing compatibility before merge

Check the impact before the change reaches main.

### PREFER preserving semver where possible and calling out breaks explicitly

Make compatibility expectations visible when a change is not additive.

### AVOID changing serialized shape without a migration or upcaster plan

Preserve old data until you have a deliberate transition path.

### CONSIDER versioned payloads when the model must evolve in place

Use versioning when additive changes are no longer enough.
