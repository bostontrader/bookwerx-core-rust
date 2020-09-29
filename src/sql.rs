use std::boxed::Box;

#[cfg(test)]
use std::collections::{HashMap, HashSet};

// Given an API key and a table.field, build a ConditionExpression that is
// suitable for inclusion in a WHERE clause
pub fn build_apikey_clause(
    apikey: String,
    field_name: String,
    table_name: String,
) -> nom_sql::ConditionExpression {
    nom_sql::ConditionExpression::ComparisonOp(nom_sql::ConditionTree {
        operator: nom_sql::Operator::Equal,
        left: Box::new(nom_sql::ConditionExpression::Base(
            nom_sql::ConditionBase::Field(nom_sql::Column {
                name: field_name,
                alias: None,
                table: Some(table_name),
                function: None,
            }),
        )),
        right: Box::new(nom_sql::ConditionExpression::Base(
            nom_sql::ConditionBase::Literal(nom_sql::Literal::String(apikey)),
        )),
    })
}

pub fn execute_query(
    mut select: Box<nom_sql::SelectStatement>,
    mut conn: crate::db::MyRocketSQLConn,
    apikey: String,
) -> Result<rocket_contrib::json::JsonValue, String> {
    // 1. Modify the select to include the apikey
    match &select.where_clause {
        // There is an existing WHERE clause A.  Modify to be WHERE apikey=... AND A.
        Some(ce1) => {
            let ce2 = crate::sql::build_apikey_clause(
                apikey,
                String::from("apikey"),
                String::from(&select.tables[0].name),
            );
            let ct1 = nom_sql::ConditionTree {
                operator: nom_sql::Operator::And,
                left: Box::new(ce2.clone()),
                right: Box::new(ce1.clone()),
            };
            let ce3 = nom_sql::ConditionExpression::LogicalOp(ct1);
            select.where_clause = Some(ce3);
        }
        None => {
            // There is no existing where clause.  Append a new one.
            let wc = crate::sql::build_apikey_clause(
                apikey,
                String::from("apikey"),
                String::from(&select.tables[0].name),
            );
            select.where_clause = Some(wc);
        }
    }

    // Now execute the query for real.
    match conn.prep_exec(select.to_string(), Vec::<String>::new()) {
        Ok(query_results) => {
            // 1. Get column info.  Get it now because we have borrowing and ownership issues if we try to access
            // this column info directly later.
            let columns: &[mysql::Column] = &query_results.columns_ref();
            let mut col_names = Vec::new();
            for c in columns {
                col_names.push(format!("{}.{}", c.table_str(), c.name_str()));
            }

            // 2. We need to build a vector of Value that will contain the info from each row.
            // Init that here.
            let mut vec: Vec<serde_json::Value> = Vec::new();

            // 3. Iterate over all the rows.
            for row_result in query_results {
                match row_result {
                    Ok(row) => {
                        // 3.1 We will build a serde_json::Map to contain the keys and values that we
                        // pick from the row_result.  Init that here.
                        let mut map = serde_json::Map::new();

                        // 3.2 Retrieve the row values into a more usable form.  This "unwrap" does this.  Not
                        // to be confused with the same named method for Result or Option.
                        let row_values: Vec<mysql_common::value::Value> = row.unwrap();

                        // 3.3 Ensure that the quantity of row values is the same as the quantity of column headers.
                        if row_values.len() != col_names.len() {
                            return Err(String::from("The quantity of row values is not the same as the quantity of column headers."));
                        }

                        // 3.4 Iterate over all the row_values and populate the row Map that will ultimately be sent back
                        // to the caller.  We need a col_idx to access the column names associated with each value.
                        let mut col_idx = 0;
                        for value in row_values {
                            match value {
                                mysql_common::value::Value::Bytes(b) => {
                                    map.insert(
                                        col_names[col_idx].clone(),
                                        serde_json::Value::String(String::from_utf8(b).unwrap()),
                                    );
                                }
                                mysql_common::value::Value::Int(i) => {
                                    map.insert(
                                        col_names[col_idx].clone(),
                                        serde_json::Value::Number(serde_json::Number::from(i)),
                                    );
                                }
                                _ => {
                                    return Err(String::from(
                                        "I have encountered an SQL type that I don't recognize.",
                                    ));
                                }
                            };
                            col_idx = col_idx + 1;
                        }
                        vec.push(serde_json::value::Value::Object(map));
                    }
                    Err(_) => {
                        return Err(String::from(
                            "Mysterious error iterating over the SQL result set.",
                        ));
                    }
                }
            }

            Ok(rocket_contrib::json::JsonValue(serde_json::Value::from(
                vec,
            )))
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

// The query must only be a SELECT
pub fn ensure_select_statement(
    ast: nom_sql::parser::SqlQuery,
) -> Result<Box<nom_sql::SelectStatement>, String> {
    match ast {
        nom_sql::SqlQuery::Select(s) => Ok(Box::new(s)),
        _ => Err(String::from("The query must only be a SELECT.")),
    }
}

// Ensure that all column types are of type Col and that all column names from the query
// are present in the constraints.
pub fn validate_columns(
    select: Box<nom_sql::SelectStatement>,
    constraints: &std::collections::HashMap<&str, std::collections::HashSet<&str>>,
) -> Result<Box<nom_sql::SelectStatement>, String> {
    for fde in &(*select).fields {
        match fde {
            nom_sql::FieldDefinitionExpression::Col(column) => {

                match &column.table {
                    Some(table_name) => {
                        let table_constraints = constraints.get::<str>(&table_name);
                        match table_constraints {
                            Some(field_constraints) => {
                                match field_constraints.get::<str>(&column.name) {
                                    Some(_) => {
                                        // do nothing
                                    },
                                    None => return Err(format!("Column '{}' is not present in the constraints for table '{}'", column.name, table_name))
                                }
                            }
                            None => return Err(format!("Table '{}' is not present in the constraints.", table_name))
                        }
                    },
                    None => return Err(format!("Column '{}' does not have an explicit table name.",column.name))
                }
            },
            _ => return Err(format!("All columns must be ordinary columns, not * or expressions.  Column '{}' fails this test.", fde.to_string()))
        }
    }

    Ok(Box::new(*select))
}

// Ensure that all tables names from the query are present in the constraints.
pub fn validate_tables(
    select: Box<nom_sql::SelectStatement>,
    constraints: &std::collections::HashMap<&str, std::collections::HashSet<&str>>,
) -> Result<nom_sql::SelectStatement, String> {
    for table in &(*select).tables {
        let table_constraints = constraints.get::<str>(&table.name);

        match table_constraints {
            Some(_) => { /* do nothing */ }
            None => {
                return Err(format!(
                    "Table '{}' is not present in the constraints.",
                    &table.name
                ))
            }
        }
    }

    Ok(*select)
}

#[test]
// If the original query does not have any where clause, create a suitable one
fn modify_where_clause_a() {
    let wc = build_apikey_clause(
        String::from("catfood"),
        String::from("apikey"),
        String::from("accounts"),
    );

    let mut n = nom_sql::parser::parse_query(String::from("SELECT id FROM accounts"));
    match n {
        Ok(ref mut ast) => {
            match ast {
                nom_sql::SqlQuery::Select(ref mut s) => {
                    s.where_clause = Some(wc);
                }
                _ => { /* do nothing */ }
            }
            assert_eq!(
                ast.to_string(),
                "SELECT id FROM accounts WHERE accounts.apikey = 'catfood'"
            );
        }
        Err(_) => assert_eq!(true, false),
    };
}

#[test]
// If the original query has an existing where clause A, create a 2nd clause B
// and amend the original clause to: WHERE B and A
// Put the new fragment first because we don't want SQL injection to eliminate it.
fn modify_where_clause_b() {
    let ce2 = build_apikey_clause(
        String::from("catfood"),
        String::from("apikey"),
        String::from("accounts"),
    );

    let mut n = nom_sql::parser::parse_query(String::from("SELECT id FROM accounts WHERE true"));
    match n {
        Ok(ref mut ast) => {
            match ast {
                nom_sql::SqlQuery::Select(ref mut s) => {
                    match &s.where_clause {
                        Some(ce1) => {
                            let ct1 = nom_sql::ConditionTree {
                                operator: nom_sql::Operator::And,
                                left: Box::new(ce2.clone()),
                                right: Box::new(ce1.clone()),
                            };

                            let ce3 = nom_sql::ConditionExpression::LogicalOp(ct1);

                            s.where_clause = Some(ce3);
                        }
                        None => assert_eq!(true, false),
                    };
                }
                _ => assert_eq!(true, false),
            }
            assert_eq!(
                ast.to_string(),
                "SELECT id FROM accounts WHERE accounts.apikey = 'catfood' AND true"
            );
        }
        Err(_) => assert_eq!(true, false),
    };
}

#[test]
fn test() {
    // 1. Unparseable SQL
    assert_eq!(
        nom_sql::parser::parse_query(String::from("ddrop table accounts")),
        Err("failed to parse query")
    );

    // 2. The query is not a SELECT statement.
    assert_eq!(
        nom_sql::parser::parse_query(String::from("drop table accounts"))
            .map_err(|e| e.to_string())
            .and_then(ensure_select_statement),
        Err(String::from("The query must only be a SELECT."))
    );

    // 3. Establish some constraints
    let mut fields = HashSet::new();
    fields.insert("id");

    let mut constraints = HashMap::new();
    constraints.insert("accounts", fields);

    // 4. The given table name is not present in the constraints.
    assert_eq!(
        nom_sql::parser::parse_query(String::from("select * from customers"))
            .map_err(|e| e.to_string())
            .and_then(ensure_select_statement)
            .and_then(|select| { validate_tables(select, &constraints) }),
        Err(format!(
            "Table 'customers' is not present in the constraints."
        ))
    );

    // 5. All columns must be ordinary columns.  Not * or expressions.
    assert_eq!(
        nom_sql::parser::parse_query(String::from("select 42 from accounts"))
            .map_err(|e| e.to_string())
            .and_then(ensure_select_statement)
            .and_then(
                |select| { validate_tables(select, &constraints) }
            ).and_then(
            |select| { validate_columns(Box::new(select), &constraints) }
            ),
        Err(String::from("All columns must be ordinary columns, not * or expressions.  Column '42' fails this test."))
    );

    assert_eq!(
        nom_sql::parser::parse_query(String::from("select * from accounts"))
            .map_err(|e| e.to_string())
            .and_then(ensure_select_statement)
            .and_then(
                |select| { validate_tables(select, &constraints) }
            ).and_then(
            |select| { validate_columns(Box::new(select), &constraints) }
        ),
        Err(String::from("All columns must be ordinary columns, not * or expressions.  Column '*' fails this test."))
    );

    // 6. All columns must be present in the constraints.

    // 6.1 No explicit table specified for the column.
    assert_eq!(
        nom_sql::parser::parse_query(String::from("select catfood from accounts"))
            .map_err(|e| e.to_string())
            .and_then(ensure_select_statement)
            .and_then(|select| { validate_tables(select, &constraints) })
            .and_then(|select| { validate_columns(Box::new(select), &constraints) }),
        Err(String::from(
            "Column 'catfood' does not have an explicit table name."
        ))
    );

    // 6.2 Explicit table name not present in the constraints.
    assert_eq!(
        nom_sql::parser::parse_query(String::from("select catfood.yum from accounts"))
            .map_err(|e| e.to_string())
            .and_then(ensure_select_statement)
            .and_then(|select| { validate_tables(select, &constraints) })
            .and_then(|select| { validate_columns(Box::new(select), &constraints) }),
        Err(format!(
            "Table 'catfood' is not present in the constraints."
        ))
    );

    // 6.3 The column name is not present in the constraints for the given table.
    assert_eq!(
        nom_sql::parser::parse_query(String::from("select accounts.yum from accounts"))
            .map_err(|e| e.to_string())
            .and_then(ensure_select_statement)
            .and_then(|select| { validate_tables(select, &constraints) })
            .and_then(|select| { validate_columns(Box::new(select), &constraints) }),
        Err(format!(
            "Column 'yum' is not present in the constraints for table 'accounts'"
        ))
    );
}
