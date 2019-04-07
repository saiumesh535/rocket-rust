use crate::state::pg_state::PgState;
use rocket_contrib::json::{Json};
use crate::gaurds::authorization::ApiKey;

use rocket::State;

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub username: String
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub users: Vec<Users>
}

#[get("/users")]
pub fn get_all_users(state: State<PgState>, _api: ApiKey) -> Json<UserResponse> {
    let mut users_data: Vec<Users> = Vec::new();
    let conn = state.connection.lock().unwrap();
    let lol = conn.query("SELECT username,password FROM users", &[]).unwrap();
    for user in lol.into_iter() {
        users_data.push(Users {
            username: user.get("username")
        });
    };
    Json(
        UserResponse {
            users: users_data
        }
    )
}