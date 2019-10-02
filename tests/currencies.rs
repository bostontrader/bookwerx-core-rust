use bookwerx_core_rust::db as D;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Examine currencies
pub fn currencies(client: &Client, apikey: &String) -> Vec<D::Currency> {

    // 1. GET /currencies. sb 200, empty array
    let mut response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    let v: Vec<D::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new currency.

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/currencies")
        .body("apikey=notarealkey&rarity=0&symbol=QTL&title=Quatloo")
        .header(ContentType::Form)
        .dispatch();
    let r: D::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 2.2 Successful post. 200  and InsertSuccess
    response = client.post("/currencies")
        .body(format!("apikey={}&rarity=0&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let li: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 2.3 Successful put. 200  and UpdateSuccess
    response = client.put("/currencies")
        .body(format!("apikey={}&id={}&rarity=0&symbol=QTL&title=Quatloo", apikey, li.data.last_insert_id))
        .header(ContentType::Form)
        .dispatch();
    let r: D::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);

    // 3. Now verify that there's a single currency.
    response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    let v: Vec<D::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 4. Try to post a currency with a duplicated symbol. 200 db error.
    response = client.post("/currencies")
        .body(format!("apikey={}&rarity=0&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: D::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 5. Now submit a single GET of the prior POST. 200 and currency.
    response = client.get(format!("/currency/{}/?apikey={}", li.data.last_insert_id, apikey))
        .dispatch();
    // The mere fact that this successfully parses a currency _is_ the test
    let _c: D::Currency = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);

    // 6. Make a 2nd Successful post. 200.
    response = client.post("/currencies")
        .body(format!("apikey={}&rarity=0&symbol=XAU&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.last_insert_id > 0);

    // 6.1 Verify that there are now two currencies
    response = client.get(format!("/currencies?apikey={}", &apikey)).dispatch();
    let v: Vec<D::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    v

}