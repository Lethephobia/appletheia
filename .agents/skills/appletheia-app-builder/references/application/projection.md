# Projection Design

Use this reference when an Appletheia application materializes committed events into read-model
fragments.

## Projector boundary

### DO keep one projector focused on one physical fragment type

Let the projector select the events that can change its fragment, materialize only affected
partitions, and return those partitions for downstream invalidation. Keep command handling and
cross-aggregate workflow coordination outside the projector.

## Worker lifecycle

### DO share the projector worker across projectors

Construct one worker from the shared runner and event subscriber. Pass each projector by reference
when starting its long-running consumer instead of storing the projector in the worker.

```rust
let worker = Arc::new(DefaultProjectorWorker::new(projector_runner, event_subscriber));

let user_worker = Arc::clone(&worker);
tokio::spawn(async move {
    user_worker.run_forever(&UserFragmentProjector).await
});

let account_worker = Arc::clone(&worker);
tokio::spawn(async move {
    account_worker.run_forever(&AccountFragmentProjector).await
});
```

Each call derives its consumer group and subscription from `PJ::Spec::DESCRIPTOR`. The worker's
graceful-stop flag is shared, so requesting a stop ends every projector consumer running on that
worker. Each projector must use the same unit-of-work type as the shared runner.
