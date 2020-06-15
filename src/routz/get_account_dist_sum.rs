use rocket::http::{RawStr, Status};

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

    // This is a vector of parameters that we recover from the request and feed into our sql statement
    let mut v1  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    // WARNING! Push these in the same order they are used in the prep_exec function!
    v1.push(account_id.html_escape().to_mut().clone());
    v1.push(apikey.html_escape().to_mut().clone());

    // time_start and time_stop are both optional. This will affect what we push onto the param stack
    // as well as the actual sql statement
    let mut time_clause = String::from("");

    match time_start {
        None => match time_stop {
            None => { },
            Some(n) => {
                v1.push(n.html_escape().to_mut().clone());
                time_clause = String::from("AND tx.time <= :time_stop");
            }
        }
        Some(n) => match time_stop {
            None =>  {
                v1.push(n.html_escape().to_mut().clone());
                time_clause = String::from("AND :time_start <= tx.time");
            },
            Some(n) =>  {
                v1.push(n.html_escape().to_mut().clone());
                time_clause = String::from("AND :time_start <= tx.time AND tx.time <= :time_stop");
            }
        }
    }

    let vec: Vec<crate::db::BalanceResult> =
        conn.prep_exec(format!("
                SELECT ac.id, ds.amount, ds.amount_exp, tx.time
                FROM accounts AS ac
                JOIN distributions AS ds ON ds.account_id = ac.id
                JOIN transactions AS tx ON tx.id = ds.transaction_id
                WHERE ac.id = :account_id
                    AND ac.apikey = :apikey
                    {}
                ORDER BY tx.time

                    ", time_clause), v1 )
            .map(|result| { // In this closure we will map `QueryResult` to `Vec<BalanceResult>`
                // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `Payment`
                result.map(|x| x.unwrap()).map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow the schema
                    let (account_id, amount, amount_exp, time) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::BalanceResult {
                        account_id: account_id,
                        amount: amount,
                        amount_exp: amount_exp,
                        time: time
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Account>`
            }).unwrap(); // Unwrap `Vec<Account>`


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

#[derive(Serialize)]
struct DFP {
    amount: i64,
    exp: i8,
}

impl DFP {
    fn add(&self, n2: &DFP) -> DFP {
        let d = self.exp - n2.exp;
        if d >= 1 {
            return n2.add(&DFP { amount: self.amount * 10, exp: self.exp - 1 })
        } else if d == 0 {
            return DFP { amount: self.amount + n2.amount, exp: self.exp }
        } else {
            return n2.add(self)
        }

    }

}

#[test]
fn test_dfp() {

    let mut n = DFP{ amount: 1, exp: -1};

    // 1.
    n = DFP { amount: 1, exp: -1}.add(&DFP { amount: 1, exp: -1});
    assert!(n.amount == 2);
    assert!(n.exp == -1);

    // 2.
    n = DFP { amount: 1, exp: -1}.add(&DFP { amount: 1, exp: 0});
    assert!(n.amount == 11);
    assert!(n.exp == -1);

    // 3.
    n = DFP { amount: 1, exp: -1}.add(&DFP { amount: 1, exp: 1});
    assert!(n.amount == 101);
    assert!(n.exp == -1);

    // 4.
    n = DFP { amount: 1, exp: 0}.add(&DFP { amount: 1, exp: -1});
    assert!(n.amount == 11);
    assert!(n.exp == -1);

    // 5.
    n = DFP { amount: 1, exp: 0}.add(&DFP { amount: 1, exp: 0});
    assert!(n.amount == 2);
    assert!(n.exp == 0);

    // 6.
    n = DFP { amount: 1, exp: 0}.add(&DFP { amount: 1, exp: 1});
    assert!(n.amount == 11);
    assert!(n.exp == 0);

    // 7.
    n = DFP { amount: 1, exp: 1}.add(&DFP { amount: 1, exp: -1});
    assert!(n.amount == 101);
    assert!(n.exp == -1);

    // 8.
    n = DFP { amount: 1, exp: 1}.add(&DFP { amount: 1, exp: 0});
    assert!(n.amount == 11);
    assert!(n.exp == 0);

    // 9.
    n = DFP { amount: 1, exp: 1}.add(&DFP { amount: 1, exp: 1});
    assert!(n.amount == 2);
    assert!(n.exp == 1);


}