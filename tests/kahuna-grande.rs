// RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test kahuna-grande

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use bookwerx_core_rust::constants as C;
use bookwerx_core_rust::db as D;
use bookwerx_core_rust::routes as R;

use rocket::config::{Config, Environment};
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

use std::collections::HashMap;

const TOOLONG: &str = "... what can this strange device be... when I touch it, it gives forth a sound.";

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {

    let client = startup();

    // 1. Everybody needs an API key.  Get that first.
    let apikey: String = apikey(&client);

    // 2. Test in this order in order to accommodate referential integrity
    let currencies = currencies(&client, &apikey);
    let accounts = accounts(&client, &apikey);
    let transactions = transactions(&client, &apikey);
    distributions(&client, &apikey, &accounts, &transactions);

    Ok(())
}

fn startup() -> Client {

    // 1. Build a full connection string of URL to the db server, along with the name of the db to use.
    let mut full_conn = String::new();
    full_conn.push_str(C::TEST_CONN);
    full_conn.push('/');
    full_conn.push_str(C::TEST_DBNAME);

    // 2. Package the full connection string into a HashMap for use by Rocket's config
    let mut hm_inner = HashMap::new();
    hm_inner.insert("url", full_conn);
    let mut hm_outer = HashMap::new();
    hm_outer.insert("mysqldb", hm_inner);

    // 3. Build Rocket's config
    let config = Config::build(Environment::Development)
        .address(C::TEST_BIND_IP)
        .port(C::TEST_BIND_PORT)
        .extra("databases", hm_outer)
        .finalize().unwrap();

    // 4. Now crank up Rocket
    let rocket = rocket::custom(config)
        .attach(D::MyRocketSQLConn::fairing())
        .mount("/", routes![
            //R::index,
            R::get_accounts,
            R::post_account,
            R::post_apikey,
            R::get_currencies,
            R::post_currency,
            R::get_distributions,
            R::post_distribution,
            R::get_transactions,
            R::post_transaction
        ]);

    // 5. Build a client to talk to our instance of Rocket
    let client = Client::new(rocket).expect("valid rocket instance");
    return client
}

