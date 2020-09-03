use crate::model::{JunDatabase};

pub struct Query;

#[juniper::graphql_object(
    Context = JunDatabase
    Scalar = juniper::DefaultScalarValue,
)]

// The root query object of the schema
impl Query {

    // curl -X POST -H "Content-Type: application/json" -d '{"query": "{ hello }"}' http://localhost:3003/graphql
    fn hello(&self) -> i32 {
        42
    }

    fn currencies(&self, database: &JunDatabase) -> i32 {

        let mut params= Vec::new();
        params.push("catfood".to_string());

        let mut m = database.0.lock().unwrap();
        let m = m.as_deref_mut().unwrap();
        let _ = m.prep_exec("SELECT id, apikey, rarity, symbol, title from currencies where apikey = :apikey", params);


        42
    }



}
