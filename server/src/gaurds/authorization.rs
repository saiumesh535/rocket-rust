use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use crate::helpers::jwt_helper::decode_jwt;

#[derive(Debug)]
pub struct ApiKey(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct UnAuthorized {
    status: bool,
    message: String,
}


impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = String;
    // not handling those custom erros now
    // will be adding in future.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, Self::Error> {
        let keys: Vec<_> = request.headers().get("token").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, "NO token Provided".to_string()));
        }
        let token_data = decode_jwt(&keys[0]);
        if token_data.is_err() {
            return Outcome::Failure((Status::BadRequest, "token is invali".to_string()));
        }
        return Outcome::Success(ApiKey(token_data.unwrap().claims.username));
    }
}