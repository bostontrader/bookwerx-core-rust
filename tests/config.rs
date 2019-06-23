use assert_cmd::prelude::*;
use bookwerx_core_rust::constants as C;
use predicates::prelude::*;
use std::process::Command;

#[test] // 1.1
fn conn_no_cli_no_env() -> Result<(), Box<std::error::Error>> {

    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;
    cmd.assert()
        .stdout(predicate::str::contains("Fatal error: No db connection string available."))
        .failure();

    Ok(())
}

#[test] // 1.2
fn conn_no_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Accessing the db via connection string [{}], as set from the environment.", C::TEST_CONN_STR)))
        .stdout(predicate::str::contains(format!("Fatal error: No database specified.")))
        .failure();

    Ok(())
}

#[test] // 1.2
fn conn_with_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(C::TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Accessing the db via connection string [{}], as set from the command line.", C::TEST_CONN_STR)))
        .stdout(predicate::str::contains(format!("Fatal error: No database specified.")))
        .failure();

    Ok(())
}

#[test] // 1.3
// The value set in the command line should override whatever is in the environment.
fn conn_with_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR);

    cmd.arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(C::TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Accessing the db via connection string [{}], as set from the command line.", C::TEST_CONN_STR)))
        .stdout(predicate::str::contains(format!("Fatal error: No database specified.")))
        .failure();

    Ok(())
}

#[test] // 2.1
fn db_no_cli_no_env() -> Result<(), Box<std::error::Error>> {

    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains("Fatal error: No database specified."))
        .failure();

    Ok(())
}

#[test] // 2.2
fn db_no_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)

        // This is what we're really testing
        .env(C::DB_KEY_ENV,C::TEST_DB_NAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Using database [{}], as set from the environment.", C::TEST_DB_NAME)))
        .stdout(predicate::str::contains(format!("Connected to [{}]", C::TEST_CONN_STR)))
        .stdout(predicate::str::contains(format!("Fatal error: Unknown database [{}].", C::TEST_DB_NAME)))
        .failure();

    Ok(())
}

#[test] // 2.2
fn db_with_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)

        // This is what we're really testing
        .arg(format!("--{}", C::DB_KEY_CLI))
        .arg(C::TEST_DB_NAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Fatal error: Unknown database [{}].", C::TEST_DB_NAME)))
        .failure();

    Ok(())
}

#[test] // 2.3
// The value set in the command line should override whatever is in the environment.
fn db_with_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)

        // This is what we're really testing
        .env(C::DB_KEY_ENV,"example-db-name-from-env")

        .arg(format!("--{}", C::DB_KEY_CLI))
        .arg(C::TEST_DB_NAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Fatal error: Unknown database [{}].", C::TEST_DB_NAME)))
        .failure();

    Ok(())
}

#[test] // 3.1
fn init_no_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
    .env(C::DB_KEY_ENV,C::TEST_DB_NAME);

    // This is what we're really testing
    // Testing nada!

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("initializing").count(0)); // we don't see this
    Ok(())
}



// 3.2

#[test] // 3.2.1
fn init_with_nonexistent_seed_file_via_cli() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::DB_KEY_ENV,C::TEST_DB_NAME)

        // This is what we're really testing
        .arg(format!("--{}", C::INIT_KEY_CLI))
        .arg("no such seed file");

    cmd.assert()
        .stdout(predicate::str::contains(format!("Initializing the db with seed file [{}], as set from the command line.", "no such seed file")))
        .stdout(predicate::str::contains(format!("Couldn't open [{}]: entity not found", "no such seed file")))
        .failure();

    Ok(())
}

#[test] // 3.2.2
fn init_with_invalid_seed_file_via_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::DB_KEY_ENV,C::TEST_DB_NAME)

        // This is what we're really testing
        .env(C::INIT_KEY_ENV,C::INVALID_SEED_FILE);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Initializing the db with seed file [{}], as set from the environment.", C::INVALID_SEED_FILE)))
        .stdout(predicate::str::contains(format!("The seed file does not contain valid SQL.")))
        .failure();

    Ok(())
}

#[test] // 3.2.3
fn init_with_valid_seed_file_cli_override_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::DB_KEY_ENV,C::TEST_DB_NAME)

        // This is what we're really testing...
        .env(C::INIT_KEY_ENV,C::INVALID_SEED_FILE)

        // ... the CLI should override the env
        .arg(format!("--{}", C::INIT_KEY_CLI))
        .arg(C::MYSQL_SEED_FILE);

    cmd.assert()
        .stdout(predicate::str::contains(format!("The seed has germinated.")))
        .success();

    Ok(())
}

