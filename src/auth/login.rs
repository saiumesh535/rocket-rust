use rocket::request::{Form};
use rocket::State;
use crate::state::test_state::MyConfig;

#[derive(FromForm)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<user_login>")]
pub fn login_handler(user_login: Form<UserLogin>, state: State<MyConfig>) -> String {
    format!("Hello! {} and {}", user_login.username, state.user_val)
}