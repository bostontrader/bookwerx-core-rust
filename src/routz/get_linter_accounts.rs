use rocket::http::{RawStr, Status};
use serde::Serialize;

// Find unused accounts
#[get("/linter/accounts?<apikey>")]
pub fn get_linter_accounts(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

    #[derive(Serialize)]
    pub struct Linter {
        pub id: u32,
        pub title: String
    }

    let mut params  = Vec::new();

    // We receive this argument as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Linter> =

        conn.prep_exec(r#"
            SELECT ac.id, ac.title
            FROM accounts AS ac
            LEFT JOIN distributions AS d on ac.id = d.account_id
            LEFT JOIN accounts_categories AS acat on ac.id = acat.account_id
            WHERE ac.apikey = :apikey AND d.account_id IS NULL AND acat.account_id IS NULL
            "#, params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, title) = rocket_contrib::databases::mysql::from_row(row);
                    Linter {
                        id,
                        title
                    }
                }).collect()
            }).unwrap();

    crate::db::ApiResponse {
        json: json!(vec),
        status: Status::Ok,
    }
}