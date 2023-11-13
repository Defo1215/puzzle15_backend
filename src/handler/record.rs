use crate::database::record;
use crate::error::PuzzleResult;
use crate::handler::auth::JwtUser;
use axum::Json;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreateRecordReq {
    pub event: String,
    pub step_count: i32,
    pub duration: i32,
    pub scramble: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreateRecordResp {
    pub id: i32,
}

pub async fn create_record(
    user: JwtUser,
    Json(req): Json<CreateRecordReq>,
) -> PuzzleResult<Json<CreateRecordResp>> {
    let record = record::insert_record(record::ActiveModel {
        user_id: ActiveValue::Set(user.user_id),
        event: ActiveValue::Set(req.event),
        step_count: ActiveValue::Set(req.step_count),
        duration: ActiveValue::Set(req.duration),
        scramble: ActiveValue::Set(req.scramble),
        ..Default::default()
    })
    .await?;
    Ok(Json(CreateRecordResp { id: record.id }))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ListRecordReq {
    pub event: String,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Record {
    pub id: i32,
    pub user_id: i32,
    pub event: String,
    pub step_count: i32,
    pub duration: i32,
    pub scramble: String,
    pub solution: String,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ListRecordResp {
    pub records: Vec<Record>,
    pub total: u64,
}

pub async fn list_record(
    user: JwtUser,
    Json(req): Json<ListRecordReq>,
) -> PuzzleResult<Json<ListRecordResp>> {
    let total = record::get_user_record_count(user.user_id, &req.event).await?;
    let records = record::get_user_records(user.user_id, &req.event, req.offset, req.limit).await?;
    let records = records
        .into_iter()
        .map(|r| Record {
            id: r.id,
            user_id: r.user_id,
            event: r.event,
            step_count: r.step_count,
            duration: r.duration,
            scramble: r.scramble,
            solution: r.solution,
            updated_at: r.updated_at.timestamp(),
            created_at: r.created_at.timestamp(),
            status: 0,
        })
        .collect();
    Ok(Json(ListRecordResp { records, total }))
}
