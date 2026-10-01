use std::fmt::Debug;

use appletheia_application::saga::{SagaStep as SagaStepTrait, SerializedSagaStep};
use appletheia_macros::{SagaStep, saga_step};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value, json};

#[test]
fn ui_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/command_pass.rs");
    t.pass("tests/ui/query_pass.rs");
    t.pass("tests/ui/aggregate_pass_default_core.rs");
    t.pass("tests/ui/aggregate_pass_core_ident.rs");
    t.pass("tests/ui/aggregate_pass_core_string.rs");
    t.pass("tests/ui/aggregate_id_pass_default.rs");
    t.pass("tests/ui/aggregate_id_pass_validate.rs");
    t.pass("tests/ui/aggregate_state_pass_default_id.rs");
    t.pass("tests/ui/aggregate_state_pass_custom_id.rs");
    t.pass("tests/ui/unique_constraints_pass.rs");
    t.pass("tests/ui/reference_indexes_pass.rs");
    t.pass("tests/ui/event_payload_pass_default_error.rs");
    t.pass("tests/ui/event_payload_pass_custom_error.rs");
}

#[saga_step]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ExampleSagaStep {
    ReserveFunds,
    Retry { attempt: u32 },
    Batch(u32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, SagaStep)]
#[serde(rename_all = "snake_case")]
enum CustomSagaStep {
    ReserveFunds,
}

#[saga_step]
#[derive(SagaStep)]
#[serde(rename_all = "snake_case")]
enum CustomAttributeSagaStep {
    ReserveFunds,
}

#[saga_step]
#[serde(tag = "type", content = "data", rename_all = "snake_case", bound = "")]
enum GenericSagaStep<T>
where
    T: Copy + Eq + Send + Sync + 'static,
    T: Serialize,
    for<'a> T: Deserialize<'a>,
{
    Value(T),
}

#[test]
fn saga_steps_preserve_adjacent_tags_and_variant_values() {
    fn round_trip<S>(step: S, expected: Value)
    where
        S: SagaStepTrait + Debug,
    {
        let serialized = SerializedSagaStep::new(step).expect("step should serialize");
        assert_eq!(serialized.value(), &expected);
        assert_eq!(
            serialized.try_to_step::<S>().expect("step should decode"),
            step
        );
    }

    round_trip(
        ExampleSagaStep::ReserveFunds,
        json!({"type": "reserve_funds"}),
    );
    round_trip(
        ExampleSagaStep::Retry { attempt: 2 },
        json!({"type": "retry", "data": {"attempt": 2}}),
    );
    round_trip(
        ExampleSagaStep::Batch(3),
        json!({"type": "batch", "data": 3}),
    );
    round_trip(
        GenericSagaStep::Value(7_u32),
        json!({"type": "value", "data": 7}),
    );
    round_trip(CustomSagaStep::ReserveFunds, json!("reserve_funds"));
    round_trip(
        CustomAttributeSagaStep::ReserveFunds,
        json!("reserve_funds"),
    );

    assert_ne!(
        ExampleSagaStep::Retry { attempt: 1 },
        ExampleSagaStep::Retry { attempt: 2 }
    );
    assert!(
        from_value::<ExampleSagaStep>(json!({"type": "retry", "data": {"attempt": "invalid"}}))
            .is_err()
    );
}
