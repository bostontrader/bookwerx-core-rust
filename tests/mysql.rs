use assert_cmd::prelude::*;
use bookwerx_core_rust::constants as C;
use predicates::prelude::*;
use std::process::Command;


#[test]
fn mysql_connect() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(C::CARGO_BIN)?;

    cmd.env(C::CONN_KEY_ENV,C::TEST_CONN_STR)
        .env(C::INIT_KEY_ENV, "1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("connected"));
    Ok(())
}
