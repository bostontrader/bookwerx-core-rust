#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;

pub mod constants {

    pub const BIND_IP_KEY_ENV: &str = "BCR_BIND_IP";
    pub const BIND_IP_KEY_CLI: &str = "bind_ip";

    pub const BIND_PORT_KEY_ENV: &str = "BCR_BIND_PORT";
    pub const BIND_PORT_KEY_CLI: &str = "bind_port";

    pub const CONN_KEY_ENV: &str = "BCR_CONN";
    pub const CONN_KEY_CLI: &str = "conn";

    pub const DBNAME_KEY_ENV: &str = "BCR_DBNAME";
    pub const DBNAME_KEY_CLI: &str = "dbname";

    pub const MODE_KEY_ENV: &str = "BCR_MODE";
    pub const MODE_KEY_CLI: &str = "mode";

    pub const SEED_KEY_ENV: &str = "BCR_SEED";
    pub const SEED_KEY_CLI: &str = "seed";

    // We also have some constants solely for testing.
    pub const TEST_BIND_IP: &str = "0.0.0.0";
    pub const TEST_BIND_PORT: u16 = 8000;
    pub const TEST_CONN: &str = "mysql://root:supersecretpassword@172.17.0.2:3306";
    pub const TEST_DBNAME: &str = "bookwerx-core-rust-test";
}

pub mod db {

    use rocket_contrib::databases::mysql;
    #[database("mysqldb")]
    pub struct MyRocketSQLConn(mysql::Conn);

}

pub mod routes {

    use rocket::http::{ContentType, RawStr, Status};
    use rocket::request::Request;
    use rocket::response;
    use rocket::response::{Responder, Response};
    use rocket_contrib::json::{Json, JsonValue};

    #[derive(Serialize)]
    pub struct About {
        about: String
    }

    #[derive(Debug)]
    pub struct ApiResponse {
        json: JsonValue,
        status: Status,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Account {
        pub id: u32,
        apikey: String,
        currency_id: u32,
        title: String
    }

    #[derive(FromForm)]
    pub struct AccountShort {
        apikey: String,
        currency_id: u32,
        title: String
    }

    #[derive(Deserialize)]
    pub struct Apikey { pub apikey: String }

    #[derive(Deserialize, Serialize)]
    pub struct Currency {
        id: u32,
        apikey: String,
        symbol: String,
        title: String
    }

    #[derive(FromForm)]
    pub struct CurrencyShort {
        apikey: String,
        symbol: String,
        title: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Distribution {
        id: u32,
        account_id: u32,
        amount: i64,
        amount_exp: i8,
        apikey: String,
        transaction_id: u32
    }

    #[derive(FromForm)]
    pub struct DistributionShort {
        account_id: u32,
        amount: i64,
        amount_exp: i8,
        apikey: String,
        transaction_id: u32
    }

    #[derive(Deserialize, Serialize)]
    pub struct Transaction {
        pub id: u32,
        apikey: String,
        notes: String,
        time: String
    }

    #[derive(FromForm)]
    pub struct TransactionShort {
        apikey: String,
        notes: String,
        time: String
    }

    impl<'r> Responder<'r> for ApiResponse {
        fn respond_to(self, req: &Request) -> response::Result<'r> {
            Response::build_from(self.json.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        }
    }

    #[get("/")]
    pub fn index() -> Json<About> {
        Json(About{about:"bookwerx-core-rust v0.6.0".to_string()})
    }

    #[get("/accounts?<apikey>")]
    pub fn get_accounts(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Account>> {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql paramterization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        //let vec: Vec<Currency> =
            //conn.prep_exec("SELECT id, apikey, symbol, title from currencies where apikey = :apikey", v1)

        let vec: Vec<Account> =
            conn.prep_exec("SELECT id, apikey, currency_id, title from accounts where apikey = :apikey", v1)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, currency_id, title) = rocket_contrib::databases::mysql::from_row(row);
                        Account {
                            id: id,
                            apikey: apikey,
                            currency_id: currency_id,
                            title: title
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
                }).unwrap(); // Unwrap `Vec<Payment>`

