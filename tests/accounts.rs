use bookwerx_core_rust::db as D;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

// Examine accounts
pub fn accounts(
    client: &Client,
    apikey: &String,
    currencies: &Vec<D::Currency>,
) -> Vec<D::AccountJoined> {
    // 1. GET /accounts.
    let mut response = client
        .get(format!("/accounts?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAccountResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false),
    }

    // 2. Try to post a new account

    // 2.1. But first post using a non-existent apikey.
    response = client
        .post("/accounts")
        .body(format!(
            "apikey=notarealkey&currency_id={}&title=Cash in mattress",
            (currencies.get(0).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false),
    }

    // 2.2 Successful post.
    response = client
        .post("/accounts")
        .body(format!(
            "apikey={}&currency_id={}&title=Cash in mattress",
            apikey,
            (currencies.get(0).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut lid: u64 = 0;
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid1) => {
            lid = lid1;
            assert!(lid > 0)
        }
        _ => assert!(false),
    }

    // 2.3 Successful put.
    response = client
        .put("/accounts")
        .body(format!(
            "apikey={}&id={}&currency_id={}&title=Cash in mattress",
            apikey,
            lid,
            (currencies.get(0).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Info(info) => assert_eq!(info, "(Rows matched: 1  Changed: 0  Warnings: 0"),
        _ => assert!(false),
    }

    // 3. Now verify that there's a single account
    response = client
        .get(format!("/accounts?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAccountResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false),
    }

    // 4. Try to post w/bad currency id
    response = client
        .post("/accounts")
        .body(format!(
            "apikey={}&currency_id=666&title=Cash in mattress",
            apikey
        ))
        .header(ContentType::Form)
        .dispatch();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false),
    }

    // 5. Now submit a single GET of the prior POST.
    response = client
        .get(format!("/account/{}/?apikey={}", lid, apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAccountResponse::One(_) => assert!(true),
        _ => assert!(false),
    }

    // 6. Make a 2nd Successful post.
    response = client
        .post("/accounts")
        .body(format!(
            "apikey={}&currency_id={}&title=Cash in cookie jar",
            apikey,
            (currencies.get(1).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 7. Make a 3rd Successful post.
    response = client
        .post("/accounts")
        .body(format!(
            "apikey={}&currency_id={}&title=Bank of Mises",
            apikey,
            (currencies.get(1).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 8. Make a 4th Successful post.  This account will not be referenced elsewhere and should be caught by the linter.
    response = client
        .post("/accounts")
        .body(format!(
            "apikey={}&currency_id={}&title=Boats n hos",
            apikey,
            (currencies.get(1).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 9. Verify that there are four accounts.
    response = client
        .get(format!("/accounts?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut ret_val = Vec::new();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAccountResponse::Many(v) => {
            assert_eq!(v.len(), 4);
            ret_val = v
        }
        _ => assert!(false),
    }
    ret_val
}
