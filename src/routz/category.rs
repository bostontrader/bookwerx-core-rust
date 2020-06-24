use crate::db::{APIResponse, GetCategoryResponse, MyRocketSQLConn, Category, CategoryShort};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[rocket::delete("/category/<id>?<apikey>")]
pub fn delete_category(id: &RawStr, apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    match conn.prep_exec("DELETE from categories where id = :id and apikey = :apikey",params) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(err.to_string())),
    }
}

#[rocket::get("/category/<id>?<apikey>")]
pub fn get_category(id: &RawStr, apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetCategoryResponse> {

    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Category> =
        conn.prep_exec("SELECT id, apikey, symbol, title from categories where id = :id and apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    Category {id, apikey, symbol, title}
                }).collect()
            }).unwrap();

    match vec.len() {
        0 => Json(GetCategoryResponse::Error(String::from("record not found"))),
        1 => Json(GetCategoryResponse::One((*vec.get(0).unwrap()).clone())), // why is this one different?
        _ => Json(GetCategoryResponse::Error(String::from("ID01T Max fubar error. More than one record found. This does not compute.")))
    }

}

#[rocket::get("/categories?<apikey>")]
pub fn get_categories(apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetCategoryResponse> {

    // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    let mut params  = Vec::new();
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Category> =
        conn.prep_exec("SELECT id, apikey, symbol, title from categories where apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    Category {id, apikey, symbol, title}
                }).collect()
            }).unwrap();

    Json(GetCategoryResponse::Many(vec))

}

#[rocket::post("/categories", data="<category>")]
pub fn post_category(category: rocket::request::Form<CategoryShort>, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    match conn.prep_exec("INSERT INTO categories (apikey, symbol, title) VALUES (:apikey, :symbol, :title)",(
        &category.apikey, &category.symbol, &category.title)) {
        Ok(result) => Json(APIResponse::LastInsertId(result.last_insert_id())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string())))
    }
}

#[rocket::put("/categories", data="<category>")]
pub fn put_category(category: rocket::request::Form<Category>, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    match conn.prep_exec("UPDATE categories SET symbol = :symbol, title = :title where id = :id and apikey = :apikey",(
        &category.symbol, &category.title, &category.id, &category.apikey)) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}