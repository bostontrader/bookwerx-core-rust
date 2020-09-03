use crate::db::MyRocketSQLConn;
use std::sync::{Arc, Mutex};

pub struct JunDatabase(pub Arc<Mutex<Option<MyRocketSQLConn>>>);

impl JunDatabase {
    pub fn new() -> JunDatabase {
        JunDatabase(Arc::new(Mutex::new(None)))
    }
}
