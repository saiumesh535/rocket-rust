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

use error_handlers::handlers;
use auth::login;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index::index,
            json_example::json,
            form_example::user_login,
            form_example::form_error,
            login::login_handler
        ])
        .register(catchers![handlers::not_found])
        .launch();
}