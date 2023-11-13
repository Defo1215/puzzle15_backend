use super::sea::get_conn;
use crate::error::PuzzleResult;
use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    pub user_id: i32,
    pub event: String,
    pub step_count: i32,
    pub duration: i32,
    pub scramble: String,
    pub solution: String,
    pub status: i32,

    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn insert_record(record: ActiveModel) -> PuzzleResult<Model> {
    let conn = get_conn().await;
    record.insert(conn).await.map_err(Into::into)
}

pub async fn get_user_records(
    user_id: i32,
    event: &str,
    offset: u64,
    limit: u64,
) -> PuzzleResult<Vec<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Event.eq(event))
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await
        .map_err(Into::into)
}

pub async fn get_user_record_count(user_id: i32, event: &str) -> PuzzleResult<u64> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Event.eq(event))
        .count(conn)
        .await
        .map_err(Into::into)
}
