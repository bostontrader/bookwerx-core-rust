use crate::db::{APIResponse, Currency, CurrencyShort, GetCurrencyResponse, MyRocketSQLConn};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[rocket::delete("/currency/<id>?<apikey>")]
pub fn delete_currency(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    match conn.prep_exec(
        "DELETE from currencies where id = :id and apikey = :apikey",
        params,
    ) {
        Ok(result) => Json(APIResponse::Info(
            String::from_utf8_lossy(&result.info()).to_string(),
        )),
        Err(err) => Json(APIResponse::Error(err.to_string())),
    }
}

#[rocket::get("/currency/<id>?<apikey>")]
pub fn get_currency(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<GetCurrencyResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Currency> =
        conn.prep_exec("SELECT id, apikey, rarity, symbol, title from currencies where id = :id and apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, rarity, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    Currency {id,apikey,rarity,symbol,title}
                }).collect()
            }).unwrap();

    match vec.len() {
        0 => Json(GetCurrencyResponse::Error(String::from("record not found"))),
        1 => Json(GetCurrencyResponse::One(vec.get(0).unwrap().clone())),
        _ => Json(GetCurrencyResponse::Error(String::from(
            "ID01T Max fubar error. More than one record found. This does not compute.",
        ))),
    }
}

#[rocket::get("/currencies?<apikey>")]
pub fn get_currencies(apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetCurrencyResponse> {
    // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    let mut params = Vec::new();
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Currency> = conn
        .prep_exec(
            "SELECT id, apikey, rarity, symbol, title from currencies where apikey = :apikey",
            params,
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, apikey, rarity, symbol, title) =
                        rocket_contrib::databases::mysql::from_row(row);
                    Currency {
                        id,
                        apikey,
                        rarity,
                        symbol,
                        title,
                    }
                })
                .collect()
        })
        .unwrap();

    Json(GetCurrencyResponse::Many(vec))
}

#[rocket::post("/currencies", data = "<currency>")]
pub fn post_currency(
    currency: rocket::request::Form<CurrencyShort>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    match conn.prep_exec("INSERT INTO currencies (apikey, rarity, symbol, title) VALUES (:apikey, :rarity, :symbol, :title)",(&currency.apikey, &currency.rarity, &currency.symbol, &currency.title)) {
        Ok(result) => Json(APIResponse::LastInsertId(result.last_insert_id())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string())))
    }
}

#[rocket::put("/currencies", data = "<currency>")]
pub fn put_currency(
    currency: rocket::request::Form<Currency>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    match conn.prep_exec("UPDATE currencies SET rarity = :rarity, symbol = :symbol, title = :title where id = :id and apikey = :apikey",(&currency.rarity, &currency.symbol, &currency.title, &currency.id, &currency.apikey)) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}
