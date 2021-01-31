use assert_cmd::prelude::*;
use bookwerx_core_rust::constants as C;
use predicates::prelude::*;
use std::process::Command;

/*
These tests should be run one at a time so be sure to set RUST_TEST_THREADS=1 when executing the tests.  For example:

RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test server_config

Test that we can provide the correct configuration via a mixture of command-line and the environment.  Other configuration is frequently needed in order to enable the server to proceed to the behavior under test.

1.1 If neither --bind_ip nor BCR_BIND_IP are specified, the server will complain and exit.  We _must have_ an IP address for the Rocket server to use or there's nothing else to do.

1.2 If either one of --bind_ip or BCR_BIND_IP are specified, the startup message will mention it.  But the server will terminate with an error because other subsequent configuration is missing.

1.3 If both --bind_ip and BCR_BIND_IP are specified, the startup message mentions the value from --bind_ip.  But the server will terminate with an error because other subsequent configuration is missing.


2.1 If neither --bind_port nor BCR_BIND_PORT are specified, the server will complain and exit.  We _must have_ a port for the Rocket server to use or there's nothing else to do.

2.2 If either one of --bind_port or BCR_BIND_PORT are specified, the startup message will mention it.  But the server will terminate with an error because other subsequent configuration is missing.

2.3 If both --bind_port and BCR_BIND_PORT are specified, the startup message mentions the value from --bind_port.  But the server will terminate with an error because other subsequent configuration is missing.


3.1 If neither --conn nor BCR_CONN are specified, the server will complain and exit.  We _must have_ a connection string or there's nothing else to do.

3.2 If either one of --conn or BCR_CONN are specified, the startup message will mention it.  But the server will terminate with an error because other subsequent configuration is missing.

3.3 If both --conn and BCR_CONN are specified, the startup message mentions the value from --conn.  But the server will terminate with an error because other subsequent configuration is missing.


4.1 If neither --dbname nor BCR_DBNAME are specified, the server will complain and exit.  We _must have_ a db name  or there's nothing else to do.

4.2 If either one of --dbname or BCR_DBNAME are specified, the startup message will mention it.  But the server will terminate with an error because other subsequent configuration is missing.

4.3 If both --dbname and BCR_DBNAME are specified, the startup message mentions the value from --dbname.  But the server will terminate with an error because other subsequent configuration is missing.


5.1 If neither --mode nor BCR_MODE are specified, the server will complain and exit.  We _must have_ an operation mode or there's nothing else to do.

5.2 If either one of --mode or BCR_MODE are specified, the startup message will mention it.  But the server will terminate with an error because other subsequent configuration is missing.

5.3 If both --mode and BCR_MODE are specified, the startup message mentions the value from --mode.  But the server will terminate with an error because other subsequent configuration is missing.
*/

const CARGO_BIN: &str = "server";
const TEST_BIND_IP: &str = "0.0.0.0";
const TEST_BIND_PORT: &str = "8888";
const TEST_CONN_STR: &str = "mysql://root:supersecretpassword@localhost:3306";
const TEST_DBNAME: &str = "bookwerx-core-rust-test";
//const TEST_MODE: &str = "test";

#[test] // 1.1
fn bind_ip_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.assert()
        .stdout(predicate::str::contains(
            "Fatal error: No binding IP address is available.",
        ))
        .failure();

    Ok(())
}

#[test] // 1.2
fn bind_ip_no_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Rocket will bind to IP address [{}], as set from the environment.",
            TEST_BIND_IP
        )))
        .failure();

    Ok(())
}

#[test] // 1.2
fn bind_ip_with_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.arg(format!("--{}", C::BIND_IP_KEY_CLI))
        .arg(TEST_BIND_IP);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Rocket will bind to IP address [{}], as set from the command line.",
            TEST_BIND_IP
        )))
        .failure();

    Ok(())
}

#[test] // 1.3
        // The value set in the command line should override whatever is in the environment.
fn bind_ip_with_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP);

    cmd.arg(format!("--{}", C::BIND_IP_KEY_CLI))
        .arg(TEST_BIND_IP);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Rocket will bind to IP address [{}], as set from the command line.",
            TEST_BIND_IP
        )))
        .failure();

    Ok(())
}

#[test] // 2.1
fn bind_port_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP);

    cmd.assert()
        .stdout(predicate::str::contains(
            "Fatal error: No binding port is available.",
        ))
        .failure();

    Ok(())
}

#[test] // 2.2
fn bind_port_no_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        // This is what we're really testing
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Rocket will bind to port [{}], as set from the environment.",
            TEST_BIND_PORT
        )))
        .failure();

    Ok(())
}

