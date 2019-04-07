use rocket::request::{Form};
use rocket::State;
use crate::state::pg_state::PgState;
use crate::helpers::jwt_helper::encode_jwt;
use rocket_contrib::json::{Json};

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct UserLoginTokenInput {
    pub username: String
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginToken {
    pub token: String
}

#[post("/login", data = "<user_login>")]
pub fn login_handler(user_login: Form<UserLogin>, state: State<PgState>) -> Result<Json<UserLoginToken>, String> {
    let conn = state.connection.lock().unwrap();
    // conn.execute("INSERT INTO users (username, password) VALUES ($1, $2)",
    //              &[&user_login.username, &user_login.password]).unwrap();
    let users = conn.query("SELECT username,password FROM users WHERE username=$1", &[&user_login.username]).unwrap();
    if users.len() == 0 {
        return Err(format!("hey!! check username or password"));
    }
    let jwt_claims = UserLoginTokenInput {
        username: user_login.username.to_owned()
    };
    let token = encode_jwt(jwt_claims);
    if token.is_ok() {
        Ok(Json(
            UserLoginToken {
                token: token.unwrap()
            }
        ))
    } else {
        Err(format!("something went wrong"))
    }
}
