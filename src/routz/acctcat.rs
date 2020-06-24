use crate::db::{Acctcat, AcctcatShort, APIResponse, GetAcctcatResponse, MyRocketSQLConn};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[rocket::delete("/acctcat/<id>?<apikey>")]
pub fn delete_acctcat(id: &RawStr, apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    match conn.prep_exec("DELETE from accounts_categories where id = :id and apikey = :apikey",params) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(err.to_string())),
    }
}

#[rocket::get("/acctcat/<id>?<apikey>")]
pub fn get_acctcat(id: &RawStr, apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetAcctcatResponse> {

    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Acctcat> =
        conn.prep_exec("SELECT id, apikey, account_id, category_id from accounts_categories where id = :id and apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, account_id, category_id) = rocket_contrib::databases::mysql::from_row(row);
                    Acctcat {id, apikey, account_id, category_id}
                }).collect()
            }).unwrap();

    match vec.len() {
        0 => Json(GetAcctcatResponse::Error(String::from("record not found"))),
        1 => Json(GetAcctcatResponse::One((*vec.get(0).unwrap()).clone())), // why is this one different?
        _ => Json(GetAcctcatResponse::Error(String::from("ID01T Max fubar error. More than one record found. This does not compute.")))
    }
}

#[rocket::get("/acctcats/for_category?<apikey>&<category_id>")]
pub fn get_acctcats_for_category(apikey: &RawStr, category_id: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetAcctcatResponse> {

    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(apikey.html_escape().to_mut().clone());
    params.push(category_id.html_escape().to_mut().clone());

    let vec: Vec<Acctcat> =
        conn.prep_exec("SELECT id, apikey, account_id, category_id from accounts_categories where apikey = :apikey and category_id = :category_id", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, account_id, category_id) = rocket_contrib::databases::mysql::from_row(row);
                    Acctcat {id, apikey, account_id, category_id}
                }).collect()
            }).unwrap();

    Json(GetAcctcatResponse::Many(vec))

}

#[rocket::post("/acctcats", data="<acctcat>")]
pub fn post_acctcat(acctcat: rocket::request::Form<AcctcatShort>, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    match conn.prep_exec("INSERT INTO accounts_categories (apikey, account_id, category_id) VALUES (:apikey, :account_id, :category_id)",(&acctcat.apikey, &acctcat.account_id, &acctcat.category_id)) {
        Ok(result) => Json(APIResponse::LastInsertId(result.last_insert_id())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string())))
    }

}

#[rocket::put("/acctcats", data="<acctcat>")]
pub fn put_acctcat(acctcat: rocket::request::Form<Acctcat>, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    match conn.prep_exec("UPDATE accounts_categories SET account_id = :account_id, category_id = :category_id where id = :id and apikey = :apikey",(&acctcat.account_id, &acctcat.category_id, &acctcat.id, &acctcat.apikey)) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}
