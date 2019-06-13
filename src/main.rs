#[macro_use]
extern crate clap;  // CLI library
use clap::{Arg, App, SubCommand};
use std::env;
use std::io;

// These are _almost_ duplicated in tests.  How can I DRY this?
const CONN_KEY_CLI: &str = "conn";
const CONN_KEY_ENV: &str = "BCR_CONN";

fn main() {

    // 1. Configure the CLI
    let matches = clap_app!(bookwerx_core_rust =>
        (version: "0.1.0")
        (author: "Thomas Radloff. <bostontrader@gmail.com>")
        (about: "A blind man in a dark room looking for a black cat that's not there.")
        (@arg conn: -c --conn +takes_value "Specifies a connection string to connect to the db.")
        (@arg init: -i --init "Initialize the db.")
    ).get_matches();


    // 2. Obtain a connection string, if available
    match matches.value_of(CONN_KEY_CLI) {
        Some(_x) =>
            println!("Accessing the db via connection string {}, as set from the command line.", _x),
        None =>
            match env::var(CONN_KEY_ENV) {
                Ok(_x) =>
                    println!("Accessing the db via connection string {}, as set from the environment.", _x),
                Err(_) => {
                    println!("Fatal error: No db connection string available.");
                    ::std::process::exit(1);
                }
        }
    }

}

#[test]
fn test() {

}