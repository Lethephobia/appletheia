# Release

Guidelines for commit messages and release-facing changes.

## Commit Messages

### DO use Conventional Commits for generated or proposed commit messages

Follow the repository format in `CONTRIBUTING.md`: `<type>(<scope>)!: <subject>`.
Use a short, imperative subject without a trailing period.

**Good**

```text
feat(authorization): add relationship resolver config
```

**Bad**

```text
Added relationship resolver configuration
```

### DON'T invent a local commit style for routine changes

Use the repository convention instead of ad hoc prefixes, emoji, or free-form subject lines.

**Bad**

```text
✨ fixed some stuff
```

**Good**

```text
fix(domain): reject invalid aggregate state
```

### PREFER a short scope that identifies the affected crate or subsystem

Use the crate name when the change is clearly confined to one crate.

**Good**

```text
docs(app-builder): restore aggregate guidance
```

**Bad**

```text
docs(all-the-library-macros-and-applications): restore aggregate guidance
```

### CONSIDER calling out breaking changes explicitly for library-facing changes

Use `!` or a `BREAKING CHANGE:` footer when the change affects downstream crates. Example-only
fixture and documentation refactors do not need compatibility framing beyond a clear subject.

**Good**

```text
refactor(application)!: change saga handler contract
```

**Bad**

```text
refactor(application): internal cleanup

Removes a required public handler method without noting the break.
```
