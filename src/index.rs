use rocket::State;
use crate::state::test_state::MyConfig;


#[get("/")]
pub fn index(state: State<MyConfig>) -> String {
    format!("Hello {}", state.user_val)
}