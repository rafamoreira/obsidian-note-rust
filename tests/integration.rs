use assert_cmd::Command;

#[test]
fn no_args_exits_nonzero_and_prints_usage() {
    let mut cmd = Command::cargo_bin("note").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}
