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
mod helpers;

use rocket_cors;
use rocket::http::Method;
use error_handlers::handlers;
use auth::login;
use state::test_state::MyConfig;
use database::postgres_sql;
use state::pg_state::PgState;
use std::{sync::Mutex};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use rocket::response::Responder;

fn cors_options() -> CorsOptions {
    // let allowed_origins = AllowedOrigins::all();
     let allowed_origins =
        AllowedOrigins::some(&["http://localhost:4200/"], &["^https://(.+).acme.com$"]);

    // You can also deserialize this
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Access-Control-Allow-Methods"]),
        allow_credentials: true,
        expose_headers: ["Content-Type", "X-Custom", "application/json"]
            .iter()
            .map(ToString::to_string)
            .collect(),
        max_age: Some(42),
        send_wildcard: false,
        fairing_route_base: "/".to_string(),
        fairing_route_rank: 0,
    }
}

#[options("/login")]
fn login_options<'r>() -> impl Responder<'r> {
    let options = cors_options().to_cors()?;
    options.respond_owned(|guard| guard.responder(()))
}


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
            login::login_handler,
            login_options,
        ])
        .manage(lol)
        .manage(pg_conn)
        .manage(cors_options().to_cors().expect("To not fail"))
        .register(catchers![handlers::not_found])
        .launch();
}
