use crate::db::{
    APIResponse, GetTransactionResponse, MyRocketSQLConn, Transaction, TransactionShort,
};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[rocket::delete("/transaction/<id>?<apikey>")]
pub fn delete_transaction(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    match conn.prep_exec(
        "DELETE from transactions where id = :id and apikey = :apikey",
        params,
    ) {
        Ok(result) => Json(APIResponse::Info(
            String::from_utf8_lossy(&result.info()).to_string(),
        )),
        Err(err) => Json(APIResponse::Error(err.to_string())),
    }
}

#[rocket::get("/transaction/<id>?<apikey>")]
pub fn get_transaction(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<GetTransactionResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Transaction> = conn
        .prep_exec(
            "SELECT id, apikey, notes, time from transactions where id = :id and apikey = :apikey",
            params,
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, apikey, notes, time) = rocket_contrib::databases::mysql::from_row(row);
                    Transaction {
                        id,
                        apikey,
                        notes,
                        time,
                    }
                })
                .collect()
        })
        .unwrap();

    match vec.len() {
        0 => Json(GetTransactionResponse::Error(String::from(
            "record not found",
        ))),
        1 => Json(GetTransactionResponse::One((*vec.get(0).unwrap()).clone())), // why is this one different?
        _ => Json(GetTransactionResponse::Error(String::from(
            "ID01T Max fubar error. More than one record found. This does not compute.",
        ))),
    }
}

#[rocket::get("/transactions?<apikey>")]
pub fn get_transactions(
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<GetTransactionResponse> {
    // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    let mut params = Vec::new();
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Transaction> = conn
        .prep_exec(
            "SELECT id, apikey, notes, time from transactions where apikey = :apikey",
            params,
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, apikey, notes, time) = rocket_contrib::databases::mysql::from_row(row);
                    Transaction {
                        id,
                        apikey,
                        notes,
                        time,
                    }
                })
                .collect()
        })
        .unwrap();

    Json(GetTransactionResponse::Many(vec))
}

#[rocket::post("/transactions", data = "<transaction>")]
pub fn post_transaction(
    transaction: rocket::request::Form<TransactionShort>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    match conn.prep_exec(
        "INSERT INTO transactions (apikey, notes, time) VALUES (:apikey, :notes, :time)",
        (&transaction.apikey, &transaction.notes, &transaction.time),
    ) {
        Ok(result) => Json(APIResponse::LastInsertId(result.last_insert_id())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}

#[rocket::put("/transactions", data = "<transaction>")]
pub fn put_transaction(
    transaction: rocket::request::Form<Transaction>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    match conn.prep_exec(
        "UPDATE transactions SET notes = :notes, time = :time where id = :id and apikey = :apikey",
        (
            &transaction.notes,
            &transaction.time,
            &transaction.id,
            &transaction.apikey,
        ),
    ) {
        Ok(result) => Json(APIResponse::Info(
            String::from_utf8_lossy(&result.info()).to_string(),
        )),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}