        Json(vec)
    }


    #[post("/accounts", data="<account>")]
    pub fn post_account(account: rocket::request::Form<AccountShort>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("INSERT INTO accounts (apikey, currency_id, title) VALUES (:apikey, :currency_id, :title)",(&account.apikey, &account.currency_id, &account.title));
        match n {
            Ok(_result) => ApiResponse {
                json: json!({"last_insert_id": _result.last_insert_id()}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::BadRequest,
                }
            }
        }

    }

    #[post("/apikeys")]
    pub fn post_apikey(mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect();

        let n = conn.prep_exec(format!("INSERT INTO apikeys (apikey) VALUES ('{}')",rand_string),());

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"apikey": rand_string}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::BadRequest,
                }
            }
        }
    }

    #[get("/currencies?<apikey>")]
    pub fn get_currencies(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Currency>> {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql paramterization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Currency> =
            conn.prep_exec("SELECT id, apikey, symbol, title from currencies where apikey = :apikey", v1)
            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
                // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `Payment`
                result.map(|x| x.unwrap()).map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow the schema
                    let (id, apikey, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    Currency {
                        id: id,
                        apikey: apikey,
                        symbol: symbol,
                        title: title
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
            }).unwrap(); // Unwrap `Vec<Payment>`

        Json(vec)
    }

    #[post("/currencies", data="<currency>")]
    pub fn post_currency(currency: rocket::request::Form<CurrencyShort>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("INSERT INTO currencies (apikey, symbol, title) VALUES (:apikey, :symbol, :title)",(&currency.apikey, &currency.symbol, &currency.title));

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"last_insert_id": _result.last_insert_id()}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::BadRequest,
                }
            }
        }
    }

    #[get("/distributions?<apikey>&<transaction_id>")]
    pub fn get_distributions(apikey: &RawStr, transaction_id: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Distribution>> {
        let vec: Vec<Distribution> =
            conn.prep_exec("SELECT id, account_id, amount, amount_exp, apikey, transaction_id from distributions", ())
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, account_id, amount, amount_exp, apikey, transaction_id) = rocket_contrib::databases::mysql::from_row(row);
                        Distribution {
                            id: id,
                            account_id: account_id,
                            amount: amount,
                            amount_exp: amount_exp,
                            apikey: apikey,
                            transaction_id: transaction_id
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
                }).unwrap(); // Unwrap `Vec<Payment>`

        Json(vec)
    }

    #[post("/distributions", data="<distribution>")]
    pub fn post_distribution(distribution: rocket::request::Form<DistributionShort>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("INSERT INTO distributions (account_id, amount, amount_exp, apikey, transaction_id) VALUES (:account_id, :amount, :amount_exp, :apikey, :transaction_id)",(&distribution.account_id, &distribution.amount, &distribution.amount_exp, &distribution.apikey, &distribution.transaction_id));

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"last_insert_id": _result.last_insert_id()}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::BadRequest,
                }
            }
        }

    }

    #[get("/transactions?<apikey>")]
    pub fn get_transactions(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Transaction>> {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql paramterization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Transaction> =
            conn.prep_exec("SELECT id, apikey, notes, time from transactions where apikey = :apikey", v1)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, notes, time) = rocket_contrib::databases::mysql::from_row(row);
                        Transaction {
                            id: id,
                            apikey: apikey,
                            notes: notes,
                            time: time
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
                }).unwrap(); // Unwrap `Vec<Payment>`

        Json(vec)
    }


    #[post("/transactions", data="<transaction>")]
    pub fn post_transaction(transaction: rocket::request::Form<TransactionShort>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("INSERT INTO transactions (apikey, notes, time) VALUES (:apikey, :notes, :time)",(&transaction.apikey, &transaction.notes, &transaction.time));
        match n {
            Ok(_result) => ApiResponse {
                json: json!({"last_insert_id": _result.last_insert_id()}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::BadRequest,
                }
            }
        }

    }

}
