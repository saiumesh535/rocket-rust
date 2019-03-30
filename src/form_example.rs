use rocket::request::{Form};
use rocket_contrib::json::{Json};

use crate::json_example::JsonResponse;

#[derive(FromForm)]
pub struct UserLogin {
    username: String,
}

#[derive(FromForm)]
pub struct FormError {
    username: String,
}

#[post("/formlogin", data = "<user>")]
pub fn user_login(user: Form<UserLogin>) -> String {
    format!("Welcome to Rocket-Crud Example {}", user.username)
}

// error handling
#[post("/formError", data = "<user>")]
pub fn form_error(user: Form<FormError>) -> Result<Json<JsonResponse>, String> {
    if user.username.len() < 3 {
        Err(format!("username {} cannot be less than 3", user.username))
    } else {
        Ok(Json(
            JsonResponse {
                username: format!("Welcome {}", user.username)
            }
        ))
    }
}