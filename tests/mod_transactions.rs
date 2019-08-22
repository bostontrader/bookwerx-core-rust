use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

pub fn transactions(client: &Client, apikey: &String) -> Vec<R::Transaction> {

    // 1. GET /transactions. sb 200, empty array
    let mut response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new transaction

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/transactions")
        .body(format!("apikey=notarealkey&notes=notes&time=12:00"))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);


    // 2.2 Successful post. 200 and InsertSuccess.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=12:00", apikey))
        .header(ContentType::Form)
        .dispatch();
    let li: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);
    // {"data":{"last_insert_id":"54"}}

    // 2.3 Successful put. 200  and UpdateSuccess
    response = client.put("/transactions")
        .body(format!("apikey={}&id={}&notes=notes&time=12:00", apikey, li.data.last_insert_id))
        .header(ContentType::Form)
        .dispatch();
    let r: R::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);
    // {"data":{"info":"(Rows matched: 1  Changed: 0  Warnings: 0"}}

    // 3. Now verify that there's a single transaction
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 4. Post bad record...

    // 5. Now submit a single GET of the prior POST. 200 and transaction.
    response = client.get(format!("/transaction/{}/?apikey={}", li.data.last_insert_id, apikey))
        .dispatch();
    // The mere fact that this successfully parses an transaction _is_ the test
    let c: R::Transaction = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);


    // 6. Make the 2nd Successful post. 200.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=12:00", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.last_insert_id > 0);
    // {"data":{"last_insert_id":"54"}}


    // 6.1 Now verify that there are two transactions
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    v

}