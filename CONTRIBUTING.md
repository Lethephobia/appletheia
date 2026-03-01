# Contributing

Thanks for contributing to **appletheia**.

This repository is a **general-purpose library** for building CQRS / Event Sourcing applications.

## Commit message convention

Use the **Conventional Commits** format so commits can be parsed for changelogs and releases.

Reference: [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)

### Git commit template (optional)

This repository provides a commit message template at `.gitmessage`.

To enable it for this repository:

```bash
git config commit.template .gitmessage
```

To enable it globally for all repositories:

```bash
git config --global commit.template /absolute/path/to/.gitmessage
```

### Format

```
<type>(<scope>)!: <subject>

<body>

<footer>
```

- `type`: required (see below)
- `scope`: optional, but recommended
- `!`: optional; use it for breaking changes
- `subject`: required; short, imperative, no trailing period
- `body`: optional; explain *why* and *how*
- `footer`: optional; breaking-change details, issue refs, etc.

### Types

Use one of the following:

- `feat`: new feature
- `fix`: bug fix
- `refactor`: code change that neither fixes a bug nor adds a feature
- `perf`: performance improvement
- `docs`: documentation-only changes
- `test`: tests only
- `build`: build system or dependencies
- `ci`: CI configuration/scripts
- `chore`: maintenance work that doesn't fit above

### Scope

Use a short scope that points to the affected area. Common examples:

- crate/module: `authorization`, `projection`, `repository`, `event`, `snapshot`, `messaging`
- infra adapter: `postgresql`, `google-cloud-pubsub`
- top-level: `workspace`

If a change is clearly confined to one crate, prefer the crate name as the scope.

### Breaking changes

If a change breaks API/behavior, use one of:

- Add `!` after the scope/type: `feat(authorization)!: ...`
- Or add a footer:
  - `BREAKING CHANGE: ...`

### Examples

```
feat(authorization): add relationship resolver config
```

```
fix(postgresql): handle null subject_relation correctly
```

```
refactor(repository)!: remove access traits from default repository

BREAKING CHANGE: EventReaderAccess/SnapshotReaderAccess/... are removed.
```
