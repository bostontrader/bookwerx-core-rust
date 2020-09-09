use bookwerx_core_rust::db as D;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

pub fn currencies(client: &Client, apikey: &String) -> Vec<D::Currency> {
    // 1. GET /currencies. sb 200, empty array
    let mut response = client
        .get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCurrencyResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false),
    }

    // 2. Try to post a new currency.

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client
        .post("/currencies")
        .body("apikey=notarealkey&rarity=0&symbol=QTL&title=Quatloo")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false),
    }

    // 2.2 Successful post. 200
    response = client
        .post("/currencies")
        .body(format!(
            "apikey={}&rarity=0&symbol=QTL&title=Quatloo",
            apikey
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

    // 2.3 Successful put. 200
    response = client
        .put("/currencies")
        .body(format!(
            "apikey={}&id={}&rarity=0&symbol=QTL&title=Quatloo",
            apikey, lid
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Info(info) => assert_eq!(info, "(Rows matched: 1  Changed: 0  Warnings: 0"),
        _ => assert!(false),
    }

    // 3. Now verify that there's a single currency.
    response = client
        .get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCurrencyResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false),
    }

    // 4. Try to post a currency with a duplicated symbol. 200 db error.
    response = client
        .post("/currencies")
        .body(format!(
            "apikey={}&rarity=0&symbol=QTL&title=Quatloo",
            apikey
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false),
    }

    // 5. Now submit a single GET of the prior POST. 200 and currency.
    response = client
        .get(format!("/currency/{}/?apikey={}", lid, apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCurrencyResponse::One(_) => assert!(true),
        _ => assert!(false),
    }

    // 6. Make a 2nd Successful post. 200.
    response = client
        .post("/currencies")
        .body(format!("apikey={}&rarity=0&symbol=XAU&title=Gold", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 7. Make a 3rd Successful post. 200.  This currency will not be referenced elsewhere and should be caught by the linter.
    response = client
        .post("/currencies")
        .body(format!(
            "apikey={}&rarity=0&symbol=GAS&title=General Atomic Shekel",
            apikey
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 8. Verify that there are now three currencies
    response = client
        .get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut ret_val = Vec::new();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCurrencyResponse::Many(v) => {
            assert_eq!(v.len(), 3);
            ret_val = v
        }
        _ => assert!(false),
    }
    ret_val
}
