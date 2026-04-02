---
name: appletheia-app-builder
description: Build CQRS and event-sourced applications with Appletheia. Use when designing application architecture, aggregates, commands, projections, sagas, authorization, authentication, or other downstream application code built on Appletheia.
---

# Appletheia App Builder

Guide downstream Appletheia application design and implementation across architecture, aggregates, commands, projections, sagas, authorization, authentication, and related application concerns.

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

- `references/domain/aggregate.md`

  Use for aggregate boundaries, command methods, state transitions, and event application.
