#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

mod index;
mod error_handlers;
mod json_example;
mod form_example;
mod auth;
mod state;
mod database;

use error_handlers::handlers;
use auth::login;
use state::test_state::MyConfig;
use database::postgres_sql;
use state::pg_state::PgState;
use std::{sync::Mutex};

fn main() {

    let lol = MyConfig {
        user_val: String::from("Hey!!")
    };

    let pg_conn = PgState {
        connection: Mutex::new(postgres_sql::connect_pg())
    };

    rocket::ignite()
        .mount("/", routes![
            index::index,
            json_example::json,
            form_example::user_login,
            form_example::form_error,
            login::login_handler
        ])
        .manage(lol)
        .manage(pg_conn)
        .register(catchers![handlers::not_found])
        .launch();
}
