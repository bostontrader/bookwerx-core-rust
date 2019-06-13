use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

// These are _almost_ duplicated in src/main.  How can I DRY this?
const CONN_KEY_CLI: &str = "--conn";
const CONN_KEY_ENV: &str = "BCR_CONN";

#[test]
fn conn_no_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("bookwerx-core-rust")?;
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Fatal error: No db connection string available."));

    Ok(())
}

#[test]
fn conn_no_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("bookwerx-core-rust")?;

    cmd.env(CONN_KEY_ENV,"example-constring-from-env");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Accessing the db via connection string example-constring-from-env, as set from the environment."));

    Ok(())
}

#[test]
fn conn_with_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("bookwerx-core-rust")?;

    cmd.arg(CONN_KEY_CLI)
        .arg("example-constring-from-cli");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Accessing the db via connection string example-constring-from-cli, as set from the command line."));
    Ok(())
}

#[test]
// The value set in the command line should override whatever is in the environment.
fn conn_with_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin("bookwerx-core-rust")?;

    cmd.env(CONN_KEY_ENV,"example-constring-from-env");

    cmd.arg(CONN_KEY_CLI)
        .arg("example-constring-from-cli");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Accessing the db via connection string example-constring-from-cli, as set from the command line."));
    Ok(())
}
