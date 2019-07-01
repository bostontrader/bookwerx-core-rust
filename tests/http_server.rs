#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use bookwerx_core_rust::routes as R;

//use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn get_index() -> Result<(), Box<dyn std::error::Error>> {

    let rocket = rocket::ignite().mount("/", routes![R::index]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.get("/");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    Ok(())
}

#[test]
fn get_accounts() -> Result<(), Box<dyn std::error::Error>> {

    let rocket = rocket::ignite().mount("/", routes![R::get_accounts]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.get("/accounts");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    Ok(())
}

#[test]
fn post_account() -> Result<(), Box<dyn std::error::Error>> {

    let rocket = rocket::ignite().mount("/", routes![R::post_account]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let req = client.post("/accounts");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    Ok(())
}
