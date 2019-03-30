#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod index;
mod error_handlers;

use error_handlers::handlers;

fn main() {
    rocket::ignite()
        .mount("/", routes![index::index])
        .register(catchers![handlers::not_found])
        .launch();
}