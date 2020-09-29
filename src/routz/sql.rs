use crate::db::MyRocketSQLConn;

use crate::sql::{ensure_select_statement, execute_query, validate_columns, validate_tables};
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::JsonValue;
use std::collections::{HashMap, HashSet};

#[rocket::get("/sql?<query>&<apikey>")]
pub fn get_query(
    query: &RawStr,
    apikey: &RawStr,
    constraints: State<HashMap<&str, HashSet<&str>>>,
    conn: MyRocketSQLConn,
) -> JsonValue {
    // 1. Tediously extract the string values of the query parameters

    // 1.1 Get the SQL query from the query parameter.  We need to percent_decode because
    // certain important symbols in an SQL query cannot be expressed in a query string.  For
    // example, the '=' sign cannot be used but must be escaped as %3d instead.
    //
    // We will subsequently run this string through a parser and the resulting AST will have to satisfy other
    // constraints, so we don't care about XSS tomfoolery here.
    let query = String::from(query.percent_decode().unwrap().to_owned());

    // 1.2 Get the apikey from the apikey parameter.  Let's hope the apikey generator never creates
    // an apikey using characters that must be % encoded so we won't % decode anything here.
    // We also don't care about html_decoding or other XSS tricks because said key will get quoted and
    // will either match a key in the db or not.  Keep an eye on this part because this smells like trouble.
    let ak = String::from(apikey.as_str());

    // 2. Parse, validate, and execute the query.
    let n = nom_sql::parser::parse_query(String::from(&query))
        .map_err(|e| e.to_string())
        .and_then(ensure_select_statement)
        .and_then(|select| validate_tables(select, &constraints))
        .and_then(|select| validate_columns(Box::new(select), &constraints))
        .and_then(|select| execute_query(select, conn, ak));

    // 3. Respond with either a JsonValue containing the correct query results or build
    // a JSON error message.
    match n {
        Ok(map) => map,
        Err(e) => {
            let mut map = serde_json::Map::new();
            map.insert(
                "error".to_string(),
                serde_json::Value::String(e.to_string()),
            );
            JsonValue(serde_json::Value::from(map))
        }
    }
}
