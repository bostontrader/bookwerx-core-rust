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



        //match m2 {
            //None => println!("none"),
            //Some(conn) => {
                //#[rocket::get("/currencies?<apikey>")]
                //pub fn get_currencies(apikey: &RawStr, mut conn: MyRocketSQLConn) -> Json<GetCurrencyResponse> {
                // We receive apikey as &RawStr.  We must convert it into a form that the mysql parametrization can use.
                let mut params= Vec::new();
                params.push("catfood".to_string());

                //params.push(apikey.html_escape().to_mut().clone());
                //let vec: Vec<Currency> =
                //let n = (*m1).prep_exec("SELECT id, apikey, rarity, symbol, title from currencies where apikey = :apikey", params);
                //.map(|result| {
                //result.map(|x| x.unwrap()).map(|row| {
                //Currency {id,apikey,rarity,symbol,title}
                //}).collect()
                //}).unwrap();
                //Json(GetCurrencyResponse::Many(vec))
                //}
            //},
        //}

        42
    }



}
