#[test]
fn ui_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/aggregate_pass_default_core.rs");
    t.pass("tests/ui/aggregate_pass_core_ident.rs");
    t.pass("tests/ui/aggregate_pass_core_string.rs");
}
