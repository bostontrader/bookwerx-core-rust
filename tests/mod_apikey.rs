use bookwerx_core_rust::routes as R;

use rocket::http::Status;
use rocket::local::Client;

// Get an API key
pub fn apikey(client: &Client) -> String {

    let mut response = client.post("/apikeys").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let ak: R::Apikey = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    ak.apikey
}
