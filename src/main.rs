use bookwerx_core_rust::constants as C;
use clap::clap_app;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    // 1. Configure the CLI
    let cli_matches = clap_app!(bookwerx_core_rust =>
        (version: "0.1.2") // Keep this in sync with TOML
        (author: "Thomas Radloff. <bostontrader@gmail.com>")
        (about: "A blind man in a dark room looking for a black cat that's not there.")
        (@arg conn: -c --conn +takes_value "Specifies a connection string to connect to the db. Ex: mysql://root:mysecretpassword@127.0.0.1:3306")
        (@arg db: -d --db +takes_value "The database name to use.")
        (@arg init: -i --init +takes_value "Initialize the db using the given seed file.")
    ).get_matches();


    // 2. Obtain a connection string, if available.
    let mut conn_string;
    match cli_matches.value_of(C::CONN_KEY_CLI) {
        Some(_x) => {
            println!("Accessing the db via connection string [{}], as set from the command line.", _x);
            conn_string = _x.to_string();
        }
        None =>
            match env::var(C::CONN_KEY_ENV) {
                Ok(_x) => {
                    println!("Accessing the db via connection string [{}], as set from the environment.", _x);
                    conn_string = _x;
                }

                Err(_) => {
                    println!("Fatal error: No db connection string available.");
                    ::std::process::exit(1);
                }
            }
    }

    // 3. Obtain a db name, if available.
    let mut db_name;
    match cli_matches.value_of(C::DB_KEY_CLI) {
        Some(_x) => {
            println!("Using database [{}], as set from the command line.", _x);
            db_name = _x.to_string();
        }
        None =>
            match env::var(C::DB_KEY_ENV) {
                Ok(_x) => {
                    println!("Using database [{}], as set from the environment.", _x);
                    db_name = _x;
                }

                Err(_) => {
                    println!("Fatal error: No database specified.");
                    ::std::process::exit(1);
                }
            }
    }

    // 4. Obtain a seed_file name for the db, if available.
    let mut seed_file = String::new();
    match cli_matches.value_of(C::INIT_KEY_CLI) {
        Some(_x) => {
            println!("Initializing the db with seed file [{}], as set from the command line.", _x);
            seed_file = _x.to_string();
        }
        None =>
            match env::var(C::INIT_KEY_ENV) {
                Ok(_x) => {
                    println!("Initializing the db with seed file [{}], as set from the environment.", _x);
                    seed_file = _x;
                }

                Err(_) => {
                  // Neither the command line nor the env says anything about initialization.  Therefore don't do any initialization.
                }
        }
    }

    // 5. If a seed_file is available then try to read it.
    // The seed file should not be very large and reading it should not be any trouble.
    let mut seed = String::new();
    if !seed_file.is_empty() {
        // Create a path to the desired file
        let path = Path::new(&seed_file);
        let display = path.display();

        // Open the path in read-only mode.
        let mut file = match File::open(&path) {
            Ok(file) => file,

            Err(why) => {
                println!("Couldn't open [{}]: {}", display, why.description());
                ::std::process::exit(1);
            },
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut seed) {
            Ok(_) => {
                println!("{} contains:{}", display, seed)
            }
            Err(why) => {
                println!("couldn't read {}: {}", display, why.description());
                ::std::process::exit(1);
            }
        }
    }

    // 6. Now try to connect to the mysql server
    match mysql::Conn::new(&conn_string) {
        Ok(mut _conn) => {
            println!("Connected to [{}]", conn_string);

            // If there is a seed, wipe the db and re-init.
            if !seed.is_empty() {
                match _conn.query(format!("DROP DATABASE IF EXISTS `{0}`; CREATE DATABASE `{0}`;;", db_name)) {
                    Ok(_) => {
                        println!("drop and create success");
                    }
                    Err(_) => {
                        println!("drop and create fail");
                        ::std::process::exit(1);
                    }
                };

                _conn.select_db(&db_name[..]);

                match _conn.query(&seed) {
                    Ok(_) => {
                        println!("The seed has germinated.")
                    }
                    Err(_x) => {
                        println!("The seed file does not contain valid SQL.  Does not compute.");
                        ::std::process::exit(1);
                    }
                };
            }

            // At this point the db should be there, newly created if need be.
            if _conn.select_db(&db_name[..]) {
                println!("USE database [{}] success.", &db_name);
            } else {
                println!("Fatal error: Unknown database [{}].", &db_name);
                ::std::process::exit(1);
            }


        }
        Err(_err) => {
            println!("{}", _err);
            ::std::process::exit(1);
        }


    }

}


#[test]
fn test() {

}