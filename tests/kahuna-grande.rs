// RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test kahuna-grande

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod mod_accounts;
mod mod_apikey;
mod mod_currencies;
mod mod_deletor;
mod mod_distributions;
mod mod_transactions;

use bookwerx_core_rust::constants as C;
use bookwerx_core_rust::db as D;
use bookwerx_core_rust::routes as R;

use rocket::config::{Config, Environment};
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

use std::collections::HashMap;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let client = startup();

    // 1. We need two API keys.  Because we need to run the tests twice, once for each key, to ensure that the records stay separate.
    let apikey1: String = mod_apikey::apikey(&client);
    let apikey2: String = mod_apikey::apikey(&client);

    kahuna_grande(&client, &apikey1);
    kahuna_grande(&client, &apikey2);

    Ok(())

}

fn kahuna_grande(client: &Client, apikey: &String) {

    // Test in this order in order to accommodate referential integrity
    let currencies = mod_currencies::currencies(&client, &apikey);
    let accounts = mod_accounts::accounts(&client, &apikey, &currencies);
    let transactions = mod_transactions::transactions(&client, &apikey);
    let distributions = mod_distributions::distributions(&client, &apikey, &accounts, &transactions);

    // Now try to delete things.  Ensure that referential integrity constraints prevent inappropriate deletions.
    mod_deletor::deletor(&client, &apikey, &accounts, &currencies, &distributions, &transactions);

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
            R::delete_account,
            R::get_account,
            R::get_accounts,
            R::post_account,
            R::put_account,

            R::post_apikey,

            R::delete_currency,
            R::get_currencies,
            R::get_currency,
            R::post_currency,
            R::put_currency,

            R::delete_distribution,
            R::get_distributions,
            R::get_distributions_for_account,
            R::get_distributions_for_tx,
            R::post_distribution,
            R::put_distribution,

            R::delete_transaction,
            R::get_transaction,
            R::get_transactions,
            R::post_transaction,
            R::put_transaction
        ]);

    // 5. Build a client to talk to our instance of Rocket
    let client = Client::new(rocket).expect("valid rocket instance");
    return client
}



// Examine distributions.  These are substantially different than the other resources.
/*fn distributions(client: &Client, apikey: &String, accounts: &Vec<R::Account>, transactions: &Vec<R::Transaction>) -> Vec<R::Distribution> {

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

}*/
