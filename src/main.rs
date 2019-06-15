use bookwerx_core_rust::constants as C;
use clap::clap_app;
use std::env;

fn main() {

    // 1. Configure the CLI
    let matches = clap_app!(bookwerx_core_rust =>
        (version: "0.1.1") // Keep this in sync with TOML
        (author: "Thomas Radloff. <bostontrader@gmail.com>")
        (about: "A blind man in a dark room looking for a black cat that's not there.")
        (@arg conn: -c --conn +takes_value "Specifies a connection string to connect to the db. Ex: mysql://root:mysecretpassword@127.0.0.1:3306")
        (@arg init: -i --init +takes_value "Initialize the db using the given seed file.")
    ).get_matches();


    // 2. Obtain a connection string, if available
    let mut conn_string = String::new();

    match matches.value_of(C::CONN_KEY_CLI) {
        Some(_x) => {
            println!("Accessing the db via connection string {}, as set from the command line.", _x);
            conn_string = _x.to_string();
        }
        None =>
            match env::var(C::CONN_KEY_ENV) {
                Ok(_x) => {
                    println!("Accessing the db via connection string {}, as set from the environment.", _x);
                    conn_string = _x;
                }

                Err(_) => {
                    println!("Fatal error: No db connection string available.");
                    ::std::process::exit(1);
                }
        }
    }

    // 3. React to the initialization flag, if available
    if matches.is_present(C::INIT_KEY_CLI) {
        println!("Initializing the db due to configuration as set from the command line.");
    } else {
        match env::var(C::INIT_KEY_ENV) {
            Ok(_x) =>
                println!("Initializing the db due to configuration as set from the environment."),
            Err(_) => {
                // Neither the command line nor the env says anything about initialization.  Therefore don't do any initialization.
            }
        }
    }

    // 4. Now try to connect to the db
    let conn = mysql::Pool::new(conn_string );
    match conn {
        Ok(_x) => {
            println!("connected")
        },
        Err(_) => {
            println!("connection error")
        }
    }
}

#[test]
fn test() {

}