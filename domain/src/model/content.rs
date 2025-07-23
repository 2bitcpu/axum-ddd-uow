use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct ContentEntity {
    pub id: i64,
    pub title: String,
    pub body: String,
}
