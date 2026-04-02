# Release

Guidelines for commit messages and release-facing changes.

## Commit Messages

### DO use Conventional Commits for generated or proposed commit messages

Follow the repository format in `CONTRIBUTING.md`: `<type>(<scope>)!: <subject>`.
Use a short, imperative subject without a trailing period.

good:
```text
feat(authorization): add relationship resolver config
```

bad:
```text
Added relationship resolver configuration
```

### DON'T invent a local commit style for routine changes

Use the repository convention instead of ad hoc prefixes, emoji, or free-form subject lines.

### PREFER a short scope that identifies the affected crate or subsystem

Use the crate name when the change is clearly confined to one crate.

### CONSIDER calling out breaking changes explicitly

Use `!` or a `BREAKING CHANGE:` footer when the change affects downstream crates.
