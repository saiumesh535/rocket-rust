use rocket::request::{Form};
use rocket::State;
use crate::state::pg_state::PgState;

#[derive(FromForm)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<user_login>")]
pub fn login_handler(user_login: Form<UserLogin>, state: State<PgState>) -> String {
    let conn = state.connection.lock().unwrap();
    // conn.execute("INSERT INTO users (username, password) VALUES ($1, $2)",
    //              &[&user_login.username, &user_login.password]).unwrap();
    let users = conn.query("SELECT username,password FROM users WHERE username=$1", &[&user_login.username]).unwrap();
    if users.len() == 0 {
        return format!("hey!! check username or password");
    }
    format!("Hello! {}", user_login.username)
}
