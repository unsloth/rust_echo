use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("rust_echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("rust_echo")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut cmd = Command::cargo_bin("rust_echo")?;
    let expected = std::fs::read_to_string(expected_file)?;

    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello1.txt")
}
#[test]
fn hello2() -> TestResult {
    run(&["Hello there"], "tests/expected/hello2.txt")
}
#[test]
fn hello3() -> TestResult {
    run(&["Hello  there"], "tests/expected/hello3.txt")
}
#[test]
fn hello1n() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello1n.txt")
}
#[test]
fn hello2n() -> TestResult {
    run(&["-n", "Hello there"], "tests/expected/hello2n.txt")
}
#[test]
fn hello3n() -> TestResult {
    run(&["-n", "Hello  there"], "tests/expected/hello3n.txt")
}
