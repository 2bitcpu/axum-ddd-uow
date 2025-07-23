use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct TagEntity {
    pub id: i64,
    pub label: String,
}
