use crate::database::user;
use crate::error::{PuzzleError, PuzzleResult};
use crate::handler::auth::{generate_jwt, JwtUser};
use axum::Json;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterReq {
    pub username: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterResp {
    pub success: bool,
}

pub async fn register(Json(req): Json<RegisterReq>) -> PuzzleResult<Json<RegisterResp>> {
    user::insert_user(user::ActiveModel {
        username: ActiveValue::Set(req.username),
        nickname: ActiveValue::Set(req.nickname),
        password: ActiveValue::Set(req.password),
        ..Default::default()
    })
    .await?;
    Ok(Json(RegisterResp { success: true }))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginResp {
    pub token: String,
}

pub async fn login(Json(req): Json<LoginReq>) -> PuzzleResult<Json<LoginResp>> {
    let user = match user::get_user(&req.username).await? {
        None => return Err(PuzzleError::Forbidden),
        Some(user) => user,
    };
    if user.password != req.password {
        return Err(PuzzleError::Forbidden);
    }
    Ok(Json(LoginResp {
        token: generate_jwt(&JwtUser {
            user_id: user.id,
            username: user.username,
            expired_at: std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() + 7 * 24 * 3600,
        })?,
    }))
}
