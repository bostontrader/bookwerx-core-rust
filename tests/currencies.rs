// RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test currencies

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use bookwerx_core_rust::db as D;
use bookwerx_core_rust::routes as R;

//use super::rocket;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::config::{Config, Environment};


#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {

    //let mut full_conn = conn_value.to_string();
    let mut full_conn = "mysql://root:supersecretpassword@172.17.0.2:3306".to_string();
    full_conn.push('/');
    //full_conn.push_str(&dbname_value.to_string());
    full_conn.push_str("bookwerx-core-rust-test");

    let mut hm_inner = std::collections::HashMap::new();
    hm_inner.insert("url".to_string(), full_conn);

    let mut hm_outer = std::collections::HashMap::new();
    hm_outer.insert("mysqldb".to_string(), hm_inner);

    let config = Config::build(Environment::Staging)
        //.address(bind_ip_value)
        .address("0.0.0.0")

        .extra("databases",hm_outer)
        //.port(bind_port_value.parse::<u16>().unwrap())
        .port(8000)

        .finalize().unwrap();

    let rocket = rocket::custom(config)
        //.ignite()
        .attach(D::MyRocketSQLConn::fairing())
        .mount("/", routes![R::index, R::get_currencies, R::post_currency]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let req = client.get("/currencies");
    let mut response = req.dispatch();

    let v: serde_json::Value = serde_json::from_str(&(response.body_string().unwrap())[..])?;
    println!("A {:?}",v);
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.as_array().unwrap().len(), 0);

    let req1 = client.post("/currencies")
        //.body("field=value&otherField=123") // 422 unprocessable entity
        .body("symbol=value&title=123")
        .header(ContentType::Form);

    let mut response1 = req1.dispatch();
    println!("B {:?}",response1.body_string());
    assert_eq!(response1.status(), Status::Ok);


    let req2 = client.get("/currencies");
    let mut response2 = req2.dispatch();
    let v2: serde_json::Value = serde_json::from_str(&(response2.body_string().unwrap())[..])?;

    println!("C {:?}",v2);
    assert_eq!(response2.status(), Status::Ok);
    assert_eq!(v2.as_array().unwrap().len(), 1);

    Ok(())
}
