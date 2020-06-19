use crate::dfp::DFP;
use rocket::http::{RawStr, Status};
use serde::{Deserialize, Serialize};

/*
Given an account_id, find all the distributions related to it, optionally filtered by time, and calculate and return their sum. Recall that the returned sum will be expressed using a decimal floating point format.

Given an optional time_stop parameter, filter the above distributions such that
distribution.time <= time_stop before computing the sum.

Given an optional time_start parameter, filter the above distributions such that
time_start <= distribution.time before computing the sum.

Omitting both time_* params gives us a very simple call to find the balance _right now_!

Setting a time_stop param let's us compute a balance as of a certain time and is what we do for
balance sheet items.

Setting both time_* params gives us the change in balance during a time period and is what we do for
 income statement items.

Setting only time_start doesn't seem real useful, but I'm sure somebody can find a need for doing this.
 */
#[get("/account_dist_sum?<apikey>&<account_id>&<time_start>&<time_stop>")]
pub fn get_account_dist_sum(apikey: &RawStr, account_id: &RawStr, time_start: Option<&RawStr>, time_stop: Option<&RawStr>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

    #[derive(Deserialize, Serialize)]
    struct BalanceResult {
        pub account_id: u32,
        pub amount: i64,
        pub amount_exp: i8,
    }

    // This is a vector of parameters that we recover from the request and feed into our sql statement
    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    // WARNING! Push these in the same order they are used in the prep_exec function!
    params.push(account_id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    // time_start and time_stop are both optional. This will affect what we push onto the param stack
    // as well as the actual sql statement
    let mut time_clause = String::from("");

    match time_start {
        None => match time_stop {
            None => { },
            Some(n) => {
                params.push(n.html_escape().to_mut().clone());
                time_clause = String::from("AND tx.time <= :time_stop");
            }
        }
        Some(time_start) => match time_stop {
            None =>  {
                params.push(time_start.html_escape().to_mut().clone());
                time_clause = String::from("AND :time_start <= tx.time");
            },
            Some(time_stop) =>  {
                params.push(time_start.html_escape().to_mut().clone());
                params.push(time_stop.html_escape().to_mut().clone());
                time_clause = String::from("AND :time_start <= tx.time AND tx.time <= :time_stop");
            }
        }
    }

    let vec: Vec<BalanceResult> =
        conn.prep_exec(format!("
                SELECT ac.id, ds.amount, ds.amount_exp
                FROM accounts AS ac
                JOIN distributions AS ds ON ds.account_id = ac.id
                JOIN transactions AS tx ON tx.id = ds.transaction_id
                WHERE ac.id = :account_id
                    AND ac.apikey = :apikey
                    {}
                    ", time_clause), params )
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (account_id, amount, amount_exp) = rocket_contrib::databases::mysql::from_row(row);
                    BalanceResult {
                        account_id,
                        amount,
                        amount_exp,
                    }
                }).collect()
            }).unwrap();


    // We now have zero or more records to sum.
    let mut sum: DFP = DFP {amount: 0, exp: 0};
    for n in vec {
        sum = sum.add(&DFP{amount:n.amount, exp:n.amount_exp });
    }

    // Now build and return the http response.
    crate::db::ApiResponse {
        json: json!({"sum": sum}),
        status: Status::Ok,
    }

}
