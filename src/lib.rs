#![feature(decl_macro)]
//#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;

pub mod constants;
pub mod db;
pub mod dfp;
pub mod routz;

pub use crate::routz::get_linter_categories;

pub mod routes {

    use rocket::http::{ContentType, Status};
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
            json: rocket_contrib::json!({"ping": "bookwerx-core-rust v0.29.0".to_string()}),
            status: Status::Ok,
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
