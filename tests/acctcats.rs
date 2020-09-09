use bookwerx_core_rust::db as D;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

// Examine acctcats
pub fn acctcats(
    client: &Client,
    apikey: &String,
    accounts: &Vec<D::AccountJoined>,
    categories: &Vec<D::Category>,
) -> Vec<D::Acctcat> {
    // 1. GET /acctcats.
    let mut response = client
        .get(format!(
            "/acctcats/for_category?apikey={}&category_id={}",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false),
    }

    // 2. Try to post a new acctcat.

    // 2.1. But first post using a non-existent apikey.
    response = client
        .post("/acctcats")
        .body(format!(
            "apikey=notarealkey&account_id={}&category_id={}",
            (accounts.get(0).unwrap()).id,
            (categories.get(0).unwrap()).id
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
        .post("/acctcats")
        .body(format!(
            "apikey={}&account_id={}&category_id={}",
            apikey,
            (accounts.get(0).unwrap()).id,   // cash in mattress
            (categories.get(0).unwrap()).id  // assets
        ))
        .header(ContentType::Form)
        .dispatch();
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
        .put("/acctcats")
        .body(format!(
            "apikey={}&id={}&account_id={}&category_id={}",
            apikey,
            lid,
            (accounts.get(0).unwrap()).id,
            (categories.get(0).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Info(info) => assert_eq!(info, "(Rows matched: 1  Changed: 0  Warnings: 0"),
        _ => assert!(false),
    }

    // 3. Now verify that there's a single acctcat.
    response = client
        .get(format!(
            "/acctcats/for_category?apikey={}&category_id={}",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false),
    }

    // 4. Try to post a acctcat with a duplicated account_id, category_id.
    response = client
        .post("/acctcats")
        .body(format!(
            "apikey={}&account_id={}&category_id={}",
            apikey,
            (accounts.get(0).unwrap()).id,
            (categories.get(0).unwrap()).id
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false),
    }

    // 5. Now submit a single GET of the prior POST.
    response = client
        .get(format!("/acctcat/{}/?apikey={}", lid, apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::One(_) => assert!(true),
        _ => assert!(false),
    }

    // 6. Make a 2nd Successful post.
    response = client
        .post("/acctcats")
        .body(format!(
            "apikey={}&account_id={}&category_id={}",
            apikey,
            (accounts.get(0).unwrap()).id,   // cash in mattress
            (categories.get(3).unwrap()).id  // specific customer
        ))
        .header(ContentType::Form)
        .dispatch();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 7. Make a 3rd Successful post.
    response = client
        .post("/acctcats")
        .body(format!(
            "apikey={}&account_id={}&category_id={}",
            apikey,
            (accounts.get(1).unwrap()).id,   // cash in cookie jar
            (categories.get(0).unwrap()).id  // assets
        ))
        .header(ContentType::Form)
        .dispatch();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 8. Make a 4th Successful post.
    response = client
        .post("/acctcats")
        .body(format!(
            "apikey={}&account_id={}&category_id={}",
            apikey,
            (accounts.get(2).unwrap()).id,   // bank of mises
            (categories.get(1).unwrap()).id  // liabilities
        ))
        .header(ContentType::Form)
        .dispatch();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 9. Verify that there are now four acctcats.  Unfortunately, because acctcat doesn't give us the ability to retrieve all of them at once, we must retrieve them via the various categories instead and combine the results.

    // 9.1 First init the retval
    let mut ret_val = Vec::new();

    // 9.2 Get the accts for category A
    response = client
        .get(format!(
            "/acctcats/for_category?apikey={}&category_id={}",
            &apikey,
            (categories.get(0).unwrap()).id
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => {
            assert_eq!(v.len(), 2);
            ret_val.extend(v)
        }
        _ => assert!(false),
    }

    // 9.3 Get the accts for category L
    response = client
        .get(format!(
            "/acctcats/for_category?apikey={}&category_id={}",
            &apikey,
            (categories.get(1).unwrap()).id
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => {
            assert_eq!(v.len(), 1);
            ret_val.extend(v)
        }
        _ => assert!(false),
    }

    // 9.4 Get the accts for category Eq
    response = client
        .get(format!(
            "/acctcats/for_category?apikey={}&category_id={}",
            &apikey,
            (categories.get(2).unwrap()).id
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => {
            assert_eq!(v.len(), 0);
            ret_val.extend(v)
        }
        _ => assert!(false),
    }

    // 9.5 Get the accts for category C
    response = client
        .get(format!(
            "/acctcats/for_category?apikey={}&category_id={}",
            &apikey,
            (categories.get(3).unwrap()).id
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => {
            assert_eq!(v.len(), 1);
            ret_val.extend(v)
        }
        _ => assert!(false),
    }

    ret_val
}
