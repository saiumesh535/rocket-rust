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

use error_handlers::handlers;
use auth::login;
use state::test_state::MyConfig;

fn main() {

    let lol = MyConfig {
        user_val: String::from("Hey!!")
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
        .register(catchers![handlers::not_found])
        .launch();
}