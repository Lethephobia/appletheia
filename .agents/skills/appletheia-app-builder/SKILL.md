---
name: appletheia-app-builder
description: Build CQRS and event-sourced applications with Appletheia. Use when designing application architecture, aggregates, commands, projections, sagas, authorization, authentication, or other downstream application code built on Appletheia.
---

# Appletheia App Builder

Guide downstream Appletheia application design and implementation across architecture, aggregates, commands, projections, sagas, authorization, authentication, and related application concerns.

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

Keep guidance focused on application decisions that are easy to get wrong with Appletheia.
Do not turn a one-time cleanup or an obsolete API into a permanent prohibition. Consolidate
overlapping rules and avoid repeating ordinary Rust advice or library internals. Before removing or
rewriting guidance, compare its history and current examples: preserve the underlying design decision
when only its API or sample code is obsolete. A shorter guide must not reverse an existing rule.

### Reference Map

- [Aggregate](references/domain/aggregate.md)

  Use for aggregate invariants, operation failures, replayable state changes, persistence indexes,
  identity metadata, value-object boundaries, and separate child-entity creation events.

- [Command](references/application/command.md)

  Use for command data and output shapes, application-level checks, transaction boundaries,
  replay-safe outputs, retryability, and worker configuration.

- [Saga](references/application/saga.md)

  Use for staged routes, incoming and outgoing steps, side-effect-free definitions, context-based
  dispatch, command ownership, parallel readiness, and optional failure reactions.
