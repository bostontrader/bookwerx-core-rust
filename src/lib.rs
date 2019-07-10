#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

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
}

pub mod db {

    use rocket_contrib::databases::mysql;
    #[database("mysqldb")]
    pub struct MyRocketSQLConn(mysql::Conn);

}

pub mod routes {

    #[get("/")]
    pub fn index() -> &'static str {
        "Hello, world!"
    }

    #[get("/accounts")]
    pub fn get_accounts() -> &'static str {
        "Get all accounts"
    }

    #[post("/accounts")]
    pub fn post_account() -> &'static str {
        "Post new account"
    }

    #[get("/currencies")]
    pub fn get_currencies() -> &'static str {
        "Get all currencies"
    }

    #[derive(FromForm)]
    pub struct Currency {
        symbol: String,
        title: String,
    }

}
