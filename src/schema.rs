use crate::model::{JunDatabase};

pub struct Query;

#[juniper::object(
    Context = JunDatabase
    Scalar = juniper::DefaultScalarValue,
)]

// The root query object of the schema
impl Query {

    // curl -X POST -H "Content-Type: application/json" -d '{"query": "{ hello }"}' http://localhost:3003/graphql
    fn hello() -> i32 {
        42
    }

    fn currencies(database: &JunDatabase) -> i32 {

        let mut params= Vec::new();
        params.push("catfood".to_string());

        let m = &*database.conn;
        let mut m1 = m.lock().unwrap();
        //let m2 = m1.as_mut();
        let m2 = m1.as_ref();
        let mut m3 = m2.unwrap();
        //(*m3).get_conn();
        //let m1 = m.lock().unwrap().as_ref().unwrap();
        //let m4 = m3.prep_exec("SELECT id, apikey, rarity, symbol, title from currencies where apikey = :apikey", params);


        42
    }



}
