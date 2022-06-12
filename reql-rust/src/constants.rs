pub(crate) const BUFFER_SIZE: usize = 1024;
pub(crate) const NULL_BYTE: u8 = b'\0';
pub(crate) const PROTOCOL_VERSION: usize = 0;
pub(crate) const DEFAULT_AUTHENTICATION_METHOD: &'static str = "SCRAM-SHA-256";
pub(crate) const DATA_SIZE: usize = 4;
pub(crate) const TOKEN_SIZE: usize = 8;
pub(crate) const HEADER_SIZE: usize = DATA_SIZE + TOKEN_SIZE;

pub const DEFAULT_RETHINKDB_HOSTNAME: &'static str = "localhost";
pub const DEFAULT_RETHINKDB_DBNAME: &'static str = "test";
pub const DEFAULT_RETHINKDB_PORT: u16 = 28015;
pub const DEFAULT_RETHINKDB_USER: &'static str = "admin";
pub const DEFAULT_RETHINKDB_PASSWORD: &'static str = "";
pub const MAX_LONGITUDE_VALUE: f64 = 180.;
pub const MAX_LATITUDE_VALUE: f64 = 90.;
