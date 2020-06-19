// RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test kahuna-grande

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod accounts;
mod account_dist_sum;
mod acctcats;
mod apikey;
mod categories;
mod currencies;
mod deletor;
mod distributions;
mod linter;
mod transactions;

use bookwerx_core_rust::constants as C;
use bookwerx_core_rust::db as D;
use bookwerx_core_rust::routes as R;
use bookwerx_core_rust::routz as Z;

use rocket::config::{Config, Environment};
//use rocket::http::ContentType;
//use rocket::http::Status;
use rocket::local::Client;

use std::collections::HashMap;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let client = startup();

    // 1. We need two API keys.  Because we need to run the tests twice, once for each key, to ensure that the records stay separate.
    let apikey1: String = apikey::apikey(&client);
    let apikey2: String = apikey::apikey(&client);

    kahuna_grande(&client, &apikey1);
    kahuna_grande(&client, &apikey2);

    Ok(())

}

fn kahuna_grande(client: &Client, apikey: &String) {

    // Test in this order in order to accommodate referential integrity
    let currencies = currencies::currencies(&client, &apikey);
    let accounts = accounts::accounts(&client, &apikey, &currencies);
    let transactions = transactions::transactions(&client, &apikey);
    let distributions = distributions::distributions(&client, &apikey, &accounts, &transactions);
    let categories = categories::categories(&client, &apikey);
    let acctcats = acctcats::acctcats(&client, &apikey, &accounts, &categories);

    linter::linter(&client, &apikey);
    let _ = account_dist_sum::account_dist_sum(&client, &apikey, &accounts);

    // Now try to delete things.  Ensure that referential integrity constraints prevent inappropriate deletions.
    deletor::deletor(&client, &apikey, &accounts, &acctcats, &categories, &currencies, &distributions, &transactions);

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
            R::index,

            R::delete_account,
            R::get_account,
            Z::get_account_dist_sum::get_account_dist_sum,
            R::get_accounts,
            R::post_account,
            R::put_account,

            R::delete_acctcat,
            R::get_acctcat,
            R::get_acctcats_for_category,
            R::post_acctcat,
            R::put_acctcat,

            R::post_apikey,

            R::delete_category,
            R::get_category,
            R::get_categories,
            Z::get_category_dist_sums::get_category_dist_sums,
            R::post_category,
            R::put_category,

            R::delete_currency,
            R::get_currency,
            R::get_currencies,
            R::post_currency,
            R::put_currency,

            R::delete_distribution,
            R::get_distribution,
            R::get_distributions,
            R::get_distributions_for_account,
            R::get_distributions_for_tx,
            R::post_distribution,
            R::put_distribution,

            Z::get_linter_accounts::get_linter_accounts,
            Z::get_linter_categories::get_linter_categories,
            Z::get_linter_currencies::get_linter_currencies,

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
