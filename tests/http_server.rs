#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use bookwerx_core_rust::constants as C;
use bookwerx_core_rust::routes as R;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;


//use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[test] // 1.1
fn server() -> Result<(), Box<std::error::Error>> {

    let rocket = rocket::ignite().mount("/", routes![index]);



    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.get("/");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    //assert_eq!(response.content_type(), Some(ContentType::Plain));
    //assert!(response.headers().get_one("X-Special").is_some());
    //assert_eq!(response.body_string(), Some("Expected Body.".into()));
    Ok(())
}

