use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct ContentTagEntity {
    pub content_id: i64,
    pub tag_id: i64,
}
