use postgres::{Connection};
use std::sync::{Arc, Mutex};

pub struct PgState {
    pub connection: Arc<Mutex<Connection>>
}
