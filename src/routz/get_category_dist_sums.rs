use crate::dfp::DFP;
use rocket::get;
use rocket::http::{RawStr, Status};
use rocket_contrib::json;
use std::collections::HashMap;

/*
Given a category_id, find all the distributions related to all accounts tagged as that category, optionally filtered by time, and calculate and return the sum of the distributions for each particular account. Recall that the returned sums will be expressed using a decimal floating point format.

Given an optional boolean decorate param, return extra decorative related fields such as account title and currency symbol.

Given an optional time_stop parameter, filter the above distributions such that
distribution.time <= time_stop before computing the sum.

Given an optional time_start parameter, filter the above distributions such that
time_start <= distribution.time before computing the sum.

Omitting both time_* params gives us a very simple call to find the balance _right now_!

Setting the time_stop param let's us compute a balance as of a certain time and is what we do for
balance sheet items.

Setting both time_* params gives us the change in balance during a time period and is what we do for
 income statement items.

Setting only time_start doesn't seem real useful, but I'm sure somebody can find a need for doing this.
 */
#[get("/category_dist_sums?<apikey>&<category_id>&<time_start>&<time_stop>&<decorate>")]
pub fn get_category_dist_sums(apikey: &RawStr, category_id: &RawStr, time_start: Option<&RawStr>, time_stop: Option<&RawStr>, decorate: Option<&RawStr>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponseOld {

    // 1. Build a vector of sanitized incoming request parameters. We will feed this to the SQL query. While we're here, let's also use this opportunity to build a time filtering clause for the SQL query.
    let mut params  = Vec::new();

    // 1.1 We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    // WARNING! Push these in the same order they are used in the prep_exec function!
    params.push(category_id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    // 1.2 time_start and time_stop are both optional.
    let mut time_clause = String::from("");

    match time_start {
        None => match time_stop {
            None => { },
            Some(time_stop) => {
                params.push(time_stop.html_escape().to_mut().clone());
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

    // 2. Obtain all of the relevant distributions.
    let vec: Vec<crate::db::BalanceResult> =
        conn.prep_exec(format!("
            SELECT ac.id, ds.amount, ds.amount_exp
            FROM categories AS ca
            JOIN accounts_categories AS acats on acats.category_id = ca.id
            JOIN accounts AS ac on acats.account_id = ac.id
            JOIN distributions AS ds ON ds.account_id = ac.id
            JOIN transactions AS tx ON tx.id = ds.transaction_id

            WHERE ca.id = :category_id
            AND ca.apikey = :apikey
            {}
            ORDER BY ac.id
            ", time_clause), params )
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (account_id, amount, amount_exp) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::BalanceResult {
                        account_id,
                        amount,
                        amount_exp
                    }
                }).collect()
            }).unwrap();


    // 3. We now have zero or more records to sum.

    // 3.1 If we only have zero records, we can return an empty, undecorated result now.
    if vec.len() == 0 {
        return crate::db::ApiResponseOld {
            json: json!({"sums": []}),
            status: Status::Ok,
        };
    }

    // 3.2 At this point we know we must have more than zero records to work with.
    // Now compute the actual sum of distributions for each particular account_id, and store the results in a HashMap.
    // We use a HashMap because we'll soon need easy access to these values, given a key.
    let mut hm = HashMap::new();

    // 3.3 If we have requested decorations we will eventually need an "in clause" of account ids to work with.  It's tempting to build that into this loop now.  Resist the urge.  Doing so makes this needlessly complicated.  It's simple and fast enough to build the in_clause separately.
    let mut sum: DFP = DFP {amount: 0, exp: 0};
    let mut prior_account_id = 0;
    for v in vec {
        if v.account_id != prior_account_id {
            // This is the first record of a new account_id
            if prior_account_id == 0 {
                // This is the very first time in the loop. Nothing to do yet.
            } else {
                // This is the first element that has a new account_id. Save the sum from the prior account_id
                hm.insert(prior_account_id, crate::db::AcctSum { account_id: prior_account_id, sum });
            }
            prior_account_id = v.account_id;
            //sum = DFP { amount: v.amount, exp: v.amount_exp };
            sum = DFP { amount: 0, exp: 0 };
        }
        // This records account_id is the same as the prior record, so just add the values
        sum = sum.add(&DFP { amount: v.amount, exp: v.amount_exp });
    }

    // 3.3.1 The above loop should have executed at least once so we should have a real v and sum.
    // Now that the iteration is done, we still need to insert the final v and sum into the HashMap.
    hm.insert(prior_account_id, crate::db::AcctSum{ account_id: prior_account_id, sum});

    // 4. We will soon need this.  Is there an easier way to do this?
    fn to_vec(hm :HashMap<u32, crate::db::AcctSum>) -> Vec<crate::db::AcctSum> {
        let mut ret_val:Vec<crate::db::AcctSum> = Vec::new();

        for (_k, v) in hm {
            ret_val.push(v);
        }
        return ret_val;
    }

    // 5.  Did the caller request decorations?
    match decorate {
        None => {
            // 5.1 If we have not requested the decorations we can return the HashMap as a Vector as the response now.
            return crate::db::ApiResponseOld {
                json: json!({"sums": to_vec(hm)}),
                status: Status::Ok,
            };
        },
        Some(braw) => {
            // 5.2 There is something passed as the decorate parameter.  Can we parse this to a bool?
            match braw.html_escape().to_mut().clone().parse() {
                Ok(b) => {
                    if b {
                        // decorate parsed to true.  Fall through and continue with decorations.
                    } else {
                        // decorate parsed to an explicit false.  No decorations, just the HashMap.
                        return crate::db::ApiResponseOld {
                            json: json!({"sums": to_vec(hm)}),
                            status: Status::Ok,
                        };
                    }
                },
                Err(e) => {
                    // Cannot parse to a bool.
                    return crate::db::ApiResponseOld {
                        json: json!({"error": e.to_string()}),
                        status: Status::Ok,
                    };
                }
            }
        }
    }

    // 6. At this point we know that we have info to return and we know that the caller has requested decorations.  So now build and execute a 2nd SQL query to retrieve the related decorative items.

    // 6.1 First build an "in clause" containing a list of relevant account_id.
    let mut in_clause = String::new();
    in_clause.push_str("(");
    let mut first_time = true;
    for (k, _) in &hm {
        if first_time {
            first_time = false;
        } else {
            in_clause.push_str(", ");
        }
        in_clause.push_str(k.to_string().as_str() )
    }
    in_clause.push_str(")");


    // 6.2 Now build and execute the SQL to get the decorations.
    // WARNING! The two queries are not atomic.  Contemplate what errors might arise because of this.
    params  = Vec::new();
    params.push(apikey.html_escape().to_mut().clone());
    let vec: Vec<crate::db::AccountCurrencyDecorations> =
        conn.prep_exec(format!("
            SELECT ac.id, ac.title, cu.id as currency_id, cu.symbol
            FROM accounts AS ac
            JOIN currencies AS cu ON ac.currency_id = cu.id
            WHERE ac.apikey = :apikey
            AND ac.id IN {}
        ", in_clause), params )
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (account_id, title, currency_id, symbol) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::AccountCurrencyDecorations {
                        account_id,
                        title,
                        currency_id,
                        symbol
                    }
                }).collect()
            }).unwrap();

    // 7.3 Now iterate over all of the Decorations, if any and build the final result.
    let mut ret_val = Vec::new();
    for d in vec {
            match hm.get(&d.account_id) {
                Some(&v) => {
                    let n = crate::db::BalanceResultDecorated {
                        account: crate::db::AccountCurrency {
                            account_id: d.account_id,
                            title: d.title,
                            currency: crate::db::CurrencySymbol {
                                currency_id: d.currency_id,
                                symbol: d.symbol,
                            },
                        },
                        sum: v.sum
                    };
                    ret_val.push(n);
                },
                _ => {
                    // This should never happen. Contemplate why.
                    panic!("max fubar error");
                }
            }
        }

        return crate::db::ApiResponseOld {
            json: json!({"sums": ret_val}),
            status: Status::Ok,
        }
    }
