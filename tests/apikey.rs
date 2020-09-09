use bookwerx_core_rust::db::PostApikeysResponse;
use rocket::http::Status;
use rocket::local::Client;

// Get an API key
pub fn apikey(client: &Client) -> String {
    let mut response = client.post("/apikeys").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut ret_val = String::new();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        PostApikeysResponse::Apikey(key) => ret_val = key,
        _ => assert!(false),
    }
    ret_val
}
