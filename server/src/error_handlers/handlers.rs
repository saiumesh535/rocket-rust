use rocket::Request;
use rocket_contrib::json::{Json};

#[derive(Serialize, Deserialize)]
pub struct NotFound {
    status: bool,
    message: String,
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<NotFound> {
    Json(NotFound {
        status: false,
        message: format!("looks like you missed here, {}", req.uri())
    })
}
