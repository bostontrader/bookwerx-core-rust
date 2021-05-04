use bookwerx_core_rust::db as D;
use rocket::http::Status;
use rocket::local::Client;
use bookwerx_core_rust::dfp::dfp::{DFP, Sign};

pub fn account_dist_sum(client: &Client, apikey: &String, accounts: &Vec<D::AccountJoined>) -> () {
    // 1. GET /account_dist_sum, bad account_id, no time_*. sb 200. sb 0.
    let mut response = client
        .get(format!(
            "/account_dist_sum?apikey={}&account_id=666",
            &apikey
        ))
        .dispatch();
    let mut r: D::DFPResult = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum, DFP { amount: vec![], exp: 0, sign: Sign::Zero });

    // 2. Good account_id with the four permutations of time_*

    // 2.1 no time_*
    response = client
        .get(format!(
            "/account_dist_sum?apikey={}&account_id={}",
            &apikey,
            (accounts.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum, DFP { amount: vec![2, 1], exp: 0, sign: Sign::Positive });

    // 2.2 time_start
    response = client
        .get(format!(
            "/account_dist_sum?apikey={}&account_id={}&time_start=2020-12",
            &apikey,
            (accounts.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum, DFP { amount: vec![9], exp: 0, sign: Sign::Positive });

    // 2.3 time_stop
    response = client
        .get(format!(
            "/account_dist_sum?apikey={}&account_id={}&time_stop=2020-12",
            &apikey,
            (accounts.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum, DFP { amount: vec![7], exp: 0, sign: Sign::Positive });

    // 2.4 time_start and time_stop
    response = client
        .get(format!(
            "/account_dist_sum?apikey={}&account_id={}&time_start=2020-12&time_stop=2020-12",
            &apikey,
            (accounts.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sum, DFP { amount: vec![4], exp: 0, sign: Sign::Positive });
}
