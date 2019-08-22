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
    pub struct Ping {
        ping: String
    }

    #[derive(Debug)]
    pub struct ApiResponse {
        json: JsonValue,
        status: Status,
    }

    #[derive(Debug, Deserialize)]
    pub struct ApiError {
        pub error: String
    }

    #[derive(Debug, Deserialize)]
    pub struct InsertMessage {
        pub last_insert_id: u32
    }

    #[derive(Debug, Deserialize)]
    pub struct InsertSuccess {
        pub data: InsertMessage
    }

    #[derive(Debug, Deserialize)]
    pub struct UpdateMessage {
        pub info: String
    }

    #[derive(Debug, Deserialize)]
    pub struct UpdateSuccess {
        pub data: UpdateMessage
    }


    #[derive(Deserialize, FromForm, Serialize)]
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

    #[derive(Debug, Deserialize, FromForm, Serialize)]
    pub struct Currency {
        pub id: u32,
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

    #[derive(Deserialize, FromForm, Serialize)]
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

    #[derive(Deserialize, FromForm, Serialize)]
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
                .raw_header("Access-Control-Allow-Origin", "*")
                .ok()
        }
    }

    #[get("/")]
    pub fn index() -> ApiResponse {
        ApiResponse {
            json: json!({"ping": "bookwerx-core-rust v0.8.0".to_string()}),
            status: Status::Ok,
        }
    }


    #[get("/accounts?<apikey>")]
    pub fn get_accounts(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Account>> {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql paramterization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Account> =
            conn.prep_exec("SELECT id, apikey, currency_id, title from accounts where apikey = :apikey", v1)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Account>`
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
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Account>`
                }).unwrap(); // Unwrap `Vec<Account>`

        Json(vec)
    }

    #[get("/account/<id>?<apikey>")]
    pub fn get_account(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql paramterization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Account> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from accounts where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, currency_id, title from accounts where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Account>`
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
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Account>`
                }).unwrap(); // Unwrap `Vec<Account>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }




    }

    #[post("/accounts", data="<account>")]
    pub fn post_account(account: rocket::request::Form<AccountShort>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("INSERT INTO accounts (apikey, currency_id, title) VALUES (:apikey, :currency_id, :title)",(&account.apikey, &account.currency_id, &account.title));
        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }

    }

    #[put("/accounts", data="<account>")]
    pub fn put_account(account: rocket::request::Form<Account>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("UPDATE accounts SET currency_id = :currency_id, title = :title where id = :id and apikey = :apikey",(&account.currency_id, &account.title, &account.id, &account.apikey));

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
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
    pub fn get_currencies(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql paramterization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Currency> =
            conn.prep_exec("SELECT id, apikey, symbol, title from currencies where apikey = :apikey", v1)
            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Currency>`
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
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Currency>`
            }).unwrap(); // Unwrap `Vec<Currency>`

        ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }


    #[get("/currency/<id>?<apikey>")]
    pub fn get_currency(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql paramterization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Currency> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from currencies where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, symbol, title from currencies where id = :id and apikey = :apikey", v1)

            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Currency>`
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
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Currency>`
                }).unwrap(); // Unwrap `Vec<Currency>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }




    }

    #[post("/currencies", data="<currency>")]
    pub fn post_currency(currency: rocket::request::Form<CurrencyShort>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("INSERT INTO currencies (apikey, symbol, title) VALUES (:apikey, :symbol, :title)",(&currency.apikey, &currency.symbol, &currency.title));

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }


    #[delete("/currency/<id>?<apikey>")]
    pub fn delete_currency(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql paramterization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());


        let n = conn.prep_exec("DELETE from currencies where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }


    #[put("/currencies", data="<currency>")]
    pub fn put_currency(currency: rocket::request::Form<Currency>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("UPDATE currencies SET symbol = :symbol, title = :title where id = :id and apikey = :apikey",(&currency.symbol, &currency.title, &currency.id, &currency.apikey));

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[get("/distribution/<id>?<apikey>")]
    pub fn get_distribution(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql paramterization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Distribution> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from distributions where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, account_id, amount, amount_exp, transaction_id from distributions where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Distribution>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, account_id, amount, amount_exp, transaction_id) = rocket_contrib::databases::mysql::from_row(row);
                        Distribution {
                            id: id,
                            apikey: apikey,
                            account_id: account_id,
                            amount: amount,
                            amount_exp: amount_exp,
                            transaction_id: transaction_id
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Distribution>`
                }).unwrap(); // Unwrap `Vec<Distribution>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }




    }

    #[get("/distributions?<apikey>&<transaction_id>")]
    //pub fn get_distributions(apikey: &RawStr, transaction_id: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> Json<Vec<Distribution>> {
    pub fn get_distributions(apikey: &RawStr, transaction_id: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql paramterization can use.
        v1.push(apikey.html_escape().to_mut().clone());
        v1.push(transaction_id.html_escape().to_mut().clone());

        let vec: Vec<Distribution> =
            conn.prep_exec("SELECT id, account_id, amount, amount_exp, apikey, transaction_id from distributions where apikey = :apikey and transaction_id = :transaction_id", v1)
            //conn.prep_exec("SELECT id, apikey, symbol, title from currencies where apikey = :apikey", v1)

            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Distribution>`
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
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Distribution>`
                }).unwrap(); // Unwrap `Vec<Distribution>`

        ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
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
                    status: Status::Ok,
                }
            }
        }

    }

    #[put("/distributions", data="<distribution>")]
    pub fn put_distribution(distribution: rocket::request::Form<Distribution>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("UPDATE distributions SET account_id = :account_id, amount = :amount, amount_exp = :amount_exp, transaction_id = :transaction_id where id = :id and apikey = :apikey",(&distribution.account_id, &distribution.amount, &distribution.amount_exp, &distribution.transaction_id, &distribution.id, &distribution.apikey));


        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
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
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Transaction>`
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
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Transaction>`
                }).unwrap(); // Unwrap `Vec<Transaction>`

        Json(vec)
    }

    #[get("/transaction/<id>?<apikey>")]
    pub fn get_transaction(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql paramterization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<Transaction> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from transactions where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, notes, time from transactions where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Transaction>`
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
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Transaction>`
                }).unwrap(); // Unwrap `Vec<Transaction>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }




    }
    
    
    #[post("/transactions", data="<transaction>")]
    pub fn post_transaction(transaction: rocket::request::Form<TransactionShort>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("INSERT INTO transactions (apikey, notes, time) VALUES (:apikey, :notes, :time)",(&transaction.apikey, &transaction.notes, &transaction.time));
        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }

    }

    #[put("/transactions", data="<transaction>")]
    pub fn put_transaction(transaction: rocket::request::Form<Transaction>, mut conn: crate::db::MyRocketSQLConn) -> ApiResponse {

        let n = conn.prep_exec("UPDATE transactions SET notes = :notes, time = :time where id = :id and apikey = :apikey",(&transaction.notes, &transaction.time, &transaction.id, &transaction.apikey));

        match n {
            Ok(_result) => ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }
    
    
}
