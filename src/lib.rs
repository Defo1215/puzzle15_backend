pub mod database;
pub mod error;
pub mod handler;

pub async fn ping() -> &'static str {
    "hello"
}
