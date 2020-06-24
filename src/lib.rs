#![feature(decl_macro)]
//#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;

pub mod constants;
pub mod db;
pub mod dfp;
pub mod routz;

pub use crate::routz::get_linter_categories;

pub mod routes {

    use rocket::http::{ContentType, RawStr, Status};
    use rocket::request::Request;
    use rocket::response;
    use rocket::response::{Responder, Response};

    impl<'r> Responder<'r> for crate::db::ApiResponseOld {
        fn respond_to(self, req: &Request) -> response::Result<'r> {
            Response::build_from(self.json.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .raw_header("Access-Control-Allow-Origin", "*")
                .ok()
        }
    }

    #[rocket::get("/")]
    pub fn index() -> crate::db::ApiResponseOld {
        crate::db::ApiResponseOld {
            json: rocket_contrib::json!({"ping": "bookwerx-core-rust v0.28.0".to_string()}),
            status: Status::Ok,
        }
    }

    #[rocket::delete("/acctcat/<id>?<apikey>")]
    pub fn delete_acctcat(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponseOld {

        let mut v1  = Vec::new();

        // We receive these arguments as &RawStr.  We must convert them into a form that the mysql parametrization can use.
        v1.push(id.html_escape().to_mut().clone());
        v1.push(apikey.html_escape().to_mut().clone());

        let n = conn.prep_exec("DELETE from accounts_categories where id = :id and apikey = :apikey",v1);

        match n {
            Ok(_result) => crate::db::ApiResponseOld {
                json: rocket_contrib::json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponseOld {
                    json: rocket_contrib::json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }

    #[rocket::get("/acctcat/<id>?<apikey>")]
    pub fn get_acctcat(id: &RawStr, apikey: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponseOld {

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
            crate::db::ApiResponseOld {
                json: rocket_contrib::json!({"error":"record not found"}),
                status: Status::Ok,
            }
        } else if vec.len() == 1 {
            // If we have one, return that
            crate::db::ApiResponseOld {
                json: rocket_contrib::json!(vec.get(0)),
                status: Status::Ok,
            }
        }
        else {
            // If we have more than one, maxfubar error.
            crate::db::ApiResponseOld {
                json: rocket_contrib::json!({"error":"max fubar error. More than one record found. This does not compute."}),
                status: Status::Ok,
            }
        }
    }

    #[rocket::get("/acctcats/for_category?<apikey>&<category_id>")]
    pub fn get_acctcats_for_category(apikey: &RawStr, category_id: &RawStr, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponseOld {

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

        crate::db::ApiResponseOld {
            json: rocket_contrib::json!(vec),
            status: Status::Ok,
        }
    }

    #[rocket::post("/acctcats", data="<acctcat>")]
    pub fn post_acctcat(acctcat: rocket::request::Form<crate::db::AcctcatShort>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponseOld {

        let n = conn.prep_exec("INSERT INTO accounts_categories (apikey, account_id, category_id) VALUES (:apikey, :account_id, :category_id)",(
            &acctcat.apikey, &acctcat.account_id, &acctcat.category_id));
        match n {
            Ok(_result) => crate::db::ApiResponseOld {
                json: rocket_contrib::json!({"data":{"last_insert_id": _result.last_insert_id()}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponseOld {
                    json: rocket_contrib::json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }

    }

    #[rocket::put("/acctcats", data="<acctcat>")]
    pub fn put_acctcat(acctcat: rocket::request::Form<crate::db::Acctcat>, mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponseOld {

        let n = conn.prep_exec("UPDATE accounts_categories SET account_id = :account_id, category_id = :category_id where id = :id and apikey = :apikey",(
            &acctcat.account_id, &acctcat.category_id, &acctcat.id, &acctcat.apikey));

        match n {
            Ok(_result) => crate::db::ApiResponseOld {
                json: rocket_contrib::json!({"data":{"info": String::from_utf8_lossy(&_result.info())}}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponseOld {
                    json: rocket_contrib::json!({"error": _err.to_string()}),
                    status: Status::Ok,
                }
            }
        }
    }



    #[rocket::post("/apikeys")]
    pub fn post_apikey(mut conn: crate::db::MyRocketSQLConn) -> crate::db::ApiResponseOld {

        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect();

        let n = conn.prep_exec(format!("INSERT INTO apikeys (apikey) VALUES ('{}')",rand_string),());

        match n {
            Ok(_result) => crate::db::ApiResponseOld {
                json: rocket_contrib::json!({"apikey": rand_string}),
                status: Status::Ok,
            },
            Err(_err) => {
                crate::db::ApiResponseOld {
                    json: rocket_contrib::json!({"error": _err.to_string()}),
                    status: Status::BadRequest,
                }
            }
        }
    }


}