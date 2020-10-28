use crate::db::{APIResponse, Trancat, TrancatShort, GetTrancatResponse, MyRocketSQLConn};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[rocket::delete("/trancat/<id>?<apikey>")]
pub fn delete_trancat(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    match conn.prep_exec(
        "DELETE from transactions_categories where id = :id and apikey = :apikey",
        params,
    ) {
        Ok(result) => Json(APIResponse::Info(
            String::from_utf8_lossy(&result.info()).to_string(),
        )),
        Err(err) => Json(APIResponse::Error(err.to_string())),
    }
}

#[rocket::get("/trancat/<id>?<apikey>")]
pub fn get_trancat(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<GetTrancatResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Trancat> =
        conn.prep_exec("SELECT id, apikey, transaction_id, category_id from transactions_categories where id = :id and apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, transaction_id, category_id) = rocket_contrib::databases::mysql::from_row(row);
                    Trancat {id, apikey, transaction_id, category_id}
                }).collect()
            }).unwrap();

    match vec.len() {
        0 => Json(GetTrancatResponse::Error(String::from("record not found"))),
        1 => Json(GetTrancatResponse::One((*vec.get(0).unwrap()).clone())), // why is this one different?
        _ => Json(GetTrancatResponse::Error(String::from(
            "ID01T Max fubar error. More than one record found. This does not compute.",
        ))),
    }
}

#[rocket::get("/trancats/for_category?<apikey>&<category_id>")]
pub fn get_trancats_for_category(
    apikey: &RawStr,
    category_id: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<GetTrancatResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(apikey.html_escape().to_mut().clone());
    params.push(category_id.html_escape().to_mut().clone());

    let vec: Vec<Trancat> =
        conn.prep_exec("SELECT id, apikey, transaction_id, category_id from transactions_categories where apikey = :apikey and category_id = :category_id", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, transaction_id, category_id) = rocket_contrib::databases::mysql::from_row(row);
                    Trancat {id, apikey, transaction_id, category_id}
                }).collect()
            }).unwrap();

    Json(GetTrancatResponse::Many(vec))
}

#[rocket::post("/trancats", data = "<trancat>")]
pub fn post_trancat(
    trancat: rocket::request::Form<TrancatShort>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    match conn.prep_exec("INSERT INTO transactions_categories (apikey, transaction_id, category_id) VALUES (:apikey, :transaction_id, :category_id)",(&trancat.apikey, &trancat.transaction_id, &trancat.category_id)) {
        Ok(result) => Json(APIResponse::LastInsertId(result.last_insert_id())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string())))
    }
}

#[rocket::put("/trancats", data = "<trancat>")]
pub fn put_trancat(
    trancat: rocket::request::Form<Trancat>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    match conn.prep_exec("UPDATE transactions_categories SET transaction_id = :transaction_id, category_id = :category_id where id = :id and apikey = :apikey",(&trancat.transaction_id, &trancat.category_id, &trancat.id, &trancat.apikey)) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}
