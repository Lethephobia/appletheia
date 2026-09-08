use super::{SagaDefinitionError, SagaName, SagaRoute, SagaState, SagaStep};
use crate::event::{EventEnvelope, EventSelector};
use std::error::Error;

/// Contains validated routes, ready for workers and runners.
pub struct SagaDefinition<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    name: SagaName,
    routes: Vec<SagaRoute<'a, S, T, E>>,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> SagaDefinition<'a, S, T, E> {
    /// Validates input conditions before workers start.
    pub fn new<R>(name: SagaName, routes: R) -> Result<Self, SagaDefinitionError>
    where
        R: IntoIterator<Item = SagaRoute<'a, S, T, E>>,
    {
        let collected_routes: Vec<_> = routes.into_iter().collect();
        for (index, route) in collected_routes.iter().enumerate() {
            for other_route in &collected_routes[..index] {
                match (route, other_route) {
                    (
                        SagaRoute::StartsOn { selector, .. },
                        SagaRoute::StartsOn {
                            selector: other_selector,
                            ..
                        },
                    ) if selector == other_selector => {
                        return Err(SagaDefinitionError::DuplicateEventRoute);
                    }
                    (
                        SagaRoute::OnEvent {
                            selector,
                            caused_by,
                            ..
                        },
                        SagaRoute::OnEvent {
                            selector: other_selector,
                            caused_by: other_cause,
                            ..
                        },
                    ) if selector == other_selector && caused_by == other_cause => {
                        return Err(SagaDefinitionError::DuplicateEventRoute);
                    }
                    (
                        SagaRoute::OnCommandFailed { caused_by, .. },
                        SagaRoute::OnCommandFailed {
                            caused_by: other_cause,
                            ..
                        },
                    ) if caused_by == other_cause => {
                        return Err(SagaDefinitionError::DuplicateCommandFailureRoute);
                    }
                    _ => {}
                }
            }
        }
        Ok(Self {
            name,
            routes: collected_routes,
        })
    }

    pub fn name(&self) -> SagaName {
        self.name
    }

    /// Collects distinct event selectors in route registration order.
    pub fn selectors(&self) -> Vec<EventSelector> {
        let mut selectors = Vec::new();
        for route in &self.routes {
            match route {
                SagaRoute::StartsOn { selector, .. } | SagaRoute::OnEvent { selector, .. } => {
                    if !selectors.contains(selector) {
                        selectors.push(*selector);
                    }
                }
                SagaRoute::OnCommandFailed { .. } => {}
            }
        }
        selectors
    }

    pub(crate) fn is_subscribed(&self, event: &EventEnvelope) -> bool {
        self.routes.iter().any(|route| match route {
            SagaRoute::StartsOn { selector, .. } | SagaRoute::OnEvent { selector, .. } => {
                selector.matches(event)
            }
            SagaRoute::OnCommandFailed { .. } => false,
        })
    }

    pub fn starts_on(&self, event: &EventEnvelope) -> bool {
        self.find_event_route(event, None).is_some()
    }

    /// Finds a start or continuation route without executing its callback.
    pub fn find_event_route(
        &self,
        event: &EventEnvelope,
        caused_by: Option<T>,
    ) -> Option<&SagaRoute<'a, S, T, E>> {
        self.routes.iter().find(|route| match route {
            SagaRoute::StartsOn { selector, .. } => caused_by.is_none() && selector.matches(event),
            SagaRoute::OnEvent {
                selector,
                caused_by: cause,
                ..
            } => caused_by == Some(*cause) && selector.matches(event),
            SagaRoute::OnCommandFailed { .. } => false,
        })
    }

    /// Finds a terminal failure route for the originating command step.
    pub fn find_command_failure_route(&self, caused_by: T) -> Option<&SagaRoute<'a, S, T, E>> {
        self.routes.iter().find(|route| matches!(route, SagaRoute::OnCommandFailed { caused_by: cause, .. } if *cause == caused_by))
    }
}

