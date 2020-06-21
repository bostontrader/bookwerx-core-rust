use bookwerx_core_rust::db as D;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

/* This function will create 3 transactions.  The times of the transactions, as well as the amounts of the related distributions, have been crafted such that the account_dist_sum and category_dist_sums functions will be able to differentiate between these tx. To wit:

Time        Amt
2020          3
2020-12       4
2020-12-31    5

Filter                     Sum
no filter	                12
time start >= 2020-12        9
time stop <= 2020-12         7
2012-12 <= time_start
  && time_stop <= 2012-12    4
*/
pub fn transactions(client: &Client, apikey: &String) -> Vec<D::Transaction> {

    // 1. GET /transactions. sb 200, empty array
    let mut response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    let v: Vec<D::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new transaction

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/transactions")
        .body(format!("apikey=notarealkey&notes=notes&time=2020"))
        .header(ContentType::Form)
        .dispatch();
    let r: D::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);


    // 2.2 Successful post. 200 and InsertSuccess.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=2020", apikey))
        .header(ContentType::Form)
        .dispatch();
    let li: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 2.3 Successful put. 200 and UpdateSuccess
    response = client.put("/transactions")
        .body(format!("apikey={}&id={}&notes=notes&time=2020", apikey, li.data.last_insert_id))
        .header(ContentType::Form)
        .dispatch();
    let r: D::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);

    // 3. Now verify that there's a single transaction
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    let v: Vec<D::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 4. Post bad record...

    // 5. Now submit a single GET of the prior POST. 200 and transaction.
    response = client.get(format!("/transaction/{}/?apikey={}", li.data.last_insert_id, apikey))
        .dispatch();
    // The mere fact that this successfully parses an transaction _is_ the test
    let _c: D::Transaction = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);


    // 6. Make the 2nd Successful post. 200.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=2020-12", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.last_insert_id > 0);

    // 7. Make the 3rd Successful post. 200.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=notes&time=2020-12-31", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.last_insert_id > 0);

    // 8. Now verify that there are three transactions
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    let v: Vec<D::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 3);

    v

}
