#![feature(decl_macro)]

pub mod constants;
pub mod db;
pub mod dfp;
pub mod routz;
pub mod sql;

pub mod routes {

    use crate::db::{MyRocketSQLConn, Ping, PostApikeysResponse, Semver};
    use rocket::http::ContentType;
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
    pub fn index() -> Json<Ping> {
        Json(Ping {
            about: "bookwerx-core-rust".to_string(),
            url: "https://github.com/bostontrader/bookwerx-core-rust".to_string(),
            v: Semver {
                // VERSION
                major: 0,
                minor: 5,
                patch: 0,
            },
        })
    }

    #[rocket::post("/apikeys")]
    pub fn post_apikey(mut conn: MyRocketSQLConn) -> Json<PostApikeysResponse> {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        match conn.prep_exec(
            format!("INSERT INTO apikeys (apikey) VALUES ('{}')", rand_string),
            (),
        ) {
            Ok(_) => Json(PostApikeysResponse::Apikey(rand_string)),
            Err(err) => Json(PostApikeysResponse::Error(err.to_string())),
        }
    }
}
