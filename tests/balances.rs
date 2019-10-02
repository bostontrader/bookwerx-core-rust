use bookwerx_core_rust::db as D;
use rocket::local::Client;
//use rocket::http::ContentType;
use rocket::http::Status;

pub fn balances(client: &Client, apikey: &String, categories: &Vec<D::Category>, transactions: &Vec<D::Transaction>) -> Vec<D::BalanceResult> {

    // 1. GET /balances. sb 200, something
    let mut response = client.get(
        format!("/balance?apikey={}&category_id={}&time={}",
            &apikey,
            (categories.get(0).unwrap()).id,
            (transactions.get(0).unwrap()).time
        )).dispatch();
    let v: Vec<D::BalanceResult> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    v

}
