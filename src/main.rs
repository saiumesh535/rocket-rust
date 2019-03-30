#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

mod index;
mod error_handlers;
mod json_example;

use error_handlers::handlers;

fn main() {
    rocket::ignite()
        .mount("/", routes![index::index, json_example::json])
        .register(catchers![handlers::not_found])
        .launch();
}