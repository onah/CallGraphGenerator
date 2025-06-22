use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("call-graph-generator").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("call-graph-generator"))
        .stdout(predicate::str::contains("A call graph generator tool"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("call-graph-generator").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("call-graph-generator"));
}
