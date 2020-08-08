use bookwerx_core_rust::db as D;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

pub fn transactions(client: &Client, apikey: &String) -> Vec<D::Transaction> {

    // 1. GET /transactions.
    let mut response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetTransactionResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 2. Try to post a new transaction

    // 2.1. But first post using a non-existent apikey.
    response = client.post("/transactions")
        .body(format!("apikey=notarealkey&notes=notes&time=2020"))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }


    // 2.2 Successful post.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=2020", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut lid: u64 = 0;
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid1) => { lid = lid1; assert!(lid > 0) },
        _ => assert!(false)
    }

    // 2.3 Successful put.
    response = client.put("/transactions")
        .body(format!("apikey={}&id={}&notes=notes&time=2020", apikey, lid))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Info(info) => assert_eq!(info, "(Rows matched: 1  Changed: 0  Warnings: 0"),
        _ => assert!(false)
    }

    // 3. Now verify that there's a single transaction
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetTransactionResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false)
    }

    // 4. Post bad record...

    // 5. Now submit a single GET of the prior POST.
    response = client.get(format!("/transaction/{}/?apikey={}", lid, apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetTransactionResponse::One(_) => assert!(true),
        _ => assert!(false)
    }

    // 6. Make the 2nd Successful post.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=2020-12", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false)
    }

    // 7. Make the 3rd Successful post.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=2020-12-31", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false)
    }

    // 8. Now verify that there are three transactions
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut ret_val = Vec::new();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetTransactionResponse::Many(v) => {
            assert_eq!(v.len(), 3);
            ret_val = v
        },
        _ => assert!(false)
    }
    ret_val

}
