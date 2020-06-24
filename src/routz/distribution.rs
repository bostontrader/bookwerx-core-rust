use crate::db::{APIResponse, GetDistributionResponse, MyRocketSQLConn, Distribution, DistributionJoined, DistributionShort};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[rocket::delete("/distribution/<id>?<apikey>")]
pub fn delete_distribution(id: &RawStr, apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    match conn.prep_exec("DELETE from distributions where id = :id and apikey = :apikey",params) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(err.to_string())),
    }
}

#[rocket::get("/distribution/<id>?<apikey>")]
pub fn get_distribution(id: &RawStr, apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetDistributionResponse> {

    let mut params  = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Distribution> =
        conn.prep_exec("SELECT id, apikey, account_id, amount, amount_exp, transaction_id from distributions where id = :id and apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, account_id, amount, amount_exp, transaction_id) = rocket_contrib::databases::mysql::from_row(row);
                    Distribution {id, apikey, account_id, amount, amount_exp, transaction_id}
                }).collect()
            }).unwrap();

    match vec.len() {
        0 => Json(GetDistributionResponse::Error(String::from("record not found"))),
        1 => Json(GetDistributionResponse::One((*vec.get(0).unwrap()).clone())),
        _ => Json(GetDistributionResponse::Error(String::from("ID01T Max fubar error. More than one record found. This does not compute.")))
    }
}

#[rocket::get("/distributions?<apikey>")]
pub fn get_distributions(apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetDistributionResponse> {

    // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    let mut params  = Vec::new();
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Distribution> =
        conn.prep_exec("SELECT id, account_id, amount, amount_exp, apikey, transaction_id from distributions where apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, account_id, amount, amount_exp, apikey, transaction_id) = rocket_contrib::databases::mysql::from_row(row);
                    Distribution {id, account_id, amount, amount_exp, apikey, transaction_id}
                }).collect()
            }).unwrap();

    Json(GetDistributionResponse::Many(vec))

}

// This is the core functionality of getting the distributions shared by for_tx and for_account
fn get_distributions_private(query: &str, params: Vec<String>, mut conn: MyRocketSQLConn) -> Json<GetDistributionResponse> {

    let vec: Vec<DistributionJoined> =
        conn.prep_exec(query, params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (did, tid, aid, amount, amount_exp, apikey, title, time, notes) = rocket_contrib::databases::mysql::from_row(row);
                    DistributionJoined {
                        id: did,
                        tid: tid,
                        aid: aid,
                        amount: amount,
                        amount_exp: amount_exp,
                        apikey: apikey,
                        account_title: title,
                        tx_notes: notes,
                        tx_time: time
                    }


                }).collect()
            }).unwrap();

    Json(GetDistributionResponse::ManyJoined(vec))

}

#[rocket::get("/distributions/for_account?<apikey>&<account_id>")]
pub fn get_distributions_for_account(apikey: &RawStr, account_id: &RawStr, conn: MyRocketSQLConn) -> Json<GetDistributionResponse> {

    let mut params  = Vec::new(); // parametrization

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(apikey.html_escape().to_mut().clone());
    params.push(account_id.html_escape().to_mut().clone());

    get_distributions_private("SELECT d.id as did, t.id as tid, a.id as aid, amount, amount_exp, d.apikey, title, time, notes from distributions as d join transactions as t on d.transaction_id = t.id join accounts as a on d.account_id = a.id where d.apikey = :apikey and account_id = :account_id order by time", params, conn)

}

#[rocket::get("/distributions/for_tx?<apikey>&<transaction_id>")]
pub fn get_distributions_for_tx(apikey: &RawStr, transaction_id: &RawStr, conn: MyRocketSQLConn) -> Json<GetDistributionResponse> {

    let mut params  = Vec::new(); // parametrization

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(apikey.html_escape().to_mut().clone());
    params.push(transaction_id.html_escape().to_mut().clone());

    get_distributions_private("SELECT d.id as did, t.id as tid, a.id as aid, amount, amount_exp, d.apikey, title, time, notes from distributions as d join transactions as t on d.transaction_id = t.id join accounts as a on d.account_id = a.id where d.apikey = :apikey and transaction_id = :transaction_id order by time", params, conn)

}

#[rocket::post("/distributions", data="<distribution>")]
pub fn post_distribution(distribution: rocket::request::Form<DistributionShort>, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    match conn.prep_exec("INSERT INTO distributions (account_id, amount, amount_exp, apikey, transaction_id) VALUES (:account_id, :amount, :amount_exp, :apikey, :transaction_id)",(&distribution.account_id, &distribution.amount, &distribution.amount_exp, &distribution.apikey, &distribution.transaction_id)) {
        Ok(result) => Json(APIResponse::LastInsertId(result.last_insert_id())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string())))
    }

}

#[rocket::put("/distributions", data="<distribution>")]
pub fn put_distribution(distribution: rocket::request::Form<Distribution>, mut conn: MyRocketSQLConn) -> Json<APIResponse> {

    match conn.prep_exec("UPDATE distributions SET account_id = :account_id, amount = :amount, amount_exp = :amount_exp, transaction_id = :transaction_id where id = :id and apikey = :apikey",(&distribution.account_id, &distribution.amount, &distribution.amount_exp, &distribution.transaction_id, &distribution.id, &distribution.apikey)) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}