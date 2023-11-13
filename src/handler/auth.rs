use crate::error::{PuzzleError, PuzzleResult};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{
    async_trait,
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtUser {
    pub user_id: i32,
    pub username: String,
    pub expired_at: u64,
}

pub fn generate_jwt(jwt_user: &JwtUser) -> PuzzleResult<String> {
    encode(
        &Header::default(),
        &jwt_user,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(Into::into)
}

#[async_trait]
impl<B> FromRequestParts<B> for JwtUser
where
    B: Send + Sync,
{
    type Rejection = PuzzleError;

    async fn from_request_parts(parts: &mut Parts, state: &B) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| PuzzleError::Forbidden)?;

        // Decode the user data
        let token_data = decode::<JwtUser>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| PuzzleError::Forbidden)?;

        if token_data.claims.expired_at > std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() {
            return Err(PuzzleError::Forbidden);
        }

        Ok(token_data.claims)
    }
}
