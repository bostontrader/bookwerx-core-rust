#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

pub mod constants;
pub mod db;
pub mod dfp;
pub mod routz;

pub use crate::routz::get_linter_categories;


pub mod routes {

    //pub mod get_linter_categories;

    use rocket::http::{ContentType, RawStr, Status};
    use rocket::request::Request;
    use rocket::response;
    use rocket::response::{Responder, Response};
    // use rocket_contrib::json::JsonValue;


    impl<'r> Responder<'r> for crate::db::ApiResponse {
        fn respond_to(self, req: &Request) -> response::Result<'r> {
            Response::build_from(self.json.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .raw_header("Access-Control-Allow-Origin", "*")
                .ok()
        }
    }

    /*#[get("/linter/categories?<apikey>")]
    pub fn get_linter_categories(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        //v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Linter> =

            conn.prep_exec(r#"
            SELECT c.id, c.symbol, c.title
            FROM categories AS c
            LEFT JOIN accounts_categories AS ac
            ON c.id = ac.category_id
            WHERE c.apikey = :apikey AND ac.category_id IS NULL
            "#, v1)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Linter>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Linter {
                            id: id,
                            symbol: symbol,
                            title: title
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Linter>`
                }).unwrap(); // Unwrap `Vec<Linter>`

        crate::db::ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }*/

    #[get("/")]
    pub fn index() -> crate::db::ApiResponse {
        crate::db::ApiResponse {
            json: json!({"ping": "bookwerx-core-rust v0.24.1".to_string()}),
            status: Status::Ok,
        }
    }

    #[delete("/account/<id>?<apikey>")]
    pub fn delete_account(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let n = conn.prep_exec("DELETE from accounts where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[get("/account/<id>?<apikey>")]
    pub fn get_account(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Account> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from accounts where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, currency_id, rarity, title from accounts where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Account>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, currency_id, rarity, title) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Account {
                            id: id,
                            apikey: apikey,
                            currency_id: currency_id,
                            rarity: rarity,
                            title: title
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Account>`
                }).unwrap(); // Unwrap `Vec<Account>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            crate::db::ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            crate::db::ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            crate::db::ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }
    }

    #[get("/accounts?<apikey>")]
    pub fn get_accounts(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::AccountDenormalized> =

            conn.prep_exec(r#"
                SELECT
                    accounts.id as id, accounts.apikey as apikey, accounts.rarity, accounts.title as title,
                    cur.symbol as cur_symbol, cur.title as cur_title,
                    ifnull(ac.category_id, 0) as ac_category_id,
                    ifnull(cat.symbol, '') as cat_symbol,
                    ifnull(cat.title, '') as cat_title

                FROM accounts LEFT JOIN accounts_categories as ac ON ac.account_id = accounts.id
                LEFT JOIN currencies as cur ON accounts.currency_id = cur.id
                LEFT JOIN categories as cat ON ac.category_id = cat.id
                WHERE accounts.apikey = :apikey
                    "#, v1 )

            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Account>`
                // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `Payment`
                result.map(|x| x.unwrap()).map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow the schema
                    let (id, apikey, rarity, title, cur_symbol, cur_title, ac_category_id, cat_symbol, cat_title) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::AccountDenormalized {
                            id: id,
                            apikey: apikey,
                            rarity: rarity,
                            title: title,
                            cur_symbol: cur_symbol,
                            cur_title: cur_title,
                            ac_category_id: ac_category_id,
                            cat_symbol: cat_symbol,
                            cat_title: cat_title
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Account>`
                }).unwrap(); // Unwrap `Vec<Account>`

        // Now we have a de-normalized vector of AccountDenormalized.  Now normalize this for JSON. One Account may have more than one Category and will thus might be repeated.  However, all rows for a particular account are together so the order of the rows will work in the following code.
        let itr = vec.iter();

        let mut prior_account_id: u32 = 0;
        let mut act_struct = crate::db::AccountJoined {
            id: 0,
            apikey: String::from(""),
            rarity: 0,
            currency: crate::db::CurrencyShort1 {symbol: String::from(""), title: String::from("")},
            title: String::from(""),
            categories: Vec::new()
        };

        let mut ret_vec: Vec<crate::db::AccountJoined> = Vec::new();
        let mut at_least_one: bool = false;

        for val in itr {
            at_least_one = true;
            if val.id == prior_account_id {
                //    continue same struct, append an acctcat later
            }
            else {
                //    new struct
                if act_struct.id > 0 {ret_vec.push(act_struct.clone());}

                let cur = crate::db::CurrencyShort1 {
                    symbol: String::from(&val.cur_symbol),
                    title: String::from(&val.cur_title)
                };

                prior_account_id = val.id;
                act_struct = crate::db::AccountJoined {
                    id: val.id,
                    apikey: String::from(&val.apikey),
                    currency: cur,
                    rarity: val.rarity,
                    title: String::from(&val.title),
                    categories: Vec::new()
                };
            }

            // if acctcat then append to val
            if val.ac_category_id > 0 {
                let n = crate::db::Acctcat2 { category_symbol: String::from(&val.cat_symbol)};
                act_struct.categories.push(n);
            }


        }
        if at_least_one {
            ret_vec.push(act_struct.clone());
        }


        crate::db::ApiResponse {
            json: json!(ret_vec),
            status: Status::Ok,
        }
    }

