use crate::db::MyRocketSQLConn;
use std::sync::{Arc, Mutex};

pub struct JunDatabase(pub Arc<Mutex<Option<MyRocketSQLConn>>>);

impl JunDatabase {
    pub fn new(conn: Option<MyRocketSQLConn>) -> JunDatabase {
        JunDatabase(Arc::new(Mutex::new(conn)))
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Account {
    id: i32,
    symbol: String,
    desc: String,
}

// Assign Database as the context type for User
//#[juniper::graphql_object(
//Context = JunDatabase,
//)]
impl Account {
    // 3. Inject the context by specifying an argument
    //    with the context type.
    // Note:
    //   - the type must be a reference
    //   - the name of the argument SHOULD be context
    //fn friends(&self, context: &Database) -> Vec<&User> {

        // 5. Use the database to lookup users
        //self.friend_ids.iter()
            //.map(|id| context.users.get(id).expect("Could not find user with ID"))
            //.collect()
    //}

    //fn name(&self) -> &str {
        //self.name.as_str()
    //}

    fn symbol(&self) -> String {
        String::from("42")
    }

    fn id(&self) -> i32 {
        self.id
    }
}



#[derive(juniper::GraphQLObject)]
pub struct Category {
    id: i32,
    symbol: String,
    desc: String,
}