#[test]
fn ui_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/aggregate_pass_default_core.rs");
    t.pass("tests/ui/aggregate_pass_core_ident.rs");
    t.pass("tests/ui/aggregate_pass_core_string.rs");
    t.pass("tests/ui/aggregate_id_pass_default.rs");
    t.pass("tests/ui/aggregate_id_pass_validate.rs");
    t.pass("tests/ui/aggregate_state_pass_default_id.rs");
    t.pass("tests/ui/aggregate_state_pass_custom_id.rs");
}
