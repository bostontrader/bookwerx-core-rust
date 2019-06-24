use assert_cmd::prelude::*;
use bookwerx_core_rust::constants as C;
use predicates::prelude::*;
use std::process::Command;

/*
These tests should be run one at a time so be sure to set RUST_TEST_THREADS=1 when executing the tests.

These tests require access to a suitably configured mysql db.

We cannot reasonably control the order of execution of these tests and each test can be affected by the state of the db, as left by the prior test.  Although it's tempting to require tests to cleanup the db after their use, said process can fail and result in subsequent hard-to-diagnose issues with other tests.  It's better to just brainwipe the db before each test.
*/

// Prepare the battlefield for the test.
fn establishInitialConditions() {

    match mysql::Conn::new(C::TEST_CONN_STR) {
        Ok(mut _conn) => {

            match _conn.query(format!("DROP DATABASE IF EXISTS `{0}`;", C::TEST_DB_NAME)) {
                Ok(_) => {
                    //println!("drop and create success");
                }
                Err(_err) => {
                    println!("{}", _err);
                    ::std::process::exit(1);
                }
            };
        }
        Err(_err) => {
            println!("{}", _err);
            ::std::process::exit(1);
        }
    };
}


#[test] // 1.1
fn conn_no_cli_no_env() -> Result<(), Box<std::error::Error>> {

    establishInitialConditions();

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
        .failure();

    Ok(())
}

#[test] // 4.1
fn bind_no_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::DB_KEY_ENV,C::TEST_DB_NAME)
        .env(C::INIT_KEY_ENV,C::MYSQL_SEED_FILE);

    // This is what we're really testing
    // Testing nada!

    cmd.assert()
        .stdout(predicate::str::contains("Fatal error: No http binding configuration available."))
        .failure();
    Ok(())
}

#[test] // 4.2
fn bind_no_cli_with_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::DB_KEY_ENV,C::TEST_DB_NAME)
        .env(C::INIT_KEY_ENV,C::MYSQL_SEED_FILE)

    // This is what we're really testing
        .env(C::BIND_KEY_ENV,C::TEST_BIND);

    cmd.assert()
        .stdout(predicate::str::contains(format!("The HTTP server will bind to [{}], as specified in the environment.", C::TEST_BIND)))
        .stdout(predicate::str::contains(format!("Bind failure")))
        .failure();

    Ok(())
}

#[test] // 4.2
fn bind_with_cli_no_env() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::DB_KEY_ENV,C::TEST_DB_NAME)
        .env(C::INIT_KEY_ENV,C::MYSQL_SEED_FILE)

        // This is what we're really testing
        .arg(format!("--{}", C::BIND_KEY_CLI))
        .arg(C::TEST_BIND);

    cmd.assert()
        .stdout(predicate::str::contains(format!("The HTTP server will bind to [{}], as specified from the command line.", C::TEST_BIND)))
        .stdout(predicate::str::contains(format!("Bind failure")))
        .failure();

    Ok(())
}