#[cfg(test)]
mod tests {
    mod counter {
        use appletheia_domain::{
            Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
            AggregateStateError, AggregateType, EventName, EventPayload, ReferenceIndexes,
            UniqueConstraints,
        };
        use serde::{Deserialize, Serialize};
        use std::fmt::{self, Display};
        use thiserror::Error;
        use uuid::Uuid;

        #[derive(Debug, Error)]
        pub(super) enum CounterIdError {
            #[error("nil uuid is not allowed")]
            NilUuid,
        }

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub(super) struct CounterId(Uuid);

        impl AggregateId for CounterId {
            type Error = CounterIdError;

            fn new() -> Self {
                Self(Uuid::now_v7())
            }

            fn value(&self) -> Uuid {
                self.0
            }

            fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
                if value.is_nil() {
                    return Err(CounterIdError::NilUuid);
                }

                Ok(Self(value))
            }
        }

        impl Display for CounterId {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterStateError {
            #[error(transparent)]
            AggregateState(#[from] AggregateStateError),
        }

        #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub(super) struct CounterState {
            id: CounterId,
        }

        impl UniqueConstraints<CounterStateError> for CounterState {}

        impl ReferenceIndexes<CounterStateError> for CounterState {}

        impl AggregateState for CounterState {
            type Error = CounterStateError;
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterEventPayloadError {
            #[error(transparent)]
            Serde(#[from] serde_json::Error),
        }

        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(tag = "type", content = "data", rename_all = "snake_case")]
        pub(super) enum CounterEventPayload {
            Opened,
        }

        impl EventPayload for CounterEventPayload {
            type Error = CounterEventPayloadError;

            fn name(&self) -> EventName {
                match self {
                    Self::Opened => EventName::new("opened"),
                }
            }
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterError {
            #[error(transparent)]
            Aggregate(#[from] AggregateError<CounterId>),

            #[error(transparent)]
            State(#[from] CounterStateError),
        }

        #[derive(Clone, Debug, Default)]
        pub(super) struct Counter {
            core: AggregateCore<CounterId, CounterState, CounterEventPayload>,
        }

        impl AggregateApply<CounterEventPayload, CounterError> for Counter {
            fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
                match payload {
                    CounterEventPayload::Opened => {
                        self.set_state(Some(CounterState {
                            id: CounterId::try_from_uuid(Uuid::now_v7())
                                .expect("generated uuid should be valid"),
                        }));
                    }
                }

                Ok(())
            }
        }

        impl Aggregate for Counter {
            type Id = CounterId;
            type State = CounterState;
            type EventPayload = CounterEventPayload;
            type Error = CounterError;

            const TYPE: AggregateType = AggregateType::new("counter");

            fn new() -> Self {
                Self {
                    core: AggregateCore::new(),
                }
            }

            fn from_id(id: Self::Id) -> Self {
                Self {
                    core: AggregateCore::from_id(id),
                }
            }

            fn core(&self) -> &AggregateCore<Self::Id, Self::State, Self::EventPayload> {
                &self.core
            }

            fn core_mut(
                &mut self,
            ) -> &mut AggregateCore<Self::Id, Self::State, Self::EventPayload> {
                &mut self.core
            }
        }
    }
    use crate::command::{
        Command, CommandAttemptCount, CommandFailedAt, CommandFailureEnvelope, CommandName,
        CommandOptions, CommandTerminalReason,
    };
    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSelector,
        EventSequence, SerializedEventPayload,
    };
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use crate::saga::*;

    use appletheia_domain::{
        Aggregate, AggregateVersion, EventId, EventName, EventOccurredAt, EventPayload,
    };
    use counter::{Counter, CounterEventPayload};
    use serde::{Deserialize, Serialize};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use uuid::Uuid;

    type Route<'a> = SagaRoute<'a, State, Step, Error>;

    #[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
    struct State {
        count: usize,
        closed: bool,
    }

