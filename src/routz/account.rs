use crate::db::{
    APIResponse, Account, AccountDenormalized, AccountJoined, AccountShort, GetAccountResponse,
    MyRocketSQLConn,
};
use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[rocket::delete("/account/<id>?<apikey>")]
pub fn delete_account(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    match conn.prep_exec(
        "DELETE from accounts where id = :id and apikey = :apikey",
        params,
    ) {
        Ok(result) => Json(APIResponse::Info(
            String::from_utf8_lossy(&result.info()).to_string(),
        )),
        Err(err) => Json(APIResponse::Error(err.to_string())),
    }
}

#[rocket::get("/account/<id>?<apikey>")]
pub fn get_account(
    id: &RawStr,
    apikey: &RawStr,
    mut conn: MyRocketSQLConn,
) -> Json<GetAccountResponse> {
    let mut params = Vec::new();

    // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
    params.push(id.html_escape().to_mut().clone());
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<Account> =
        conn.prep_exec("SELECT id, apikey, currency_id, title from accounts where id = :id and apikey = :apikey", params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, currency_id, title) = rocket_contrib::databases::mysql::from_row(row);
                    Account {id,apikey,currency_id,title}
                }).collect()
            }).unwrap();

    match vec.len() {
        0 => Json(GetAccountResponse::Error(String::from("record not found"))),
        1 => Json(GetAccountResponse::One((*vec.get(0).unwrap()).clone())), // why is this one different?
        _ => Json(GetAccountResponse::Error(String::from(
            "ID01T Max fubar error. More than one record found. This does not compute.",
        ))),
    }
}

#[rocket::get("/accounts?<apikey>")]
pub fn get_accounts(apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetAccountResponse> {
    // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
    let mut params = Vec::new();
    params.push(apikey.html_escape().to_mut().clone());

    let vec: Vec<AccountDenormalized> =
        conn.prep_exec(r#"
                SELECT
                    accounts.id as id, accounts.apikey as apikey, accounts.title as title,
                    cur.symbol as cur_symbol, cur.title as cur_title,
                    ifnull(ac.category_id, 0) as ac_category_id,
                    ifnull(cat.symbol, '') as cat_symbol,
                    ifnull(cat.title, '') as cat_title

                FROM accounts LEFT JOIN accounts_categories as ac ON ac.account_id = accounts.id
                LEFT JOIN currencies as cur ON accounts.currency_id = cur.id
                LEFT JOIN categories as cat ON ac.category_id = cat.id
                WHERE accounts.apikey = :apikey
                    "#, params )

            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, apikey, title, cur_symbol, cur_title, ac_category_id, cat_symbol, cat_title) = rocket_contrib::databases::mysql::from_row(row);
                    AccountDenormalized { id, apikey, title, cur_symbol, cur_title, ac_category_id, cat_symbol, cat_title}
                }).collect()
            }).unwrap();

    // Now we have a de-normalized vector of AccountDenormalized.  Now normalize this for JSON. One Account may have more than one Category and will thus might be repeated.  However, all rows for a particular account are together so the order of the rows will work in the following code.
    let itr = vec.iter();

    let mut prior_account_id: u32 = 0;
    let mut act_struct = AccountJoined {
        id: 0,
        apikey: String::from(""),
        currency: crate::db::CurrencyShort1 {
            symbol: String::from(""),
            title: String::from(""),
        },
        title: String::from(""),
        categories: Vec::new(),
    };

    let mut ret_vec: Vec<AccountJoined> = Vec::new();
    let mut at_least_one: bool = false;

    for val in itr {
        at_least_one = true;
        if val.id == prior_account_id {
            //    continue same struct, append an acctcat later
        } else {
            //    new struct
            if act_struct.id > 0 {
                ret_vec.push(act_struct.clone());
            }

            let cur = crate::db::CurrencyShort1 {
                symbol: String::from(&val.cur_symbol),
                title: String::from(&val.cur_title),
            };

            prior_account_id = val.id;
            act_struct = AccountJoined {
                id: val.id,
                apikey: String::from(&val.apikey),
                currency: cur,
                title: String::from(&val.title),
                categories: Vec::new(),
            };
        }

        // if acctcat then append to val
        if val.ac_category_id > 0 {
            let n = crate::db::Acctcat2 {
                category_symbol: String::from(&val.cat_symbol),
            };
            act_struct.categories.push(n);
        }
    }
    if at_least_one {
        ret_vec.push(act_struct.clone());
    }

    Json(GetAccountResponse::Many(ret_vec))
}

#[rocket::post("/accounts", data = "<account>")]
pub fn post_account(
    account: rocket::request::Form<AccountShort>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    println!("{:?}", account);

    match conn.prep_exec("INSERT INTO accounts (apikey, currency_id, title) VALUES (:apikey, :currency_id, :title)",(
        &account.apikey, &account.currency_id, &account.title)) {
        Ok(result) => Json(APIResponse::LastInsertId(result.last_insert_id())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string())))
    }
}

#[rocket::put("/accounts", data = "<account>")]
pub fn put_account(
    account: rocket::request::Form<Account>,
    mut conn: MyRocketSQLConn,
) -> Json<APIResponse> {
    match conn.prep_exec("UPDATE accounts SET currency_id = :currency_id, title = :title where id = :id and apikey = :apikey",(
        &account.currency_id, &account.title, &account.id, &account.apikey)) {
        Ok(result) => Json(APIResponse::Info(String::from_utf8_lossy(&result.info()).to_string())),
        Err(err) => Json(APIResponse::Error(String::from(err.to_string()))),
    }
}
