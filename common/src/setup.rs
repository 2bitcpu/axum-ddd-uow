use crate::types::{BoxError, DbPool};
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;

pub async fn init_db(conn: &str) -> Result<DbPool, BoxError> {
    let options = SqliteConnectOptions::from_str(conn)?.create_if_missing(true);
    let pool = sqlx::SqlitePool::connect_with(options).await?;
    if let Ok(query) = tokio::fs::read_to_string("migrate.sql").await {
        sqlx::query(&query).execute(&pool).await?;
    }
    Ok(pool)
}
