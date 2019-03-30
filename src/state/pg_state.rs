use postgres::{Connection};
use std::{sync::Mutex};

pub struct PgState {
    pub connection: Mutex<Connection>
}