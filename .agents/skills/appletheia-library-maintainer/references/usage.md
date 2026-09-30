# Usage

Guidelines for tests, example crates, and how to exercise the library in practice.

## Checks

### DO run `cargo fmt --all` after editing Rust code

Keep formatting consistent before review or commit.

**Good**

```sh
cargo fmt --all
```

**Bad**

```sh
git commit -am "fix(domain): adjust validation" # Rust edits were not formatted.
```

### PREFER run `cargo clippy` for the touched crate or workspace slice first

Use the narrowest scope that covers the change, then widen to the workspace when shared code or cross-crate contracts change.

**Good**

```sh
cargo clippy -p appletheia-domain --all-targets
```

**Bad**

```sh
cargo clippy -p appletheia-macros # Only aggregate domain code changed; this misses it.
```

### PREFER run `cargo test` for the touched crate or workspace slice first

Run the smallest test scope that covers the change, then expand when the change affects shared traits, macros, or generated code.

**Good**

```sh
cargo test -p appletheia-domain
```

**Bad**

```sh
cargo test -p appletheia-macros # Does not cover the changed domain implementation.
```

### CONSIDER rerunning checks after regenerating code or fixtures

Refresh generated output first, then rerun the checks that validate it.

**Good**

```text
Change macro -> refresh expected output -> rerun affected macro tests
```

**Bad**

```text
Run tests -> regenerate changed output -> assume earlier test results cover the new output
```

## Tests

### DO keep unit tests close to the code they cover

Place focused unit tests near the implementation they verify.

**Good**

```rust
// example_value.rs
#[cfg(test)]
mod tests {
    use super::ExampleValue;
    // Focused ExampleValue tests.
}
```

**Bad**

```rust
// unrelated_selector.rs
#[cfg(test)]
mod tests {
    // Tests for ExampleValue live here despite no connection to this module.
}
```

### PREFER use `thiserror::Error` for custom test errors

Keep test-only error types small and readable.

**Good**

```rust
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("invalid test state")]
struct TestError;
```

**Bad**

```rust
#[derive(Debug, PartialEq, Eq)]
struct TestError;
```

### DON'T add application-specific behavior to library tests

Keep tests centered on the contract the library guarantees.

**Good**

```text
Repository test: saved events can be loaded and replayed
```

**Bad**

```text
Repository test: Banking account freezes after a particular business policy threshold
```

### PREFER handwritten implementations when a unit test directly targets a core trait or value object

Use helper types only when they make the contract clearer. Prefer the handwritten item under test itself, and let helper types use macros when they keep the fixture easier to read.

**Good**

```text
Aggregate trait test: handwritten minimal aggregate implements the contract directly
```

**Bad**

```text
Aggregate trait test: assertions only inspect macro-generated tokens instead of trait behavior
```

### PREFER use proc macros from `appletheia-macros` in non-macro test crates when it improves readability

Add them through `dev-dependencies` when they improve readability for test-only helper types.

**Good**

```text
Use a macro for a helper payload while testing repository behavior
```

**Bad**

```text
Copy a long handwritten payload implementation into every repository test fixture
```

### AVOID testing macro expansion details in downstream crates

Keep expansion assertions in `appletheia-macros`. Downstream domain, application, and infrastructure tests should focus on the trait or API being verified.

**Good**

```text
Application test: generated payload works with event handling
```

**Bad**

```text
Application test: exact generated token string must match a snapshot
```

### DO test macro expansion behavior in `appletheia-macros`

Keep expansion assertions close to the macro that defines the contract.

**Good**

```text
Change generated method signature -> add or update coverage in appletheia-macros
```

**Bad**

```text
Change generated method signature -> check only an unrelated domain unit test
```

### CONSIDER example-crate tests when they prove library behavior more clearly

Use example crates to verify integration points and generated code when that improves confidence.

**Good**

```text
Use a Banking command test to check changed repository and handler integration
```

**Bad**

```text
Assume a macro token test alone proves Banking still works with the changed contract
```

## Examples

### DO update example crates when they are used to demonstrate library behavior

Keep examples aligned with the contract they are meant to exercise.

**Good**

```text
Change Saga builder API -> update Banking saga routes alongside the library
```

**Bad**

```text
Change Saga builder API -> leave Banking examples using removed methods
```

### PREFER the smallest example that proves the behavior

Show the contract without pulling in unrelated application logic.

**Good**

```text
One aggregate and two events demonstrate replay
```

**Bad**

```text
Add OIDC, object storage, and a transfer saga to demonstrate a single value conversion
```
