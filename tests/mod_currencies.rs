use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Examine currencies
pub fn currencies(client: &Client, apikey: &String) -> Vec<R::Currency> {

    // 1. GET /currencies. sb 200, empty array
    let mut response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new currency.

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/currencies")
        .body("apikey=notarealkey&symbol=QTL&title=Quatloo")
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);
    // {
    //  "error": "MySqlError { ERROR 1452 (23000): Cannot add or update a child row: a foreign key constraint fails (`bookwerx-core-rust-production`.`currencies`, CONSTRAINT `currencies_ibfk_1` FOREIGN KEY (`apikey`) REFERENCES `apikeys` (`apikey`)) }"
    //}

    // 2.2 Successful post. 200  and InsertSuccess
    response = client.post("/currencies")
        .body(format!("apikey={}&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let li: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);
    // {"data":{"last_insert_id":"54"}}

    // 2.3 Successful put. 200  and UpdateSuccess
    response = client.put("/currencies")
        .body(format!("apikey={}&id={}&symbol=QTL&title=Quatloo", apikey, li.data.last_insert_id))
        .header(ContentType::Form)
        .dispatch();
    let r: R::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);
    // {"data":{"info":"(Rows matched: 1  Changed: 0  Warnings: 0"}}

    // 3. Now verify that there's a single currency.
    response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 4. Try to post a currency with a duplicated symbol. 200 db error.
    response = client.post("/currencies")
        .body(format!("apikey={}&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);
    // {"error":"MySqlError { ERROR 1062 (23000): Duplicate entry 'sss-HeStH3ihUv' for key 'symbol' }"}

    // 5. Now submit a single GET of the prior POST. 200 and currency.
    response = client.get(format!("/currency/{}/?apikey={}", li.data.last_insert_id, apikey))
        .dispatch();
    // The mere fact that this successfully parses a currency _is_ the test
    let c: R::Currency = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    // {"apikey":"HeStH3ihUv","id":54,"symbol":"sss","title":"ttt"}

    // 6. Make a 2nd Successful post. 200.
    response = client.post("/currencies")
        .body("apikey=key&title=Gold, g&symbol=XAU")
        .body(format!("apikey={}&symbol=XAU&title=Quatloo", apikey))

        .header(ContentType::Form)
        .dispatch();
    let r: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.last_insert_id > 0);
    // {"data":{"last_insert_id":"54"}}

    // 6.1 Verify that there are now two currencies
    response = client.get(format!("/currencies?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    v

}