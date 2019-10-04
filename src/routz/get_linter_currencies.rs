pub fn add_to_waitlist() {}

use rocket::http::{RawStr, Status};

#[get("/linter/currencies?<apikey>")]
pub fn get_linter_currencies(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

    let mut v1  = Vec::new();

    // We receive this argument as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    v1.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<crate::db::Linter> =

        conn.prep_exec(r#"
            SELECT c.id, c.symbol, c.title
            FROM currencies AS c
            LEFT JOIN accounts AS ac
            ON c.id = ac.currency_id
            WHERE c.apikey = :apikey AND ac.currency_id IS NULL
            "#, v1)
            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Linter>`
                // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `Payment`
                result.map(|x| x.unwrap()).map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow the schema
                    let (id, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::Linter {
                        id: id,
                        symbol: symbol,
                        title: title
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Linter>`
            }).unwrap(); // Unwrap `Vec<Linter>`

    crate::db::ApiResponse {
        json: json!(vec),
        status: Status::Ok,
    }
}