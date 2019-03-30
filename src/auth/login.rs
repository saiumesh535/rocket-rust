use rocket::request::{Form};

#[derive(FromForm)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<user_login>")]
pub fn login_handler(user_login: Form<UserLogin>) -> String {
    format!("Hello! {}", user_login.username)
}