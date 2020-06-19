use bookwerx_core_rust::db as D;
use bookwerx_core_rust::dfp::DFP;
use rocket::http::Status;
use rocket::local::Client;
use serde::Deserialize;

// 1. Define some useful structs. These are tedious little things needed only here.
#[derive(Deserialize)]
struct BalanceResult {
    pub account_id: u32,
    pub amount: i64,
    pub amount_exp: i8,
}

#[derive(Deserialize)]
struct AccountCurrency {
    pub account_id: u32,
    pub title: String,
    pub currency: CurrencySymbol
}

#[derive(Deserialize)]
struct CurrencySymbol {
    pub currency_id: u32,
    pub symbol: String
}

#[derive(Deserialize)]
struct BalanceResultDecorated {
    pub account: AccountCurrency,
    pub sum: DFP
}

struct Decorations {
    account_id: u32,
    title: String,
    currency_id: u32,
    symbol: String,
}

#[derive(Deserialize)]
struct AcctSum {
    account_id: u32,
    sum: DFP,
}

#[derive(Deserialize)]
struct Sums {
    sums: Vec<AcctSum>
}

#[derive(Deserialize)]
struct SumsDecorated {
    sums: Vec<BalanceResultDecorated>
}

pub fn category_dist_sums(client: &Client, apikey: &String, categories: &Vec<D::Category>) -> () {
    // 1. GET /category_dist_sums, bad category_id, no time_*. sb 200. sb empty array.
    let mut response = client.get(format!("/category_dist_sums?apikey={}&category_id=666", &apikey)).dispatch();
    let mut r: Sums = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 0);

    // 2. Good category_id and the four permutations of time_*

    // 2.1 no time_*
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    //assert_eq!(r.sum.amount, 0);
    //assert_eq!(r.sum.exp, 0);

    // 2.2 time_start
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&time_start=2020", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    //assert_eq!(r.sum.amount, 0);
    //assert_eq!(r.sum.exp, 0);

    // 2.3 time_stop
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&time_stop=2020", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    //assert_eq!(r.sum.amount, 0);
    //assert_eq!(r.sum.exp, 0);

    // 2.4 time_start and time_stop
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&time_start=2020&time_stop=2021", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    //assert_eq!(r.sum.amount, 0);
    //assert_eq!(r.sum.exp, 0);

    // 3. Decorations
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&decorate=true", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    let mut r: SumsDecorated = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    //assert_eq!(r.sum.amount, 0);
    //assert_eq!(r.sum.exp, 0);
}