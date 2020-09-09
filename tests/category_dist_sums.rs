use bookwerx_core_rust::db as D;
use rocket::http::Status;
use rocket::local::Client;

pub fn category_dist_sums(client: &Client, apikey: &String, categories: &Vec<D::Category>) -> () {
    // 1. GET /category_dist_sums, bad category_id, no time_*.
    let mut response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id=666",
            &apikey
        ))
        .dispatch();
    let mut r: D::Sums = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 0);

    // 2. GET /category_dist_sums, get a single account (Cash in mattress) tagged by all of two categories (Assets, Specific customer) Examine the four permutations of time.

    // 2.1 No time_*.
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={},{}",
            &apikey,
            (categories.get(0).unwrap()).id,
            (categories.get(3).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 1);
    assert_eq!(r.sums[0].sum.amount, 12);
    assert_eq!(r.sums[0].sum.exp, 0);

    // 2.2 time_start.
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={},{}&time_start=2020-12",
            &apikey,
            (categories.get(0).unwrap()).id,
            (categories.get(3).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 1);
    assert_eq!(r.sums[0].sum.amount, 9);
    assert_eq!(r.sums[0].sum.exp, 0);

    // 2.3 time_stop
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={},{}&time_stop=2020-12",
            &apikey,
            (categories.get(0).unwrap()).id,
            (categories.get(3).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 1);
    assert_eq!(r.sums[0].sum.amount, 7);
    assert_eq!(r.sums[0].sum.exp, 0);

    // 2.4 time_start and time_stop.
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={},{}&time_start=2020-12&time_stop=2020-12",
            &apikey,
            (categories.get(0).unwrap()).id,
            (categories.get(3).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 1);
    assert_eq!(r.sums[0].sum.amount, 4);
    assert_eq!(r.sums[0].sum.exp, 0);

    // 3. GET /category_dist_sums, get two accounts (Cash in mattress, Cash in cookie jar) tagged with a single category (Assets)
    //  Examine the four permutations of time.

    // 3.1 No time_*.
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={}",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);
    assert_eq!(r.sums[0].sum.amount.abs(), 12);
    assert_eq!(r.sums[0].sum.amount + r.sums[1].sum.amount, 0);
    assert_eq!(r.sums[0].sum.exp, 0);
    assert_eq!(r.sums[1].sum.exp, 0);

    // 3.2 time_start.
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={}&time_start=2020-12",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);
    assert_eq!(r.sums[0].sum.amount.abs(), 9);
    assert_eq!(r.sums[0].sum.amount + r.sums[1].sum.amount, 0);
    assert_eq!(r.sums[0].sum.exp, 0);
    assert_eq!(r.sums[1].sum.exp, 0);

    // 3.3 time_stop
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={}&time_stop=2020-12",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);
    assert_eq!(r.sums[0].sum.amount.abs(), 7);
    assert_eq!(r.sums[0].sum.amount + r.sums[1].sum.amount, 0);
    assert_eq!(r.sums[0].sum.exp, 0);
    assert_eq!(r.sums[1].sum.exp, 0);

    // 3.4 time_start and time_stop.
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={}&time_start=2020-12&time_stop=2020-12",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    r = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);
    assert_eq!(r.sums[0].sum.amount.abs(), 4);
    assert_eq!(r.sums[0].sum.amount + r.sums[1].sum.amount, 0);
    assert_eq!(r.sums[0].sum.exp, 0);
    assert_eq!(r.sums[1].sum.exp, 0);

    // 3. Decorations

    // 3.1 Unparsable decorate
    //response = client.get(format!("/category_dist_sums?apikey={}&category_id={}&decorate=unparsable", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    //let r: D::ApiResponse = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    //assert_eq!(response.status(), Status::Ok);
    //assert_eq!(r.json.as_str().unwrap(), String::from("catfood"));

    // 3.2 decorate = explicit false.  Means no decoration.
    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={}&decorate=false",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    let r: D::Sums = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);

    response = client
        .get(format!(
            "/category_dist_sums?apikey={}&category_id={}&decorate=true",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    let r: D::SumsDecorated = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(r.sums.len(), 2);
}
