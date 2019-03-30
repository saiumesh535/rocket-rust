#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod index;

fn main() {
    rocket::ignite().mount("/", routes![index::index]).launch();
}
