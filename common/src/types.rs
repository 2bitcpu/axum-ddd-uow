pub type DbPool = sqlx::SqlitePool;
pub type DbExecutor = sqlx::SqliteConnection;
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
