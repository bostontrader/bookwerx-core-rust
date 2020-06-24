use bookwerx_core_rust::db as D;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Examine acctcats
pub fn acctcats(client: &Client, apikey: &String, accounts: &Vec<D::AccountJoined>, categories: &Vec<D::Category>) -> Vec<D::Acctcat> {

    // 1. GET /acctcats.
    let mut response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 2. Try to post a new acctcat.

    // 2.1. But first post using a non-existent apikey.
    response = client.post("/acctcats")
        .body(format!("apikey=notarealkey&account_id={}&category_id={}",(accounts.get(0).unwrap()).id,(categories.get(0).unwrap()).id))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 2.2 Successful post.
    response = client.post("/acctcats")
        .body(
            format!("apikey={}&account_id={}&category_id={}"
                , apikey
                , (accounts.get(0).unwrap()).id
                , (categories.get(0).unwrap()).id
            ))
        .header(ContentType::Form)
        .dispatch();
    let mut lid: u64 = 0;
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid1) => { lid = lid1; assert!(lid > 0) },
        _ => assert!(false)
    }

    // 2.3 Successful put.
    response = client.put("/acctcats")
        .body(
            format!("apikey={}&id={}&account_id={}&category_id={}"
                , apikey, lid
                , (accounts.get(0).unwrap()).id
                ,(categories.get(0).unwrap()).id

            ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Info(info) => assert_eq!(info, "(Rows matched: 1  Changed: 0  Warnings: 0"),
        _ => assert!(false)
    }

    // 3. Now verify that there's a single acctcat.
    response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false)
    }

    // 4. Try to post a acctcat with a duplicated account_id, category_id.
    response = client.post("/acctcats")
        .body(
            format!("apikey={}&account_id={}&category_id={}"
                    , apikey
                    , (accounts.get(0).unwrap()).id
                    ,(categories.get(0).unwrap()).id

            ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 5. Now submit a single GET of the prior POST.
    response = client.get(format!("/acctcat/{}/?apikey={}", lid, apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::One(_) => assert!(true),
        _ => assert!(false)
    }

    // 6. Make a 2nd Successful post.
    response = client.post("/acctcats")
        .body(
            format!("apikey={}&account_id={}&category_id={}"
                    , apikey
                    , (accounts.get(1).unwrap()).id
                    , (categories.get(0).unwrap()).id
            ))
        .header(ContentType::Form)
        .dispatch();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false)
    }

    // 7. Verify that there are now two acctcats
    response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut ret_val = Vec::new();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => {
            assert_eq!(v.len(), 2);
            ret_val = v
        },
        _ => assert!(false)
    }
    ret_val

}