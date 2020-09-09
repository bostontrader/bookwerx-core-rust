use crate::model::{JunDatabase};
use serde::{Serialize};

#[derive(Debug)]
#[derive(Serialize)]
struct Rs {
    pub amount: i32,
    pub exp: i32,
    pub time: String,
    pub symbol: String,
}

pub struct Query;

#[juniper::graphql_object(
    Context = JunDatabase
    Scalar = juniper::DefaultScalarValue,
)]

// The root query object of the schema
impl Query {

    // Get a collection of zero or more accounts.
    fn accounts(&self, database: &JunDatabase) -> String {
        // 1. Build the basic query string with suitable placeholders for the prepared statement parameters
        let q = format!(
            "SELECT id, currency_id, rarity, title
            FROM accounts "
        );
        /*let q = format!(
            "SELECT amount, amount_exp, `time`, c.symbol AS symbol FROM distributions AS d
                JOIN transactions AS t on t.id = d.transaction_id
                JOIN accounts AS a on a.id = d.account_id
                JOIN currencies AS c on c.id = a.currency_id
                WHERE account_id =
                    ( SELECT a.id FROM accounts_categories AS ac
                      JOIN accounts AS a ON ac.account_id = a.id
                      WHERE a.apikey = :apikey AND ac.category_id IN(:in_list)
                      GROUP BY a.id HAVING COUNT(a.id) = {}
                    )",l);
*/
        // 1.3
        //let mut params = Vec::new();
        //params.push(api_key);

        // 1.4 Obtain a connection to the db using deep magic
        let mut m = database.0.lock().unwrap();
        let m = m.as_deref_mut().unwrap();

        // 1.5 Now execute the query
        //let n: Vec<Rs> = m.prep_exec(q, params).map(|result| {
            //result.map(|x| x.unwrap()).map(|row| {
                //let (amount, exp, time, symbol) = rocket_contrib::databases::mysql::from_row(row);
                //Rs {amount, exp, time, symbol}
            //}).collect()
        //}).unwrap();

        //let serialized = serde_json::to_string(&n).unwrap();
        //String::from(serialized)

        String::from("[]")

    }
        /*
           Get all distributions for all accounts that are tagged with _all_ categories passed in category_ids.
           If category_ids is empty then no accounts are relevant and no distributions get returned.

           This is our first hello-graphql query that generates actual useful output.  As such it is not very well
           polished.  Be gentle with it.

           Returns a String that should be parsable into JSON.

           curl -X POST -H "Content-Type: application/json" -d '{"query": "{ distributions(categoryIds:[], apiKey:\"catfood\") }"}' http://localhost:3003/graphq
         */


    #[graphql(arguments(category_ids(description = "A list of relevant category ids.")))]
    fn distributions(&self, database: &JunDatabase, category_ids: Vec<i32>, api_key: String) -> String {

        /* 1. Our first goal is to find a list of accounts that are tagged with _all_ of the given categories.  If
           given an empty Vec of category_ids no accounts qualify.  If we _do_ have a Vec of category_ids of len > 0
           then we want to build a String of comma separated category_id for subsequent use in our query.  However,
           in order to prevent SQL injection, we'll inject this String into a prepared statement.
         */

        let l = category_ids.len();
        if l > 0 {

            /* 1.1 Build a comma separated list of the category_ids for use in our query
               recall that we'll feed the in_list into the query as a part of a prepared
               statement in order to avoid SQL injection.
             */

            let mut in_list = String::from(category_ids[0].to_string());
            for category_id in &category_ids[1..l] {
                in_list.push_str(&format!(", {}", category_id));
            };

            // 1.2 Now build the basic query string with suitable placeholders for the prepared statement parameters
            let q = format!(
                "SELECT amount, amount_exp, `time`, c.symbol AS symbol FROM distributions AS d
                JOIN transactions AS t on t.id = d.transaction_id
                JOIN accounts AS a on a.id = d.account_id
                JOIN currencies AS c on c.id = a.currency_id
                WHERE account_id =
                    ( SELECT a.id FROM accounts_categories AS ac
                      JOIN accounts AS a ON ac.account_id = a.id
                      WHERE a.apikey = :apikey AND ac.category_id IN(:in_list)
                      GROUP BY a.id HAVING COUNT(a.id) = {}
                    )",l);

            // 1.3 Pack our apikey and in_list into a Vec of String in order to subsequently use this
            // to build a prepared statement.  Be sure to insert them in this order.
            let mut params = Vec::new();
            params.push(api_key);
            params.push(in_list);

            // 1.4 Obtain a connection to the db using deep magic
            let mut m = database.0.lock().unwrap();
            let m = m.as_deref_mut().unwrap();

            // 1.5 Now execute the query
            let n: Vec<Rs> = m.prep_exec(q, params).map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (amount, exp, time, symbol) = rocket_contrib::databases::mysql::from_row(row);
                    Rs {amount, exp, time, symbol}
                }).collect()
            }).unwrap();

            let serialized = serde_json::to_string(&n).unwrap();
            String::from(serialized)
        } else {
            String::from("[]")
        }
    }

}
