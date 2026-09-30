---
name: appletheia-library-maintainer
description: Maintain the Appletheia library, macros, and examples. Use when changing crate APIs, trait contracts, macro expansion, generated code, docs, compatibility, release behavior, commits, commit messages, or repository skills.
---

# Appletheia Library Maintainer

Guide maintenance of the reusable `appletheia` library surface, its macro crate, and example fixtures.

## References

Select the references relevant to the task from the index below; read additional references only
when their guidance is needed. The reference files follow an Effective Dart style:

- DO

  Use this for rules that should be followed by default. Treat violations as exceptional and require a clear reason.

- DON'T

  Use this for things to avoid. If a design depends on one of these, revisit the approach first.

- PREFER

  Use this for the recommended default. It is acceptable to choose another path when the context justifies it.

- AVOID

  Use this for patterns that are usually a bad fit. Keep them for cases where the alternative has a clear cost.

- CONSIDER

  Use this for optional guidance or tradeoffs. Apply it when the surrounding context makes the choice worthwhile.

Each directive includes **Good** and **Bad** fenced examples. DO, PREFER, and CONSIDER show
Good first; DON'T and AVOID show Bad first. Examples are focused excerpts, not standalone programs.
Preserve the comparisons when updating guidance and keep them aligned with the current contract.

### Reference Map

- [Style](references/style.md)

  Use for repository-wide Rust style, file layout, imports, and source organization.

- [Documentation](references/documentation.md)

  Use for doc comments, prose style, and public API documentation.

- [Usage](references/usage.md)

  Use for tests, example crates, and how to exercise the library in practice.

- [Design](references/design.md)

  Use for public API shape, macro contracts, compatibility, and semver.

- [Release](references/release.md)

  Use for commit messages and release-facing guidance.

- [Skill Authoring](references/skills.md)

  Use for skill scope, SKILL.md reading guidance and indexes, reference structure, examples, and
  preserving design decisions during updates.
