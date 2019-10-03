use bookwerx_core_rust::db as D;

use rocket::local::Client;
use rocket::http::Status;

pub fn linter(client: &Client, apikey: &String)  {

    // 1. GET /linter/categories. sb 200, one item
    let mut response = client.get(format!("/linter/categories?apikey={}", &apikey))
        .dispatch();
    let v: Vec<D::Linter> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    
}
