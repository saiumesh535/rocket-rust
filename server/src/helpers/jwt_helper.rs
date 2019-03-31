extern crate jsonwebtoken as jwt;

use jwt::{encode, Algorithm, Header};
use crate::auth::login::UserLogin;

pub fn encode_jwt(my_claims: UserLogin) ->Result<String, jwt::errors::Error> {
    let key = "secret";
    let mut header = Header::default();
    header.kid = Some("signing_key".to_owned());
    header.alg = Algorithm::HS512;
    match encode(&header, &my_claims, key.as_ref()) {
        Ok(t) => Ok(t),
        Err(e) => Err(e),
    }
}
