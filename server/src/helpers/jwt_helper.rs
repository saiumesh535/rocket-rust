extern crate jsonwebtoken as jwt;

use jwt::{encode, decode, Algorithm, Header, Validation, TokenData};
use jwt::errors::ErrorKind;
use crate::auth::login::UserLoginTokenInput;

pub fn encode_jwt(my_claims: UserLoginTokenInput) ->Result<String, jwt::errors::Error> {
    let key = "secret";
    let mut header = Header::default();
    header.kid = Some("signing_key".to_owned());
    header.alg = Algorithm::HS512;
    match encode(&header, &my_claims, key.as_ref()) {
        Ok(t) => Ok(t),
        Err(e) => Err(e),
    }
}

pub fn decode_jwt(token: &str) -> Result<TokenData<UserLoginTokenInput>, String> {
    let key = "secret";
    let validation = Validation { validate_exp: false, algorithms:
        vec![Algorithm::HS512], ..Validation::default() };
    match decode::<UserLoginTokenInput>(token, key.as_ref(), &validation) {
        Ok(data) => Ok(data),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => Err("Invalid Token".to_string()),
            ErrorKind::InvalidIssuer => Err("Invalid Issuer".to_string()),
            _ => {
                println!("{:?}", err);
                Err("Some other issue with token".to_string())
            }
        }
    }
}