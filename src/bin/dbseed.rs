use bookwerx_core_rust::constants as C;

use clap::clap_app;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    // 1. Configure the CLI
    let cli_matcher = clap_app!(bookwerx_core_rust =>
        (version: "0.13.0") // Keep this in sync with TOML
        (author: "Thomas Radloff. <bostontrader@gmail.com>")
        (about: "A blind man in a dark room looking for a black cat that's not there.")
        (@arg conn: -c --conn +takes_value "Specifies a connection string to connect to the db. Ex: mysql://root:mysecretpassword@127.0.0.1:3306")
        (@arg dbname: -d --dbname +takes_value "The database name to use.")
        (@arg seed: -s --seed +takes_value "Initialize the db using the given seed file.")
    ).get_matches();


    // 2. Obtain the configuration arguments passed via command line or environment, if any.

    // 2.1 conn_value.  Must have.
    let conn_value;
    match cli_matcher.value_of(C::CONN_KEY_CLI) {
        Some(_result) => {
            println!("Accessing the db via connection string [{}], as set from the command line.", _result);
            conn_value = _result.to_string();
        }
        None =>
            match env::var(C::CONN_KEY_ENV) {
                Ok(_result) => {
                    println!("Accessing the db via connection string [{}], as set from the environment.", _result);
                    conn_value = _result;
                }

                Err(_) => {
                    println!("Fatal error: No db connection string is available.");
                    ::std::process::exit(1);
                }
            }
    }

    // 2.2 dbname_value.  Must have.
    let dbname_value;
    match cli_matcher.value_of(C::DBNAME_KEY_CLI) {
        Some(_result) => {
            println!("Using db [{}], as set from the command line.", _result);
            dbname_value = _result.to_string();
        }
        None =>
            match env::var(C::DBNAME_KEY_ENV) {
                Ok(_result) => {
                    println!("Using db [{}], as set from the environment.", _result);
                    dbname_value = _result;
                }

                Err(_) => {
                    println!("Fatal error: No db name is available.");
                    ::std::process::exit(1);
                }
            }
    }


    // 2.3 seed_value.  Must have.
    let mut seed_value = String::new();
    match cli_matcher.value_of(C::SEED_KEY_CLI) {
        Some(_result) => {
            println!("Initializing the db with seed file [{}], as set from the command line.", _result);
            seed_value = _result.to_string();
        }
        None =>
            match env::var(C::SEED_KEY_ENV) {
                Ok(_result) => {
                    println!("Initializing the db with seed file [{}], as set from the environment.", _result);
                    seed_value = _result;
                }

                Err(_) => {
                    println!("Fatal error: No seed file is available.");
                }
            }
    }


    // 3. A seed file is available so now try to read it.
    // Create a path to the desired file
    let path = Path::new(&seed_value);
    let display = path.display();

    // Open the path in read-only mode.
    let mut file = match File::open(&path) {
        Ok(file) => file,

        Err(why) => {
            println!("Couldn't open [{}]: {}", display, why.description());
            ::std::process::exit(1);
        },
    };

    // Read the file contents into a string.
    let mut seed_contents: String = String::new();
    match file.read_to_string(&mut seed_contents) {
        Ok(_) => {
            // We have successfully read the seed file.  Now connect to the db and reseed.
            match mysql::Conn::new(&conn_value) {
                Ok(mut _conn) => {
                    //println!("Connected to [{}]", conn_string);
                    // Now wipe the db and re-init.
                    match _conn.query(format!("DROP DATABASE IF EXISTS `{0}`; CREATE DATABASE `{0}`;", dbname_value)) {
                        Ok(_) => {
                            println!("drop and create success");
                        }
                        Err(_) => {
                            // How can we test this?
                            println!("drop and create fail");
                            ::std::process::exit(1);
                        }
                    };

                    _conn.select_db(&dbname_value[..]);

                    match _conn.query(&seed_contents) {
                        Ok(_) => {
                            println!("The seed has germinated.");
                        }
                        Err(_x) => {
                            println!("The seed file does not contain valid SQL.  Does not compute.");
                            ::std::process::exit(1);
                        }
                    };
                }
                Err(_err) => {
                    println!("Cannot connect to the db server {}", _err);
                    ::std::process::exit(1);
                }
            }
        }
        Err(_err) => {
            // How can we test this?
            println!("couldn't read {}: {}", display, _err.description());
            ::std::process::exit(1);
        }
    }
}