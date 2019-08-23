use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

pub fn accounts(client: &Client, apikey: &String, currencies: &Vec<R::Currency>) -> Vec<R::Account> {

    // 1. GET /accounts. sb 200, empty array
    let mut response = client.get(format!("/accounts?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Account> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new account

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/accounts")
        .body(format!("apikey=notarealkey&currency_id={}&rarity=0&title=cash in mattress", (currencies.get(0).unwrap()).id))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);
    // {
    //  "error": "MySqlError { ERROR 1452 (23000): Cannot add or update a child row: a foreign key constraint fails (`bookwerx-core-rust-production`.`currencies`, CONSTRAINT `currencies_ibfk_1` FOREIGN KEY (`apikey`) REFERENCES `apikeys` (`apikey`)) }"
    //}

    // 2.2 Successful post. 200 and InsertSuccess.
    response = client.post("/accounts")
        .body(format!("apikey={}&currency_id={}&rarity=0&title=cash in mattress", apikey, (currencies.get(0).unwrap()).id))
        .header(ContentType::Form)
        .dispatch();
    let li: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);
    // {"data":{"last_insert_id":"54"}}

    // 2.3 Successful put. 200  and UpdateSuccess
    response = client.put("/accounts")
        .body(format!("apikey={}&id={}&currency_id={}&rarity=0&title=cash in mattress", apikey, li.data.last_insert_id, (currencies.get(0).unwrap()).id))
        .header(ContentType::Form)
        .dispatch();
    let r: R::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);
    // {"data":{"info":"(Rows matched: 1  Changed: 0  Warnings: 0"}}

    // 3. Now verify that there's a single account
    response = client.get(format!("/accounts?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Account> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 4. Try to post w/bad currency id
    response = client.post("/accounts")
        .body(format!("apikey={}&currency_id=666&rarity=0&title=cash in mattress", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 5. Now submit a single GET of the prior POST. 200 and account.
    response = client.get(format!("/account/{}/?apikey={}", li.data.last_insert_id, apikey))
        .dispatch();
    // The mere fact that this successfully parses an account _is_ the test
    let c: R::Account = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);


    // 6. Make the 2nd Successful post. 200.
    response = client.post("/accounts")
        .body(format!("apikey={}&currency_id={}&rarity=0&title=bank of mises", apikey, (currencies.get(1).unwrap()).id))
        .header(ContentType::Form)
        .dispatch();
    let r: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.last_insert_id > 0);
    // {"data":{"last_insert_id":"54"}}


    // 6.1 Now verify that there are two accounts
    response = client.get(format!("/accounts?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Account> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    v

}
