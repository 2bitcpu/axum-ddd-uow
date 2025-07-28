use async_trait::async_trait;
use common::types::BoxError;
use domain::interface::content_tag::ContentTagInterface;
use domain::model::content_tag::ContentTagEntity;
use sqlx::SqliteConnection;

/// ContentRepository構造体は、ContentInterfaceの具体的な実装です。
/// データベース接続への可変参照を保持します。
pub struct ContentTagRepository<'a> {
    conn: &'a mut SqliteConnection,
}

// `ContentRepository`を生成するためのヘルパー関数
impl<'a> ContentTagRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> ContentTagInterface for ContentTagRepository<'a> {
    async fn create(&mut self, entity: &ContentTagEntity) -> Result<ContentTagEntity, BoxError> {
        let sql = "INSERT INTO content_tag (content_id, tag_id) VALUES (?, ?) RETURNING *";
        Ok(sqlx::query_as::<_, ContentTagEntity>(sql)
            .bind(&entity.content_id)
            .bind(&entity.tag_id)
            .fetch_one(&mut *self.conn)
            .await?)
    }

    async fn select(
        &mut self,
        content_id: i64,
        tag_id: i64,
    ) -> Result<Option<ContentTagEntity>, BoxError> {
        let sql = "SELECT * FROM content_tag WHERE content_id = ? and tag_id = ?";
        Ok(sqlx::query_as::<_, ContentTagEntity>(sql)
            .bind(content_id)
            .bind(tag_id)
            .fetch_optional(&mut *self.conn)
            .await?)
    }

    async fn delete(&mut self, entity: &ContentTagEntity) -> Result<u64, BoxError> {
        let sql = "DELETE FROM content_tag WHERE content_id = ? and tag_id = ?";
        Ok(sqlx::query(sql)
            .bind(&entity.content_id)
            .bind(&entity.tag_id)
            .execute(&mut *self.conn)
            .await?
            .rows_affected())
    }

    async fn delete_by_content_id(&mut self, content_id: i64) -> Result<u64, BoxError> {
        let sql = "DELETE FROM content_tag WHERE content_id = ?";
        Ok(sqlx::query(sql)
            .bind(content_id)
            .execute(&mut *self.conn)
            .await?
            .rows_affected())
    }

    async fn delete_by_tag_id(&mut self, tag_id: i64) -> Result<u64, BoxError> {
        let sql = "DELETE FROM content_tag WHERE tag_id = ?";
        Ok(sqlx::query(sql)
            .bind(tag_id)
            .execute(&mut *self.conn)
            .await?
            .rows_affected())
    }
}
