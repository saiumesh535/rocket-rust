use rocket_contrib::json::{Json};

#[derive(Serialize, Deserialize)]
pub struct JsonResponse {
    pub username: String
}

#[get("/json")]
pub fn json() -> Json<JsonResponse> {
    Json(
        JsonResponse {
            username: String::from("Hello!! from rust json")
        }
    )
}