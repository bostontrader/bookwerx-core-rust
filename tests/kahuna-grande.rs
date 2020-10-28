// RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test kahuna-grande

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod account_dist_sum;
mod accounts;
mod acctcats;
mod apikey;
mod categories;
mod category_dist_sums;
mod currencies;
mod deletor;
mod distributions;
mod linter;
mod trancats;
mod transactions;

use bookwerx_core_rust::constants as C;
use bookwerx_core_rust::db as D;
use bookwerx_core_rust::routes as R;
use bookwerx_core_rust::routz as Z;

use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket::local::Client;

use std::collections::HashMap;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let client = startup();

    // 1. Test the ping
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let _: D::Ping = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();

    // 2. We need two API keys.  Because we need to run the tests twice, once for each key, to ensure that the records stay separate.
    let apikey1: String = apikey::apikey(&client);
    let apikey2: String = apikey::apikey(&client);

    kahuna_grande(&client, &apikey1);
    kahuna_grande(&client, &apikey2);

    Ok(())
}

fn kahuna_grande(client: &Client, apikey: &String) {
    // Test in this order in order to accommodate referential integrity

    // QTL | Quatloo
    // XAU | Gold
    // GAS | General Atomic Shekel <- Unused. Let the linter find this.
    let currencies = currencies::currencies(&client, &apikey);

    // Cash in mattress   | QTL
    // Cash in cookie jar | QTL
    // Bank of Mises      | XAU
    // Boats n hos        | XAU <- Unused. Let the linter find this.
    let accounts = accounts::accounts(&client, &apikey, &currencies);

    // A  | Assets
    // L  | Liabilities
    // Eq | Equity <- Unused. Let the linter find this.
    // C  | Specific customer
    let categories = categories::categories(&client, &apikey);

    // One account (cash in mattress) is tagged with two categories (assets, specific customer).
    // One category (assets) tags two accounts (cash in mattress | cookie jar)
    // Cash in mattress   | Assets
    // Cash in mattress   | Specific customer
    // Cash in cookie jar | Assets
    // Bank of Mises      | Liabilities
    let acctcats = acctcats::acctcats(&client, &apikey, &accounts, &categories);

    /* We will create 3 transactions dated 2020, 2020-12, and 2020-12-31.

    We will create the following distributions for these 3 transactions:
    tx  | account            | amount
    tx0 | cash in mattress   | 3
        | cash in cookie jar | -3
    tx1 | cash in mattress   | 4
        | cash in cookie jar | -4
    tx2 | cash in mattress   | 5
        | cash in cookie jar | -5

    Given these transactions and distributions:

    A. account_dist_sum, for account "cash in mattress" will return the following values for the four permutations of time filter

    Filter                     Sum
    no filter	                12
    time start >= 2020-12        9
    time stop <= 2020-12         7
    2012-12 <= time_start
      && time_stop <= 2012-12    4

    B. category_dist_sums, for categories "assets" and "specific customer" will return the same.

    C. category_dist_sums, for the single category "assets" will return zeros.
    */
    let transactions = transactions::transactions(&client, &apikey);
    let distributions = distributions::distributions(&client, &apikey, &accounts, &transactions);

    // Now test transactions_categories.  In this test we connect various categories to transactions.  Don't worry about any other apparent meaning.
    let trancats = trancats::trancats(&client, &apikey, &transactions, &categories);

    // Do some linting
    linter::linter(&client, &apikey);
    let _ = account_dist_sum::account_dist_sum(&client, &apikey, &accounts);
    let _ = category_dist_sums::category_dist_sums(&client, &apikey, &categories);

    // Now try to delete things.  Ensure that referential integrity constraints prevent inappropriate deletions.
    deletor::deletor(
        &client,
        &apikey,
        &accounts,
        &acctcats,
        &categories,
        &currencies,
        &distributions,
        &trancats,
        &transactions,
    );
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
        .finalize()
        .unwrap();

    // 4. Now crank up Rocket
    let rocket = rocket::custom(config)
        .attach(D::MyRocketSQLConn::fairing())
        .mount(
            "/",
            routes![
                R::index,
                Z::account::delete_account,
                Z::account::get_account,
                Z::get_account_dist_sum::get_account_dist_sum,
                Z::account::get_accounts,
                Z::account::post_account,
                Z::account::put_account,
                Z::acctcat::delete_acctcat,
                Z::acctcat::get_acctcat,
                Z::acctcat::get_acctcats_for_category,
                Z::acctcat::post_acctcat,
                Z::acctcat::put_acctcat,
                R::post_apikey,
                Z::category::delete_category,
                Z::category::get_category,
                Z::category::get_categories,
                Z::category::get_category_bysym,
                Z::get_category_dist_sums::get_category_dist_sums,
                Z::category::post_category,
                Z::category::put_category,
                Z::currency::delete_currency,
                Z::currency::get_currency,
                Z::currency::get_currencies,
                Z::currency::post_currency,
                Z::currency::put_currency,
                Z::distribution::delete_distribution,
                Z::distribution::get_distribution,
                Z::distribution::get_distributions,
                Z::distribution::get_distributions_for_account,
                Z::distribution::get_distributions_for_tx,
                Z::distribution::post_distribution,
                Z::distribution::put_distribution,
                Z::get_linter_accounts::get_linter_accounts,
                Z::get_linter_categories::get_linter_categories,
                Z::get_linter_currencies::get_linter_currencies,
                Z::trancat::delete_trancat,
                Z::trancat::get_trancat,
                Z::trancat::get_trancats_for_category,
                Z::trancat::post_trancat,
                Z::trancat::put_trancat,
                Z::transaction::delete_transaction,
                Z::transaction::get_transaction,
                Z::transaction::get_transactions,
                Z::transaction::post_transaction,
                Z::transaction::put_transaction
            ],
        );

    // 5. Build a client to talk to our instance of Rocket
    let client = Client::new(rocket).expect("valid rocket instance");
    return client;
}
