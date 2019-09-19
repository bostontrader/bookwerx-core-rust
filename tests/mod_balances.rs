use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

pub fn accounts(client: &Client, apikey: &String, categories: &Vec<R::Category>, transactions: &Vec<R::Transaction>) -> Vec<R::BalanceResult> {

    // 1. GET /balances. sb 200, something
    response = client.get(
        format!("/balances?apikey={}&category_id={}&time={}",
            &apikey,
            (currencies.get(0).unwrap()).id,
            (transactions.get(0).unwrap()).time
        )).dispatch();
    let v: Vec<R::AccountJoined> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    v

}
