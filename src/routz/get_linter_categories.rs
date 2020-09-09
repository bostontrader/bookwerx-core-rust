use rocket::get;
use rocket::http::{RawStr, Status};
use rocket_contrib::json;

#[get("/linter/categories?<apikey>")]
pub fn get_linter_categories(
    apikey: &RawStr,
    mut conn: crate::db::MyRocketSQLConn,
) -> crate::db::ApiResponseOld {
    let mut params = Vec::new();

    // We receive this argument as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<crate::db::LinterLong> = conn
        .prep_exec(
            r#"
            SELECT c.id, c.symbol, c.title
            FROM categories AS c
            LEFT JOIN accounts_categories AS ac
            ON c.id = ac.category_id
            WHERE c.apikey = :apikey AND ac.category_id IS NULL
            "#,
            params,
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::LinterLong { id, symbol, title }
                })
                .collect()
        })
        .unwrap();

    crate::db::ApiResponseOld {
        json: json!(vec),
        status: Status::Ok,
    }
}