// Examine accounts
fn accounts(client: &Client, apikey: &String) -> Vec<R::Account> {

    // 1. GET /accounts, empty array
    let mut response = client.get(format!("/accounts?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Lots of gyrations to find out that this is an array of zero elements.
    let v: serde_json::Value = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.as_array().unwrap().len(), 0);

    // 2. Try to post a new account, but trigger many errors first.

    // 2.1 Post with a missing required field (title)
    response = client.post("/accounts")
        .body("apikey=key&currency_id=666")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.2 Post with an extraneous field.  422.
    response = client.post("/accounts")
        .body("apikey=key&currency_id=666&title=cash in mattress&extraneous=true") // 422 unprocessable entity
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.3.1 Post using a title that's too long.  400.
    response = client.post("/accounts")
        .body(format!("apikey=key&currency_id=666&title={}", TOOLONG))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.3.2 Post using an apikey that's too long.  400.
    response = client.post("/accounts")
        .body(format!("apikey={}&currency_id=666&title=catfood", TOOLONG))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.4.1 Post using a non-numeric currency_id.  422.
    response = client.post("/accounts")
        .body("apikey=key&currency_id=catfood&title=yum")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.4.2 Post using a non-existant apikey. 400
    response = client.post("/accounts")
        .body("apikey=notarealkey&currency_id=1&title=cash in mattress")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.5 Successful post. 200. WTF with the currency_id??
    response = client.post("/accounts")
        .body(format!("apikey={}&currency_id=2&title=cash in mattress", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 3. Now verify that there's a single account
    response = client.get(format!("/accounts?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of one element.
    let v: serde_json::Value = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.as_array().unwrap().len(), 1);

    // 4. Make the 2nd Successful post. 200. (why does currency_id skip from 2 to 4?
    response = client.post("/accounts")
        .body(format!("apikey={}&currency_id=4&title=bank of mises", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 4.1 Now verify that there are two accounts
    response = client.get(format!("/accounts?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of two elements.
    let v: Vec<R::Account> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.len(), 2);

    v

}

// Get an API key
fn apikey(client: &Client) -> String {

    let mut response = client.post("/apikeys").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let ak: R::Apikey = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    ak.apikey
}

// Examine currencies
fn currencies(client: &Client, apikey: &String) -> Vec<R::Currency> {

    // 1. GET /currencies, empty array
    let mut response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Lots of gyrations to find out that this is an array of zero elements.
    let v: serde_json::Value = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.as_array().unwrap().len(), 0);

    // 2. Try to post a new currency, but trigger many errors first.

    // 2.1 Post with a missing required field (title)
    response = client.post("/currencies")
        .body("apikey=key&symbol=value&otherField=123")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.2 Post with an extraneous field.  422.
    response = client.post("/currencies")
        .body("apikey=key&symbol=QTL&title=Quatloo&extraneous_field=true") // 422 unprocessable entity
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.3 Fields that are too long.

    // 2.3.1 Post using an apikey that's too long.  400.
    response = client.post("/currencies")
        .body(format!("apikey={}&symbol=QTL&title", TOOLONG))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.3.2 Post using a symbol that's too long.  400.
    response = client.post("/currencies")
        .body(format!("apikey=key&title=Quatloo&symbol={}", TOOLONG))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.3.3 Post using a title that's too long.  400.
    response = client.post("/currencies")
        .body(format!("apikey=key&symbol=QTL&title={}", TOOLONG))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.4 Post using a non-existant apikey. 400
    response = client.post("/currencies")
        .body("apikey=notarealkey&symbol=QTL&title=Quatloo")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.5 Successful post. 200.
    response = client.post("/currencies")
        .body(format!("apikey={}&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 3. Now verify that there's a single currency and try to erroneously post a currency with a duplicated symbol
    response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of one element.
    let v: serde_json::Value = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.as_array().unwrap().len(), 1);

    // 3.1 Duplicate post. 400.
    response = client.post("/currencies")
        .body(format!("apikey={}&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 4. 2nd Successful post. 200.
    response = client.post("/currencies")
        .body("apikey=key&title=Gold, g&symbol=XAU")
        .body(format!("apikey={}&symbol=XAU&title=Quatloo", apikey))

        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 4.1 Now verify that there are two currencies
    response = client.get(format!("/currencies?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of two elements.
    let v: Vec<R::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.len(), 2);

    v

}

// Examine distributions.  These are substantially different than the other resources.
fn distributions(client: &Client, apikey: &String, accounts: &Vec<R::Account>, transactions: &Vec<R::Transaction>) -> Vec<R::Distribution> {

    // 1. GET /distributions, empty array
    // A distribution does not have an apikey, but its parent transaction does.  Make sure all this matches.
    let account_id: u32 = (*accounts.get(0).unwrap()).id;
    let transaction_id: u32 = (*transactions.get(0).unwrap()).id;

    let mut response = client.get(format!("/distributions?apikey={}&transaction_id={}", &apikey, &transaction_id)).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Lots of gyrations to find out that this is an array of zero elements.
    let v: Vec<R::Distribution> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.len(), 0);

    // 2. Try to post a new distribution, but trigger many errors first.

    // 2.1 Post with an extraneous field.  422.
    response = client.post("/distributions")
        .body(format!("account_id={}&amount=12345&amount_exp=-2&apikey={}&transaction_id={}&extraneous=true", account_id, apikey, transaction_id)) // 422 unprocessable entity
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.2 Post using a non-integer amount.  422.
    response = client.post("/distributions")
        .body(format!("account_id={}&amount=nonnumeric&amount_exp=-2&apikey={}&transaction_id={}", account_id, apikey, transaction_id))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.2 Post using a non-integer exp.  422.
    response = client.post("/distributions")
        .body(format!("account_id={}&amount=1000&amount_exp=nonnumeric&apikey={}&transaction_id={}", account_id, apikey, transaction_id))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.3 Post using an apikey that's too long.  400.
    response = client.post("/distributions")
        .body(format!("account_id={}&amount=1000&amount_exp=0&apikey={}&transaction_id={}", account_id, TOOLONG, transaction_id))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.5 Successful post. 200.
    response = client.post("/distributions")
        .body(format!("account_id={}&amount=12550&amount_exp=-2&apikey={}&transaction_id={}", account_id, apikey, transaction_id))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 3. Now verify that there's a single distribution
    response = client.get(format!("/distributions?apikey={}&transaction_id={}", &apikey, &transaction_id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of one element.
    let v: Vec<R::Distribution> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.len(), 1);

    // 4. Make the 2nd Successful post. 200.
    response = client.post("/distributions")
        .body(format!("account_id={}&amount=-12550&amount_exp=-2&apikey={}&transaction_id={}", account_id, apikey, transaction_id))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 4.1 Now verify that there are two distributions
    response = client.get(format!("/distributions?apikey={}&transaction_id={}", &apikey, &transaction_id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of two elements.
    let v: Vec<R::Distribution> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.len(), 2);

    v

}

// Examine transactions
fn transactions(client: &Client, apikey: &String) -> Vec<R::Transaction> {

    // 1. GET /transactions, empty array
    let mut response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Lots of gyrations to find out that this is an array of zero elements.
    let v: serde_json::Value = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.as_array().unwrap().len(), 0);

    // 2. Try to post a new transaction, but trigger many errors first.

    // 2.1 Post with an extraneous field.  422.
    response = client.post("/transactions")
        .body("apikey=key&notes=initial capital&time=now&extraneous=true") // 422 unprocessable entity
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // 2.2 Post using an apikey that's too long.  400.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=initial capital&time=now", TOOLONG))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.3 Post using a non-existant apikey. 400
    response = client.post("/transactions")
        .body("apikey=notarealkey&notes=initial capital&time=now")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    // 2.5 Successful post. 200.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=initial capital&time=now", apikey))
        .header(ContentType::Form)
        .dispatch();
    println!("{:?}", response.body_string().unwrap());
    assert_eq!(response.status(), Status::Ok);

    // 3. Now verify that there's a single transaction
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of one element.
    let v: serde_json::Value = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.as_array().unwrap().len(), 1);

    // 4. Make the 2nd Successful post. 200.
    response = client.post("/transactions")
        .body(format!("apikey={}&notes=initial capital&time=now", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // 4.1 Now verify that there are two transactions
    response = client.get(format!("/transactions?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    // Lots of gyrations to find out that this is an array of two elements.
    let v: Vec<R::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(v.len(), 2);

    v

}