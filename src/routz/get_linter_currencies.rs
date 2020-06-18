use rocket::http::{RawStr, Status};

// Find unused currencies.
#[get("/linter/currencies?<apikey>")]
pub fn get_linter_currencies(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

    let mut params  = Vec::new();

    // We receive this argument as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<crate::db::Linter> =

        conn.prep_exec(r#"
            SELECT c.id, c.symbol, c.title
            FROM currencies AS c
            LEFT JOIN accounts AS ac
            ON c.id = ac.currency_id
            WHERE c.apikey = :apikey AND ac.currency_id IS NULL
            "#, params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::Linter {
                        id,
                        symbol,
                        title
                    }
                }).collect()
            }).unwrap();

    crate::db::ApiResponse {
        json: json!(vec),
        status: Status::Ok,
    }
}