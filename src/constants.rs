pub const BIND_IP_KEY_ENV: &str = "BCR_BIND_IP";
pub const BIND_IP_KEY_CLI: &str = "bind_ip";

pub const BIND_PORT_KEY_ENV: &str = "BCR_BIND_PORT";
pub const BIND_PORT_KEY_CLI: &str = "bind_port";

pub const CONN_KEY_ENV: &str = "BCR_CONN";
pub const CONN_KEY_CLI: &str = "conn";

pub const DBNAME_KEY_ENV: &str = "BCR_DBNAME";
pub const DBNAME_KEY_CLI: &str = "dbname";

pub const MODE_KEY_ENV: &str = "BCR_MODE";
pub const MODE_KEY_CLI: &str = "mode";

pub const SEED_KEY_ENV: &str = "BCR_SEED";
pub const SEED_KEY_CLI: &str = "seed";

// We also have some constants solely for testing.
pub const TEST_BIND_IP: &str = "0.0.0.0";
pub const TEST_BIND_PORT: u16 = 8000;
pub const TEST_CONN: &str = "mysql://root:supersecretpassword@localhost:3306";
pub const TEST_DBNAME: &str = "bookwerx-core-rust-test";
