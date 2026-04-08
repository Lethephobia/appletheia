# Usage

Guidelines for tests, example crates, and how to exercise the library in practice.

## Checks

### DO run `cargo fmt --all` after editing Rust code

Keep formatting consistent before review or commit.

### PREFER run `cargo clippy` for the touched crate or workspace slice first

Use the narrowest scope that covers the change, then widen to the workspace when shared code or cross-crate contracts change.

### PREFER run `cargo test` for the touched crate or workspace slice first

Run the smallest test scope that covers the change, then expand when the change affects shared traits, macros, or generated code.

### CONSIDER rerunning checks after regenerating code or fixtures

Refresh generated output first, then rerun the checks that validate it.

## Tests

### DO keep unit tests close to the code they cover

Place focused unit tests near the implementation they verify.

### PREFER use `thiserror::Error` for custom test errors

Keep test-only error types small and readable.

good:
```rust
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("invalid test state")]
struct TestError;
```

bad:
```rust
#[derive(Debug, PartialEq, Eq)]
struct TestError;
```

### DON'T add application-specific behavior to library tests

Keep tests centered on the contract the library guarantees.

### PREFER handwritten implementations when a unit test directly targets a core trait or value object

Use helper types only when they make the contract clearer. Prefer the handwritten item under test itself, and let helper types use macros when they keep the fixture easier to read.

### PREFER use proc macros from `appletheia-macros` in non-macro test crates when it improves readability

Add them through `dev-dependencies` when they improve readability for test-only helper types.

### AVOID testing macro expansion details in downstream crates

Keep expansion assertions in `appletheia-macros`. Downstream domain, application, and infrastructure tests should focus on the trait or API being verified.

### DO test macro expansion behavior in `appletheia-macros`

Keep expansion assertions close to the macro that defines the contract.

### CONSIDER example-crate tests when they prove library behavior more clearly

Use example crates to verify integration points and generated code when that improves confidence.

## Examples

### DO update example crates when they are used to demonstrate library behavior

Keep examples aligned with the contract they are meant to exercise.

### PREFER the smallest example that proves the behavior

Show the contract without pulling in unrelated application logic.
