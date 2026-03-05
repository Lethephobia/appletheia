use std::path::PathBuf;
use std::process::Command;

fn run_cargo_check(manifest_path: PathBuf) {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());

    let output = Command::new(cargo)
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(manifest_path)
        .output()
        .expect("failed to run cargo");

    if !output.status.success() {
        panic!(
            "cargo check failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn aggregate_fixtures_compile() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");

    run_cargo_check(
        root.join("aggregate_via_appletheia_alias")
            .join("Cargo.toml"),
    );
    run_cargo_check(root.join("aggregate_via_domain_alias").join("Cargo.toml"));
}
