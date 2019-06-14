use assert_cmd::prelude::*;
use bookwerx_core_rust::constants as C;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn conn_no_cli_no_env() -> Result<(), Box<std::error::Error>> {

    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Fatal error: No db connection string available."));

    Ok(())
}

#[test]
fn conn_no_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Accessing the db via connection string mysql://root:supersecretpassword@127.0.0.1:3306, as set from the environment."));

    Ok(())
}

#[test]
fn conn_with_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(C::TEST_CONN_STR);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Accessing the db via connection string mysql://root:supersecretpassword@127.0.0.1:3306, as set from the command line."));
    Ok(())
}

#[test]
// The value set in the command line should override whatever is in the environment.
fn conn_with_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,"example-constring-from-env");

    cmd.arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(C::TEST_CONN_STR);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Accessing the db via connection string mysql://root:supersecretpassword@127.0.0.1:3306, as set from the command line."));
    Ok(())
}


#[test]
fn init_no_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("initializing").count(0));
    Ok(())
}

#[test]
fn init_no_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::INIT_KEY_ENV,"1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initializing the db due to configuration as set from the environment."));
    Ok(())
}

#[test]
fn init_with_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .arg(format!("--{}", C::INIT_KEY_CLI));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initializing the db due to configuration as set from the command line."));
    Ok(())
}