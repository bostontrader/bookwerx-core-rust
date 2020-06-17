use assert_cmd::prelude::*;
use bookwerx_core_rust::constants as C;
use predicates::prelude::*;
use std::process::Command;

/*
These tests should be run one at a time so be sure to set RUST_TEST_THREADS=1 when executing the tests.  For example:

RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test dbseed

Some of these tests require access to a suitably configured mysql db.


1.1 If neither --conn nor BCR_CONN are specified, the server will complain and exit.  We _must have_ a connection string or there's nothing else to do.

1.2 If either one of --conn or BCR_CONN are specified, the startup message will mention it.  But the server will terminate with an error because other subsequent configuration is missing.

1.3 If both --conn and BCR_CONN are specified, the startup message mentions the value from --conn.  But the server will terminate with an error because other subsequent configuration is missing.


2.1 If neither --dbname nor BCR_DBNAME are specified, the server will complain and exit.  We _must have_ a db name or there's nothing else to do.

2.2 If either one of --dbname or BCR_DBNAME are specified, the startup message will mention it.  But the server will terminate with an error because other subsequent configuration is missing.

2.3 If both --dbname and BCR_DBNAME are specified, the startup message mentions the value from --dbname.  But the server will terminate with an error because other subsequent configuration is missing.


Now that we know that a connection string and a database name can be specified, it's time to do the same for specifying the seed file.  In addition to testing the presence or absence of the seed file name, we must also test whether the seed file points to a non-existent file, a file that contains nonsense, and a file that can be used successfully.

3.1 If neither --init nor BCR_INIT are specified, the server will complain and exit.  We _must have_ a seed file name or there's nothing else to do.

3.2 If either one of --init or BCR_INIT are specified...

Use these variations to test seed file errors...

3.2.1 If --seed, as configured via the command line is present, the startup message will mention it.  But point this to a non-existent file and the server will shut down in error.

3.2.2 If BCR_SEED, as configured via the environment is present, the startup message will mention it.  But point this to a file that contains nonsense and the server will shutdown in error.

3.3 If both --seed and BCR_SEED are specified, the startup message will mention the value from --seed.  Point this to a file that contains valid SQL and the server will shutdown successfully.

Finally, a few random edge cases to smoke out some obscure error messages.

*/

const CARGO_BIN: &str = "dbseed";
const TEST_CONN_STR: &str = "mysql://root:supersecretpassword@172.17.0.2:3306";
const TEST_DBNAME: &str = "bookwerx-core-rust-test";
const VALID_SEED_FILE: &str = "dbseed.sql";
const INVALID_SEED_FILE: &str = "tests/invalid-seed.sql";


#[test] // 1.1
fn conn_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.assert()
        .stdout(predicate::str::contains("Fatal error: No db connection string is available."))
        .failure();

    Ok(())
}


#[test] // 1.2
fn conn_no_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Accessing the db via connection string [{}], as set from the environment.", TEST_CONN_STR)))
        .failure();

    Ok(())
}


#[test] // 1.2
fn conn_with_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Accessing the db via connection string [{}], as set from the command line.", TEST_CONN_STR)))
        .failure();

    Ok(())
}


#[test] // 1.3
// The value set in the command line should override whatever is in the environment.
fn conn_with_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)
        .arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Accessing the db via connection string [{}], as set from the command line.", TEST_CONN_STR)))
        .failure();

    Ok(())
}


#[test] // 2.1
fn dbname_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains("Fatal error: No db name is available."))
        .failure();

    Ok(())
}


#[test] // 2.2
fn dbname_no_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)

        // This is what we're really testing
        .env(C::DBNAME_KEY_ENV,TEST_DBNAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Using db [{}], as set from the environment.", TEST_DBNAME)))
        .failure();

    Ok(())
}


#[test] // 2.2
fn dbname_with_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)

        // This is what we're really testing
        .arg(format!("--{}", C::DBNAME_KEY_CLI))
        .arg(TEST_DBNAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Using db [{}], as set from the command line.", TEST_DBNAME)))
        .failure();

    Ok(())
}


#[test] // 2.3
// The value set in the command line should override whatever is in the environment.
fn dbname_with_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)

        // This is what we're really testing
        .env(C::DBNAME_KEY_ENV,"example-db-name-from-env")

        .arg(format!("--{}", C::DBNAME_KEY_CLI))
        .arg(TEST_DBNAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!("Using db [{}], as set from the command line.", TEST_DBNAME)))
        .failure();

    Ok(())
}


#[test] // 3.1
fn seed_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)
        .env(C::DBNAME_KEY_ENV,TEST_DBNAME);

    // This is what we're really testing
    // Testing nada!

    cmd.assert()
        .stdout(predicate::str::contains(format!("Fatal error: No seed file is available.")))
        .failure();
    Ok(())
}


#[test] // 3.2.1
fn seed_with_nonexistent_seed_file_via_cli() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)
        .env(C::DBNAME_KEY_ENV,TEST_DBNAME)

         // This is what we're really testing
        .arg(format!("--{}", C::SEED_KEY_CLI))
        .arg("bullshit-seedfile-name");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(format!("Initializing the db with seed file [{}], as set from the command line.", "bullshit-seedfile-name")))
        .stdout(predicate::str::contains(format!("Couldn't open [{}]:", "bullshit-seedfile-name")));

    Ok(())
}


#[test] // 3.2.2
fn seed_with_invalid_seed_file_via_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)
        .env(C::DBNAME_KEY_ENV,TEST_DBNAME)

        // This is what we're really testing
        .env(C::SEED_KEY_ENV,INVALID_SEED_FILE);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(format!("Initializing the db with seed file [{}], as set from the environment.", INVALID_SEED_FILE)))
        .stdout(predicate::str::contains(format!("The seed file does not contain valid SQL.")));

    Ok(())
}


#[test] // 3.2.3
fn seed_with_valid_seed_file_cli_override_env() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,TEST_CONN_STR)
        .env(C::DBNAME_KEY_ENV,TEST_DBNAME)

        // This is what we're really testing...
        .env(C::SEED_KEY_ENV,INVALID_SEED_FILE)

        // ... the CLI should override the env
        .arg(format!("--{}", C::SEED_KEY_CLI))
        .arg(VALID_SEED_FILE);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("The seed has germinated.")));

    Ok(())
}


#[test] // 4.1
fn invalid_conn() -> Result<(), Box<dyn std::error::Error>> {

    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::CONN_KEY_ENV,"mysql://root:wrongpassword@172.17.0.2:3306")
        .env(C::DBNAME_KEY_ENV,TEST_DBNAME)
        .env(C::SEED_KEY_ENV,VALID_SEED_FILE);

    cmd.assert()
        .stdout(predicate::str::contains("Cannot connect to the db server"))
        .failure();

    Ok(())
}
