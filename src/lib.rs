pub mod constants {

    pub const CARGO_BIN :&str = "bookwerx-core-rust";
    pub const CONN_KEY_ENV :&str = "BCR_CONN";
    pub const CONN_KEY_CLI :&str = "conn";
    pub const DB_KEY_ENV :&str = "BCR_DB";
    pub const DB_KEY_CLI :&str = "db";
    pub const INIT_KEY_ENV :&str = "BCR_INIT";
    pub const INIT_KEY_CLI :&str = "init";

    pub const MYSQL_SEED_FILE :&str = "dbseed.sql";
    pub const INVALID_SEED_FILE :&str = "tests/invalid-seed.sql";
    pub const TEST_CONN_STR :&str = "mysql://root:supersecretpassword@172.17.0.2:3306";
    pub const TEST_DB_NAME :&str = "bookwerx-core-rust-test";

}