use bookwerx_core_rust::db as D;
use rocket::http::Status;
use rocket::local::Client;

pub fn account_dist_sum(client: &Client, apikey: &String, accounts: &Vec<D::AccountJoined>) -> () {
    // 1. GET /account_dist_sum, bad account_id, no time_*. sb 200. sb 0.
    let mut response = client.get(format!("/account_dist_sum?apikey={}&account_id=666", &apikey)).dispatch();
    let mut r: D::DFPResult = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum.amount, 0);
    assert_eq!(r.sum.exp, 0);

    // 2. Good account_id with the four permutations of time_*

    // 2.1 no time_*
    response = client.get(format!("/account_dist_sum?apikey={}&account_id={}", &apikey, (accounts.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum.amount, 12);
    assert_eq!(r.sum.exp, 0);

    // 2.2 time_start
    response = client.get(format!("/account_dist_sum?apikey={}&account_id={}&time_start=2020-12", &apikey, (accounts.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum.amount, 9);
    assert_eq!(r.sum.exp, 0);

    // 2.3 time_stop
    response = client.get(format!("/account_dist_sum?apikey={}&account_id={}&time_stop=2020-12", &apikey, (accounts.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum.amount, 7);
    assert_eq!(r.sum.exp, 0);

    // 2.4 time_start and time_stop
    response = client.get(format!("/account_dist_sum?apikey={}&account_id={}&time_start=2020-12&time_stop=2020-12", &apikey, (accounts.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum.amount, 4);
    assert_eq!(r.sum.exp, 0);
}