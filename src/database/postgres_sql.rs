use postgres::{Connection, TlsMode};

const POSTGRES_CONN_STRING: &str = "postgres://letznav:letzNav@localhost:5432/rust-local";

pub fn connect_pg() -> Connection {
    Connection::connect(POSTGRES_CONN_STRING, TlsMode::None).unwrap()
}