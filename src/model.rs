use crate::db::{MyRocketSQLConn};
use std::sync::{Arc, Mutex};


pub struct JunDatabase {
    pub conn: Arc<Mutex<Option<MyRocketSQLConn>>>,
    //pub conn: Arc<Mutex<Option<Box<MyRocketSQLConn>>>>,
}

impl JunDatabase {
    pub fn new() -> JunDatabase {
        JunDatabase {
            conn: Arc::new(Mutex::new(None)),
        }
    }
}
