use bookwerx_core_rust::db as D;
use rocket::http::Status;
use rocket::local::Client;

pub fn linter(client: &Client, apikey: &String) {
    // 1. GET /linter/accounts.
    let mut response = client
        .get(format!("/linter/accounts?apikey={}", &apikey))
        .dispatch();
    let v: Vec<D::LinterShort> =
        serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 2. GET /linter/categories.
    response = client
        .get(format!("/linter/categories?apikey={}", &apikey))
        .dispatch();
    let v: Vec<D::LinterLong> =
        serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 3. GET /linter/currencies.
    response = client
        .get(format!("/linter/currencies?apikey={}", &apikey))
        .dispatch();
    let v: Vec<D::LinterLong> =
        serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);
}