    impl SagaState for State {}
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
    enum Step {
        First,
        Second,
    }

    impl SagaStep for Step {}
    #[derive(Debug, thiserror::Error)]
    enum Error {
        #[error(transparent)]
        Context(#[from] SagaContextError),
        #[error("policy refused")]
        Refused,
    }

    #[derive(Serialize, Deserialize)]
    struct FollowUp {}

    impl Command for FollowUp {
        const NAME: CommandName = CommandName::new("follow_up");
    }

    fn event() -> EventEnvelope {
        let message_id = MessageId::new();
        let correlation_id = CorrelationId::from(message_id.value());
        let payload = CounterEventPayload::Opened;
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).unwrap(),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Counter::TYPE),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).unwrap(),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(payload.into_json_value().unwrap()).unwrap(),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System).unwrap(),
        }
    }

    fn instance(input: &EventEnvelope) -> SagaInstance<State, Step> {
        SagaInstance::new(
            SagaNameOwned::from(SagaName::new("counter_saga")),
            input.correlation_id,
            input.event_id,
        )
    }

    const OPENED: EventName = EventName::new("opened");

    #[test]
    fn route_lookup_does_not_execute_callbacks() {
        let calls = AtomicUsize::new(0);
        let definition =
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_start_step(Step::First)
                .on::<Counter>(OPENED)
                .handle(|_, _| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
                .add_failure_step(Step::Second)
                .on(Step::First)
                .handle(|_, _| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
                .build()
                .unwrap();
        assert!(matches!(
            definition.find_event_route(&event(), None),
            Some(SagaRoute::StartsOn { .. })
        ));
        assert!(matches!(
            definition.find_command_failure_route(Step::First),
            Some(SagaRoute::OnCommandFailed { .. })
        ));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn new_rejects_duplicate_routes() {
        assert!(matches!(
            SagaDefinition::<State, Step, Error>::new(
                SagaName::new("counter_saga"),
                [
                    Route::starts_on::<Counter, _>(OPENED, Step::First, |_, _| Ok(())),
                    Route::starts_on::<Counter, _>(OPENED, Step::First, |_, _| Ok(()))
                ]
            ),
            Err(SagaDefinitionError::DuplicateEventRoute)
        ));
        assert!(matches!(
            SagaDefinition::<State, Step, Error>::new(
                SagaName::new("counter_saga"),
                [
                    Route::on_event::<Counter, _>(Step::First, OPENED, Step::First, |_, _| Ok(())),
                    Route::on_event::<Counter, _>(Step::First, OPENED, Step::First, |_, _| Ok(()))
                ]
            ),
            Err(SagaDefinitionError::DuplicateEventRoute)
        ));
        assert!(matches!(
            SagaDefinition::<State, Step, Error>::new(
                SagaName::new("counter_saga"),
                [
                    Route::on_command_failed(Step::First, Step::First, |_, _| Ok(())),
                    Route::on_command_failed(Step::First, Step::Second, |_, _| Ok(()))
                ]
            ),
            Err(SagaDefinitionError::DuplicateCommandFailureRoute)
        ));
    }

    #[test]
    fn start_and_continuation_can_share_a_selector_in_either_order() {
        for reverse in [false, true] {
            let mut routes = vec![
                Route::starts_on::<Counter, _>(OPENED, Step::First, |ctx, _| {
                    ctx.set_state(State {
                        count: 1,
                        closed: false,
                    });
                    Ok(())
                }),
                Route::on_event::<Counter, _>(Step::First, OPENED, Step::Second, |ctx, _| {
                    ctx.state_required_mut()?.count += 1;
                    Ok(())
                }),
            ];
            if reverse {
                routes.reverse();
            }
            let definition = SagaDefinition::new(SagaName::new("counter_saga"), routes).unwrap();
            let input = event();
            let mut saga = instance(&input);
            {
                let Some(
                    SagaRoute::StartsOn { step, handler, .. }
                    | SagaRoute::OnEvent { step, handler, .. },
                ) = definition.find_event_route(&input, None)
                else {
                    panic!("matching route");
                };
                let mut context =
                    SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
                handler(&mut context, &input)
            }
            .unwrap();
            {
                let Some(
                    SagaRoute::StartsOn { step, handler, .. }
                    | SagaRoute::OnEvent { step, handler, .. },
                ) = definition.find_event_route(&input, Some(Step::First))
                else {
                    panic!("matching route");
                };
                let mut context =
                    SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
                handler(&mut context, &input)
            }
            .unwrap();
            assert_eq!(saga.state.as_ref().unwrap().count, 2);
        }
    }

    #[test]
    fn same_selector_on_different_steps_subscribes_once_and_routes_by_step() {
        let definition = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [
                Route::on_event::<Counter, _>(Step::First, OPENED, Step::First, |ctx, _| {
                    ctx.set_state(State {
                        count: 1,
                        closed: false,
                    });
                    Ok(())
                }),
                Route::on_event::<Counter, _>(Step::Second, OPENED, Step::First, |ctx, _| {
                    ctx.set_state(State {
                        count: 2,
                        closed: false,
                    });
                    ctx.append_command_with_options(&FollowUp {}, CommandOptions::default())?;
                    Ok(())
                }),
            ],
        )
        .unwrap();
        let input = event();
        let mut saga = instance(&input);
        assert_eq!(
            definition.selectors(),
            vec![EventSelector::new::<Counter>(OPENED)]
        );
        assert!(definition.find_event_route(&input, None).is_none());
        {
            let Some(
                SagaRoute::StartsOn { step, handler, .. }
                | SagaRoute::OnEvent { step, handler, .. },
            ) = definition.find_event_route(&input, Some(Step::Second))
            else {
                panic!("matching route");
            };
            let mut context = SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
            handler(&mut context, &input)
        }
        .unwrap();
        assert_eq!(saga.state_required().unwrap().count, 2);
        assert_eq!(
            saga.uncommitted_commands[0]
                .saga_origin
                .as_ref()
                .unwrap()
                .step
                .try_into_step::<Step>()
                .unwrap(),
            Step::First
        );
    }

    #[test]
    fn step_builder_preserves_incoming_and_outgoing_steps() {
        let calls = AtomicUsize::new(0);
        let definition =
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_step(Step::Second)
                .on::<Counter>(Step::First, OPENED)
                .handle(|ctx, input| {
                    assert_eq!(input.payload().name(), OPENED);
                    calls.fetch_add(1, Ordering::SeqCst);
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                })
                .build()
                .unwrap();
        let input = event();
        let mut saga = instance(&input);
        assert!(
            definition
                .find_event_route(&input, Some(Step::Second))
                .is_none()
        );
        {
            let Some(
                SagaRoute::StartsOn { step, handler, .. }
                | SagaRoute::OnEvent { step, handler, .. },
            ) = definition.find_event_route(&input, Some(Step::First))
            else {
                panic!("matching route");
            };
            let mut context = SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
            handler(&mut context, &input)
        }
        .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        let command = &saga.uncommitted_commands[0];
        assert_eq!(command.causation_id, CausationId::from(input.event_id));
        assert_eq!(
            command
                .saga_origin
                .as_ref()
                .unwrap()
                .step
                .try_into_step::<Step>()
                .unwrap(),
            Step::Second
        );
    }

    #[test]
    fn context_keeps_fan_out_commands_and_assigns_input_causation() {
        let definition =
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_start_step(Step::First)
                .on::<Counter>(OPENED)
                .handle(|ctx, _| {
                    ctx.set_state(State::default());
                    ctx.append_command(&FollowUp {})?;
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                })
                .add_failure_step(Step::Second)
                .on(Step::First)
                .handle(|ctx, _| {
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                })
                .build()
                .unwrap();
        let input = event();
        let mut saga = instance(&input);
        {
            let Some(
                SagaRoute::StartsOn { step, handler, .. }
                | SagaRoute::OnEvent { step, handler, .. },
            ) = definition.find_event_route(&input, None)
            else {
                panic!("matching route");
            };
            let mut context = SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
            handler(&mut context, &input)
        }
        .unwrap();
        assert_eq!(saga.uncommitted_commands.len(), 2);
        assert_ne!(
            saga.uncommitted_commands[0].message_id,
            saga.uncommitted_commands[1].message_id
        );
        for command in &saga.uncommitted_commands {
            assert_eq!(command.causation_id, CausationId::from(input.event_id));
            assert_eq!(
                command
                    .saga_origin
                    .as_ref()
                    .unwrap()
                    .step
                    .try_into_step::<Step>()
                    .unwrap(),
                Step::First
            );
            assert_eq!(
                command.saga_origin.as_ref().unwrap().saga_instance_id,
                saga.saga_instance_id
            );
        }
        let command = saga.uncommitted_commands[0].clone();
        let failure = CommandFailureEnvelope::new(
            &command,
            command.saga_origin.clone().unwrap(),
            CommandTerminalReason::NonRetryable,
            CommandAttemptCount::first(),
            CommandFailedAt::now(),
        );
        saga.clear_uncommitted_commands();
        {
            let Some(SagaRoute::OnCommandFailed { step, handler, .. }) =
                definition.find_command_failure_route(Step::First)
            else {
                panic!("matching route");
            };
            let mut context =
                SagaContext::new(&mut saga, CausationId::from(failure.failure_id), *step);
            handler(&mut context, &failure)
        }
        .unwrap();
        assert_eq!(
            saga.uncommitted_commands[0].causation_id,
            CausationId::from(failure.failure_id)
        );
        assert_eq!(
            saga.uncommitted_commands[0]
                .saga_origin
                .as_ref()
                .unwrap()
                .step
                .try_into_step::<Step>()
                .unwrap(),
            Step::Second
        );
        let commands_before = saga.uncommitted_commands.len();
        assert!(
            definition
                .find_command_failure_route(Step::Second)
                .is_none()
        );
        assert_eq!(saga.uncommitted_commands.len(), commands_before);
    }

    #[test]
    fn decode_errors_do_not_call_the_handler() {
        let calls = AtomicUsize::new(0);
        let definition = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [Route::starts_on::<Counter, _>(
                EventName::new("renamed"),
                Step::First,
                |_, _| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                },
            )],
        )
        .unwrap();
        let mut input = event();
        let mut saga = instance(&input);
        input.event_name = EventNameOwned::from(EventName::new("renamed"));
        assert!(matches!(
            {
                let Some(
                    SagaRoute::StartsOn { step, handler, .. }
                    | SagaRoute::OnEvent { step, handler, .. },
                ) = definition.find_event_route(&input, None)
                else {
                    panic!("matching route");
                };
                let mut context =
                    SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
                handler(&mut context, &input)
            },
            Err(SagaRouteError::EventNameMismatch)
        ));
        input.payload =
            SerializedEventPayload::try_from(serde_json::json!({"type":"invalid"})).unwrap();
        assert!(matches!(
            {
                let Some(
                    SagaRoute::StartsOn { step, handler, .. }
                    | SagaRoute::OnEvent { step, handler, .. },
                ) = definition.find_event_route(&input, None)
                else {
                    panic!("matching route");
                };
                let mut context =
                    SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
                handler(&mut context, &input)
            },
            Err(SagaRouteError::EventEnvelope(_))
        ));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    struct Policy<'a> {
        calls: &'a AtomicUsize,
    }

    struct BorrowingSaga<'a> {
        policy: Policy<'a>,
        builds: &'a AtomicUsize,
    }

    impl Saga for BorrowingSaga<'_> {
        type State = State;
        type Step = Step;
        type HandlerError = Error;

        fn definition(&self) -> Result<SagaDefinition<'_, State, Step, Error>, SagaError> {
            self.builds.fetch_add(1, Ordering::SeqCst);
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_start_step(Step::First)
                .on::<Counter>(OPENED)
                .handle(|ctx, _| {
                    self.policy.calls.fetch_add(1, Ordering::SeqCst);
                    if ctx.state().is_some_and(|state| state.closed) {
                        return Ok(());
                    }
                    ctx.set_state(State {
                        count: 1,
                        closed: true,
                    });
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                })
                .build()
                .map_err(SagaError::from)
        }
    }

    #[test]
    fn definition_borrows_di_service_once_and_state_can_block_later_inputs() {
        let calls = AtomicUsize::new(0);
        let builds = AtomicUsize::new(0);
        let saga = BorrowingSaga {
            policy: Policy { calls: &calls },
            builds: &builds,
        };
        let definition = saga.definition().unwrap();
        let input = event();
        let mut state = instance(&input);
        {
            let Some(
                SagaRoute::StartsOn { step, handler, .. }
                | SagaRoute::OnEvent { step, handler, .. },
            ) = definition.find_event_route(&input, None)
            else {
                panic!("matching route");
            };
            let mut context =
                SagaContext::new(&mut state, CausationId::from(input.event_id), *step);
            handler(&mut context, &input)
        }
        .unwrap();
        {
            let Some(
                SagaRoute::StartsOn { step, handler, .. }
                | SagaRoute::OnEvent { step, handler, .. },
            ) = definition.find_event_route(&input, None)
            else {
                panic!("matching route");
            };
            let mut context =
                SagaContext::new(&mut state, CausationId::from(input.event_id), *step);
            handler(&mut context, &input)
        }
        .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert_eq!(builds.load(Ordering::SeqCst), 1);
        assert_eq!(state.uncommitted_commands.len(), 1);
    }

    #[test]
    fn handler_errors_remain_typed() {
        let definition = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [Route::starts_on::<Counter, _>(
                OPENED,
                Step::First,
                |_, _| Err(Error::Refused),
            )],
        )
        .unwrap();
        let input = event();
        let mut saga = instance(&input);
        assert!(matches!(
            {
                let Some(
                    SagaRoute::StartsOn { step, handler, .. }
                    | SagaRoute::OnEvent { step, handler, .. },
                ) = definition.find_event_route(&input, None)
                else {
                    panic!("matching route");
                };
                let mut context =
                    SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
                handler(&mut context, &input)
            },
            Err(SagaRouteError::Handler(Error::Refused))
        ));
    }

    #[test]
    fn builder_preserves_definition_validation_error() {
        let result =
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_start_step(Step::First)
                .on::<Counter>(OPENED)
                .handle(|_, _| Ok(()))
                .add_start_step(Step::Second)
                .on::<Counter>(OPENED)
                .handle(|_, _| Ok(()))
                .build();
        assert!(matches!(
            result,
            Err(SagaDefinitionBuilderError::Definition(
                SagaDefinitionError::DuplicateEventRoute
            ))
        ));
    }

    #[test]
    fn context_errors_keep_their_source_through_route() {
        let definition =
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_start_step(Step::First)
                .on::<Counter>(OPENED)
                .handle(|ctx, _| {
                    ctx.state_required()?;
                    Ok(())
                })
                .build()
                .unwrap();
        let input = event();
        let mut saga = instance(&input);
        assert!(matches!(
            {
                let Some(
                    SagaRoute::StartsOn { step, handler, .. }
                    | SagaRoute::OnEvent { step, handler, .. },
                ) = definition.find_event_route(&input, None)
                else {
                    panic!("matching route");
                };
                let mut context =
                    SagaContext::new(&mut saga, CausationId::from(input.event_id), *step);
                handler(&mut context, &input)
            },
            Err(SagaRouteError::Handler(Error::Context(
                SagaContextError::Instance(SagaInstanceError::NoState)
            )))
        ));
    }
}
