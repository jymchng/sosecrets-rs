//! Example taken and tweaked from
//! https://github.com/mikkyang/rust-jwt/blob/master/examples/custom_claims.rs

use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, Verified, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sosecrets_rs::{
    prelude::{
        typenum::{U0, U1},
        Secret,
    },
    traits::ExposeSecret,
};
use std::{env, error::Error};

#[derive(Default, Deserialize, Serialize)]
struct Custom {
    sub: String,
    rhino: bool,
}

pub fn new_token(
    user_id: &str,
    secret_password: Secret<String, U1, U0>,
) -> Result<Secret<String, U1, U0>, Box<dyn Error>> {
    let header: Header = Default::default();
    let claims = Custom {
        sub: user_id.into(),
        rhino: true,
    };
    let unsigned_token = Token::new(header, claims);

    let secret_key = get_secret_key_from_env();

    let (_, signed_token) = secret_key?.expose_secret(move |exposed_key| {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(&*exposed_key.as_bytes()).map_err(|_e| "Invalid key")?;
        let (_, returned_value) = secret_password.expose_secret(|exposed_password| {
            if *exposed_password != "password" {
                return Err("Wrong password used!");
            }
            return Ok(());
        });
        // let (next_secret, returned_value) = secret_password.expose_secret(|exposed_password| {
        //     if *exposed_password != "password" {
        //         return Err("Wrong password used!");
        //     }
        //     return Ok(());
        // });
        // let (next_secret, returned_value) = next_secret.expose_secret(|exposed_password| {
        //     if *exposed_password != "password" {
        //         return Err("Wrong password used!");
        //     }
        //     return Ok(());
        // });
        let _ = returned_value?;
        let signed_token = unsigned_token
            .sign_with_key(&key)
            .map_err(|_e| "Sign error")?;
        Ok::<std::string::String, Box<dyn Error>>(signed_token.into())
    });
    Ok::<Secret<String, U1, U0>, Box<dyn Error>>(Secret::new(signed_token?))
}

pub fn login(token: &str) -> Result<String, Box<dyn Error>> {
    let secret_key = get_secret_key_from_env()?;

    let (_, token) = secret_key.expose_secret(|exposed_secret_key| {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(&*exposed_secret_key.as_bytes()).map_err(|_e| "Invalid key")?;
        let token: Token<Header, Custom, _> =
            VerifyWithKey::verify_with_key(token, &key).map_err(|_e| "Verification failed")?;
        Ok::<Token<Header, Custom, Verified>, Box<dyn Error>>(token)
    });
    let (_, claims) = token?.into();
    Ok(claims.sub)
}

pub(crate) fn get_secret_key_from_env() -> Result<Secret<String, U1>, Box<dyn Error>> {
    Ok(env::var("SECRET_KEY").map(Secret::new)?)
}
