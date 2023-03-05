use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}


#[test]
fn die_no_args() -> TestResult {
    let mut command = Command::cargo_bin("echor")?;
    command.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut command = Command::cargo_bin("echor")?;
    command.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult{
    let outfile = "tests/expected/hello1.txt";
    run(&["Hello there"], outfile)
}

#[test]
fn hello2() -> TestResult{
    let outfile = "tests/expected/hello2.txt";
    run(&["Hello", "there"], outfile)
}