#[test] // 2.2
fn bind_port_with_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        // This is what we're really testing
        .arg(format!("--{}", C::BIND_PORT_KEY_CLI))
        .arg(TEST_BIND_PORT);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Rocket will bind to port [{}], as set from the command line.",
            TEST_BIND_PORT
        )))
        .failure();

    Ok(())
}

#[test] // 2.3
        // The value set in the command line should override whatever is in the environment.
fn bind_port_with_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        // This is what we're really testing
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        .arg(format!("--{}", C::BIND_PORT_KEY_CLI))
        .arg(TEST_BIND_PORT);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Rocket will bind to port [{}], as set from the command line.",
            TEST_BIND_PORT
        )))
        .failure();

    Ok(())
}

#[test] // 3.1
fn conn_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT);

    cmd.assert()
        .stdout(predicate::str::contains(
            "Fatal error: No db connection string is available.",
        ))
        .failure();

    Ok(())
}

#[test] // 3.2
fn conn_no_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        // This is what we're really testing
        .env(C::CONN_KEY_ENV, TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Accessing the db via connection string [{}], as set from the environment.",
            TEST_CONN_STR
        )))
        //.stdout(predicate::str::contains(format!("Fatal error: No database specified.")))
        .failure();

    Ok(())
}

#[test] // 3.2
fn conn_with_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        // This is what we're really testing
        .arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Accessing the db via connection string [{}], as set from the command line.",
            TEST_CONN_STR
        )))
        //.stdout(predicate::str::contains(format!("Fatal error: No database specified.")))
        .failure();

    Ok(())
}

#[test] // 3.3
        // The value set in the command line should override whatever is in the environment.
fn conn_with_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        // This is what we're really testing
        .env(C::CONN_KEY_ENV, TEST_CONN_STR)
        .arg(format!("--{}", C::CONN_KEY_CLI))
        .arg(TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Accessing the db via connection string [{}], as set from the command line.",
            TEST_CONN_STR
        )))
        .failure();

    Ok(())
}

#[test] // 4.1
fn dbname_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        .env(C::CONN_KEY_ENV, TEST_CONN_STR);

    cmd.assert()
        .stdout(predicate::str::contains(
            "Fatal error: No db name is available.",
        ))
        .failure();

    Ok(())
}

#[test] // 4.2
fn dbname_no_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        .env(C::CONN_KEY_ENV, TEST_CONN_STR)
        // This is what we're really testing
        .env(C::DBNAME_KEY_ENV, TEST_DBNAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Using db [{}], as set from the environment.",
            TEST_DBNAME
        )))
        .failure();

    Ok(())
}

#[test] // 4.2
fn dbname_with_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        .env(C::CONN_KEY_ENV, TEST_CONN_STR)
        // This is what we're really testing
        .arg(format!("--{}", C::DBNAME_KEY_CLI))
        .arg(TEST_DBNAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Using db [{}], as set from the command line.",
            TEST_DBNAME
        )))
        .failure();

    Ok(())
}

#[test] // 4.3
        // The value set in the command line should override whatever is in the environment.
fn dbname_with_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        .env(C::CONN_KEY_ENV, TEST_CONN_STR)
        // This is what we're really testing
        .env(C::DBNAME_KEY_ENV, "example-db-name-from-env")
        .arg(format!("--{}", C::DBNAME_KEY_CLI))
        .arg(TEST_DBNAME);

    cmd.assert()
        .stdout(predicate::str::contains(format!(
            "Using db [{}], as set from the command line.",
            TEST_DBNAME
        )))
        .failure();

    Ok(())
}

#[test] // 5.1
fn mode_no_cli_no_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;

    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV, TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV, TEST_BIND_PORT)
        .env(C::CONN_KEY_ENV, TEST_CONN_STR)
        .env(C::DBNAME_KEY_ENV, TEST_DBNAME);

    cmd.assert()
        .stdout(predicate::str::contains(
            "Fatal error: No operating mode is available.",
        ))
        .failure();

    Ok(())
}

//#[test] // 5.2 This test should enable the server to start up.  But it starts blockingly, so it never returns to this test. How can we test this?
/*fn mode_no_cli_with_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CARGO_BIN)?;
    // This is necessary to make the test proceed far enough to test what we want.
    cmd.env(C::BIND_IP_KEY_ENV,TEST_BIND_IP)
        .env(C::BIND_PORT_KEY_ENV,TEST_BIND_PORT)
        .env(C::CONN_KEY_ENV,TEST_CONN_STR)
        .env(C::DBNAME_KEY_ENV,TEST_DBNAME)
        // This is what we're really testing
        .env(C::MODE_KEY_ENV,TEST_MODE);
    cmd.assert()
        .stdout(predicate::str::contains(format!("Operating in {} mode, as set from the environment.", TEST_MODE)))
        .failure();
    Ok(())
}*/
