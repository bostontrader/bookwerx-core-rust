use bookwerx_core_rust::db as D;
use rocket::http::Status;
use rocket::local::Client;

pub fn category_dist_sums(client: &Client, apikey: &String, accounts: &Vec<D::AccountJoined>, categories: &Vec<D::Category>) -> () {
    // 1. GET /category_dist_sums, bad category_id, no time_*. sb 200. sb empty array.
    let mut response = client.get(format!("/category_dist_sums?apikey={}&category_id=666", &apikey)).dispatch();
    let mut r: D::Sums = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 0);

    // 2. Good category_id and the four permutations of time_*

    // 2.1 no time_*
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);

    // There should be two AcctSum records returned.  Verify this and find the particular one that we want
    assert_eq!(r.sums.len(), 2);
    let account_id1: u32 = (*accounts.get(0).unwrap()).id;
    for a in r.sums {
        if a.account_id == account_id1 {
            assert_eq!(a.sum.amount, 12);
            assert_eq!(a.sum.exp, 0);
        }
    }

    // 2.2 time_start
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&time_start=2020-12", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 1);
    let a = r.sums.get(0).unwrap();
    assert_eq!(a.sum.amount, 9);
    assert_eq!(a.sum.exp, 0);

    // 2.3 time_stop
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&time_stop=2020-12", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);

    // There should be two AcctSum records returned.  Verify this and find the particular one that we want
    assert_eq!(r.sums.len(), 2);
    let account_id1: u32 = (*accounts.get(0).unwrap()).id;
    for a in r.sums {
        if a.account_id == account_id1 {
            assert_eq!(a.sum.amount, 7);
            assert_eq!(a.sum.exp, 0);
        }
    }

    // 2.4 time_start and time_stop
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&time_start=2020-12&time_stop=2020-12", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 1);
    let a = r.sums.get(0).unwrap();
    assert_eq!(a.sum.amount, 4);
    assert_eq!(a.sum.exp, 0);

    // 3. Decorations

    // 3.1 Unparsable decorate
    //response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&decorate=unparsable", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    //let r: D::ApiResponse = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    //assert_eq!(response.status(), Status::Ok);
    //assert_eq!(r.json.as_str().unwrap(), String::from("catfood"));

    // 3.2 decorate = explicit false.  Means no decoration.
    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&decorate=false", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    let r: D::Sums = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);

    response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&decorate=true", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    let r: D::SumsDecorated = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);
}