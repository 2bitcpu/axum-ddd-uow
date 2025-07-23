use async_trait::async_trait;
use common::types::BoxError;
use domain::interface::content::ContentInterface;
use domain::model::content::ContentEntity;
use sqlx::SqliteConnection;

/// ContentRepository構造体は、ContentInterfaceの具体的な実装です。
/// データベース接続への可変参照を保持します。
pub struct ContentRepository<'a> {
    conn: &'a mut SqliteConnection,
}

// `ContentRepository`を生成するためのヘルパー関数
impl<'a> ContentRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> ContentInterface for ContentRepository<'a> {
    async fn create(&mut self, entity: &ContentEntity) -> Result<ContentEntity, BoxError> {
        let sql = "INSERT INTO content (title, body) VALUES (?, ?) RETURNING *";
        Ok(sqlx::query_as::<_, ContentEntity>(sql)
            .bind(&entity.title)
            .bind(&entity.body)
            .fetch_one(&mut *self.conn)
            .await?)
    }

    async fn select(&mut self, id: i64) -> Result<Option<ContentEntity>, BoxError> {
        let sql = "SELECT * FROM content WHERE id = ?";
        Ok(sqlx::query_as::<_, ContentEntity>(sql)
            .bind(id)
            .fetch_optional(&mut *self.conn)
            .await?)
    }

    async fn update(&mut self, entity: &ContentEntity) -> Result<Option<ContentEntity>, BoxError> {
        let sql = "UPDATE content SET title = ?, body = ? WHERE id = ? RETURNING *";
        Ok(sqlx::query_as::<_, ContentEntity>(sql)
            .bind(&entity.title)
            .bind(&entity.body)
            .bind(&entity.id)
            .fetch_optional(&mut *self.conn)
            .await?)
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BoxError> {
        let sql = "DELETE FROM content WHERE id = ?";
        Ok(sqlx::query(sql)
            .bind(id)
            .execute(&mut *self.conn)
            .await?
            .rows_affected())
    }
}
