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

    use rocket::http::{ContentType, Status};
    use rocket_contrib::json::{Json, JsonValue};
    use rocket::request::Request;
    use rocket::response;
    use rocket::response::{Responder, Response};

    #[derive(Debug)]
    pub struct ApiResponse {
        json: JsonValue,
        status: Status,
    }

    #[derive(Serialize)]
    pub struct Account {
        id: u32,
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

    #[derive(Serialize)]
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

    impl<'r> Responder<'r> for ApiResponse {
        fn respond_to(self, req: &Request) -> response::Result<'r> {
            Response::build_from(self.json.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        }
    }

    #[get("/")]
    pub fn index() -> &'static str {
        "Welcome to bookwerx-core-rust"
    }

    #[get("/accounts")]
    pub fn get_accounts(mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Account>> {

        let vec: Vec<Account> =
            conn.prep_exec("SELECT id, apikey, currency_id, title from accounts", ())
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

    #[get("/currencies")]
    pub fn get_currencies(mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Currency>> {

        let vec: Vec<Currency> =
            conn.prep_exec("SELECT id, apikey, symbol, title from currencies", ())
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


}