    #[post("/accounts", data="<account>")]
    pub fn post_account(account: rocket::request::Form<crate::db::AccountShort>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("INSERT INTO accounts (apikey, currency_id, rarity, title) VALUES (:apikey, :currency_id, :rarity, :title)",(
            &account.apikey, &account.currency_id, &account.rarity, &account.title));
        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }

    }

    #[put("/accounts", data="<account>")]
    pub fn put_account(account: rocket::request::Form<crate::db::Account>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("UPDATE accounts SET currency_id = :currency_id, rarity = :rarity, title = :title where id = :id and apikey = :apikey",(
            &account.currency_id, &account.rarity, &account.title, &account.id, &account.apikey));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }


    #[delete("/acctcat/<id>?<apikey>")]
    pub fn delete_acctcat(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let n = conn.prep_exec("DELETE from accounts_categories where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[get("/acctcat/<id>?<apikey>")]
    pub fn get_acctcat(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Acctcat> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from acctcats where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, account_id, category_id from accounts_categories where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Acctcat>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, account_id, category_id) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Acctcat {
                            id: id,
                            apikey: apikey,
                            account_id: account_id,
                            category_id: category_id
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Acctcat>`
                }).unwrap(); // Unwrap `Vec<Acctcat>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            crate::db::ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            crate::db::ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            crate::db::ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }
    }


    #[get("/acctcats/for_category?<apikey>&<category_id>")]
    pub fn get_acctcats_for_category(apikey: &RawStr, category_id: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut params  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        params.push(apikey.html_escape().to_mut().clone());
        params.push(category_id.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Acctcat> =
            conn.prep_exec("SELECT id, apikey, account_id, category_id from accounts_categories where apikey = :apikey and category_id = :category_id", params)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Acctcat>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, account_id, category_id) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Acctcat {
                            id: id,
                            apikey: apikey,
                            account_id: account_id,
                            category_id: category_id
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Acctcat>`
                }).unwrap(); // Unwrap `Vec<Acctcat>`

        crate::db::ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }

    #[post("/acctcats", data="<acctcat>")]
    pub fn post_acctcat(acctcat: rocket::request::Form<crate::db::AcctcatShort>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("INSERT INTO accounts_categories (apikey, account_id, category_id) VALUES (:apikey, :account_id, :category_id)",(
            &acctcat.apikey, &acctcat.account_id, &acctcat.category_id));
        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }

    }

    #[put("/acctcats", data="<acctcat>")]
    pub fn put_acctcat(acctcat: rocket::request::Form<crate::db::Acctcat>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("UPDATE accounts_categories SET account_id = :account_id, category_id = :category_id where id = :id and apikey = :apikey",(
            &acctcat.account_id, &acctcat.category_id, &acctcat.id, &acctcat.apikey));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    
    
    #[post("/apikeys")]
    pub fn post_apikey(mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect();

        let n = conn.prep_exec(format!("INSERT INTO apikeys (apikey) VALUES ('{}')",rand_string),());

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"apikey": rand_string}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::BadRequest,
                }
            }
        }
    }

    #[delete("/category/<id>?<apikey>")]
    pub fn delete_category(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());


        let n = conn.prep_exec("DELETE from categories where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[get("/category/<id>?<apikey>")]
    pub fn get_category(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Category> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from categories where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, symbol, title from categories where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Category>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Category {
                            id: id,
                            apikey: apikey,
                            symbol: symbol,
                            title: title
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Category>`
                }).unwrap(); // Unwrap `Vec<Category>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            crate::db::ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            crate::db::ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            crate::db::ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }




    }

    #[get("/categories?<apikey>")]
    pub fn get_categories(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Category> =
            conn.prep_exec("SELECT id, apikey, symbol, title from categories where apikey = :apikey", v1)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Category>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Category {
                            id: id,
                            apikey: apikey,
                            symbol: symbol,
                            title: title
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Category>`
                }).unwrap(); // Unwrap `Vec<Category>`

        crate::db::ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }

    #[post("/categories", data="<category>")]
    pub fn post_category(category: rocket::request::Form<crate::db::CategoryShort>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("INSERT INTO categories (apikey, symbol, title) VALUES (:apikey, :symbol, :title)",(
            &category.apikey, &category.symbol, &category.title));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[put("/categories", data="<category>")]
    pub fn put_category(category: rocket::request::Form<crate::db::Category>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("UPDATE categories SET symbol = :symbol, title = :title where id = :id and apikey = :apikey",(
            &category.symbol, &category.title, &category.id, &category.apikey));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    
    #[delete("/currency/<id>?<apikey>")]
    pub fn delete_currency(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());


        let n = conn.prep_exec("DELETE from currencies where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[get("/currency/<id>?<apikey>")]
    pub fn get_currency(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Currency> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from currencies where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, rarity, symbol, title from currencies where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Currency>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, rarity, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Currency {
                            id: id,
                            apikey: apikey,
                            rarity: rarity,
                            symbol: symbol,
                            title: title
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Currency>`
                }).unwrap(); // Unwrap `Vec<Currency>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            crate::db::ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            crate::db::ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            crate::db::ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }




    }

    #[get("/currencies?<apikey>")]
    pub fn get_currencies(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Currency> =
            conn.prep_exec("SELECT id, apikey, rarity, symbol, title from currencies where apikey = :apikey", v1)
            .map(|result| { // In this closure we will map `QueryResult` to `Vec<Currency>`
                // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                // will map each `MyResult` to contained `row` (no proper error handling)
                // and second call to `map` will map each `row` to `Payment`
                result.map(|x| x.unwrap()).map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow the schema
                    let (id, apikey, rarity, symbol, title) = rocket_contrib::databases::mysql::from_row(row);
                    crate::db::Currency {
                        id: id,
                        apikey: apikey,
                        rarity: rarity,
                        symbol: symbol,
                        title: title
                    }
                }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Currency>`
            }).unwrap(); // Unwrap `Vec<Currency>`

        crate::db::ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }

    #[post("/currencies", data="<currency>")]
    pub fn post_currency(currency: rocket::request::Form<crate::db::CurrencyShort>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("INSERT INTO currencies (apikey, rarity, symbol, title) VALUES (:apikey, :rarity, :symbol, :title)",(
            &currency.apikey, &currency.rarity, &currency.symbol, &currency.title));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[put("/currencies", data="<currency>")]
    pub fn put_currency(currency: rocket::request::Form<crate::db::Currency>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("UPDATE currencies SET rarity = :rarity, symbol = :symbol, title = :title where id = :id and apikey = :apikey",(
            &currency.rarity, &currency.symbol, &currency.title, &currency.id, &currency.apikey));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }


    #[delete("/distribution/<id>?<apikey>")]
    pub fn delete_distribution(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let n = conn.prep_exec("DELETE from distributions where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[get("/distribution/<id>?<apikey>")]
    pub fn get_distribution(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Distribution> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from distributions where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, account_id, amount, amount_exp, transaction_id from distributions where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Distribution>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, account_id, amount, amount_exp, transaction_id) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Distribution {
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
            crate::db::ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            crate::db::ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            crate::db::ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }
    }

    #[get("/distributions?<apikey>")]
    pub fn get_distributions(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Distribution> =
            conn.prep_exec("SELECT id, account_id, amount, amount_exp, apikey, transaction_id from distributions where apikey = :apikey", v1)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Distribution>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, account_id, amount, amount_exp, apikey, transaction_id) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Distribution {
                            id: id,
                            account_id: account_id,
                            amount: amount,
                            amount_exp: amount_exp,
                            apikey: apikey,
                            transaction_id: transaction_id
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Distribution>`
                }).unwrap(); // Unwrap `Vec<Distribution>`

        crate::db::ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }

    // This is the core functionality of getting the distributions shared by for_tx and for_account
    fn get_distributions_private(query: &str, params: Vec<String>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let vec: Vec<crate::db::DistributionJoined> =
            conn.prep_exec(query, params)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Distribution>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (did, tid, aid, amount, amount_exp, apikey, title, time, notes) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::DistributionJoined {
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


                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Distribution>`
                }).unwrap(); // Unwrap `Vec<Distribution>`

        crate::db::ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }

    #[get("/distributions/for_account?<apikey>&<account_id>")]
    pub fn get_distributions_for_account(apikey: &RawStr, account_id: &RawStr, conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut params  = Vec::new(); // parametrization

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        params.push(apikey.html_escape().to_mut().clone());
        params.push(account_id.html_escape().to_mut().clone());

        get_distributions_private("SELECT d.id as did, t.id as tid, a.id as aid, amount, amount_exp, d.apikey, title, time, notes from distributions as d join transactions as t on d.transaction_id = t.id join accounts as a on d.account_id = a.id where d.apikey = :apikey and account_id = :account_id order by time", params, conn)

    }

    #[get("/distributions/for_tx?<apikey>&<transaction_id>")]
    pub fn get_distributions_for_tx(apikey: &RawStr, transaction_id: &RawStr, conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut params  = Vec::new(); // parametrization

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        params.push(apikey.html_escape().to_mut().clone());
        params.push(transaction_id.html_escape().to_mut().clone());

        get_distributions_private("SELECT d.id as did, t.id as tid, a.id as aid, amount, amount_exp, d.apikey, title, time, notes from distributions as d join transactions as t on d.transaction_id = t.id join accounts as a on d.account_id = a.id where d.apikey = :apikey and transaction_id = :transaction_id order by time", params, conn)

    }

    #[post("/distributions", data="<distribution>")]
    pub fn post_distribution(distribution: rocket::request::Form<crate::db::DistributionShort>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("INSERT INTO distributions (account_id, amount, amount_exp, apikey, transaction_id) VALUES (:account_id, :amount, :amount_exp, :apikey, :transaction_id)",(&distribution.account_id, &distribution.amount, &distribution.amount_exp, &distribution.apikey, &distribution.transaction_id));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }

    }

    #[put("/distributions", data="<distribution>")]
    pub fn put_distribution(distribution: rocket::request::Form<crate::db::Distribution>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("UPDATE distributions SET account_id = :account_id, amount = :amount, amount_exp = :amount_exp, transaction_id = :transaction_id where id = :id and apikey = :apikey",(&distribution.account_id, &distribution.amount, &distribution.amount_exp, &distribution.transaction_id, &distribution.id, &distribution.apikey));


        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }


    #[delete("/transaction/<id>?<apikey>")]
    pub fn delete_transaction(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let n = conn.prep_exec("DELETE from transactions where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[get("/transaction/<id>?<apikey>")]
    pub fn get_transaction(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Transaction> =
            // conn.prep_exec("SELECT id, apikey, symbol, title from transactions where apikey = :apikey", v1)
            conn.prep_exec("SELECT id, apikey, notes, time from transactions where id = :id and apikey = :apikey", v1)

                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Transaction>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, notes, time) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Transaction {
                            id: id,
                            apikey: apikey,
                            notes: notes,
                            time: time
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Transaction>`
                }).unwrap(); // Unwrap `Vec<Transaction>`

        if vec.len() == 0 {
            // If we have zero, return that error.
            crate::db::ApiResponse {
                json: json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            crate::db::ApiResponse {
                json: json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            crate::db::ApiResponse {
                json: json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }




    }

    #[get("/transactions?<apikey>")]
    pub fn get_transactions(apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
        let mut v1  = Vec::new();
        v1.push(apikey.html_escape().to_mut().clone());

        let vec: Vec<crate::db::Transaction> =
            conn.prep_exec("SELECT id, apikey, notes, time from transactions where apikey = :apikey", v1)
                .map(|result| { // In this closure we will map `QueryResult` to `Vec<Transaction>`
                    // `QueryResult` is an iterator over `MyResult<row, err>` so first call to `map`
                    // will map each `MyResult` to contained `row` (no proper error handling)
                    // and second call to `map` will map each `row` to `Payment`
                    result.map(|x| x.unwrap()).map(|row| {
                        // ⚠️ Note that from_row will panic if you don't follow the schema
                        let (id, apikey, notes, time) = rocket_contrib::databases::mysql::from_row(row);
                        crate::db::Transaction {
                            id: id,
                            apikey: apikey,
                            notes: notes,
                            time: time
                        }
                    }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Transaction>`
                }).unwrap(); // Unwrap `Vec<Transaction>`

        crate::db::ApiResponse {
            json: json!(vec),
            status: Status::Ok,
        }
    }

    #[post("/transactions", data="<transaction>")]
    pub fn post_transaction(transaction: rocket::request::Form<crate::db::TransactionShort>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("INSERT INTO transactions (apikey, notes, time) VALUES (:apikey, :notes, :time)",(&transaction.apikey, &transaction.notes, &transaction.time));
        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }

    }

    #[put("/transactions", data="<transaction>")]
    pub fn put_transaction(transaction: rocket::request::Form<crate::db::Transaction>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponse {

        let n = conn.prep_exec("UPDATE transactions SET notes = :notes, time = :time where id = :id and apikey = :apikey",(&transaction.notes, &transaction.time, &transaction.id, &transaction.apikey));

        match n {
            Ok(_result) => crate::db::ApiResponse {
                json: json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponse {
                    json: json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }
    
    
}