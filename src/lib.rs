#![feature(decl_macro)]
//#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;

pub mod constants;
pub mod db;
pub mod dfp;
pub mod routz;

//use crate::routz::get_linter_categories;

pub mod routes {

    use crate::db::{MyRocketSQLConn, PostApikeysResponse};
    use rocket::http::{ContentType, Status};
    use rocket::request::Request;
    use rocket::response;
    use rocket::response::{Responder, Response};
    use rocket_contrib::json::Json;

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
            json: rocket_contrib::json!({"ping": "bookwerx-core-rust v0.30.0".to_string()}),
            status: Status::Ok,
        }
    }

    #[rocket::post("/apikeys")]
    pub fn post_apikey(mut conn: MyRocketSQLConn) -> Json<PostApikeysResponse> {

        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect();

        match conn.prep_exec(format!("INSERT INTO apikeys (apikey) VALUES ('{}')",rand_string),()) {
            Ok(_) => Json(PostApikeysResponse::Apikey(rand_string)),
            Err(err) => Json(PostApikeysResponse::Error(err.to_string())),
        }
    }
}
