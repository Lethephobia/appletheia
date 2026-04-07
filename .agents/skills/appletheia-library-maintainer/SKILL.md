---
name: appletheia-library-maintainer
description: Maintain the Appletheia library, macros, and examples. Use when changing crate APIs, trait contracts, macro expansion, generated code, docs, compatibility, release behavior, commits, or commit messages.
---

# Appletheia Library Maintainer

Guide maintenance of the reusable `appletheia` library surface, its macro crate, and example fixtures.

## References

The reference files follow an Effective Dart style:

- Do

  Use this for rules that should be followed by default. Treat violations as exceptional and require a clear reason.

- DON'T

  Use this for things to avoid. If a design depends on one of these, revisit the approach first.

- PREFER

  Use this for the recommended default. It is acceptable to choose another path when the context justifies it.

- AVOID

  Use this for patterns that are usually a bad fit. Keep them for cases where the alternative has a clear cost.

- CONSIDER

  Use this for optional guidance or tradeoffs. Apply it when the surrounding context makes the choice worthwhile.

### Reference Map

- `references/style.md`

  Use for repository-wide Rust style, file layout, imports, and source organization.

- `references/documentation.md`

  Use for doc comments, prose style, and public API documentation.

- `references/usage.md`

  Use for tests, example crates, and how to exercise the library in practice.

- `references/design.md`

  Use for public API shape, macro contracts, compatibility, and semver.

- `references/release.md`

  Use for commit messages and release-facing guidance.
