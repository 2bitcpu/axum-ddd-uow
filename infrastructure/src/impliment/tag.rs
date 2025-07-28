use async_trait::async_trait;
use common::types::BoxError;
use domain::interface::tag::TagInterface;
use domain::model::tag::TagEntity;
use sqlx::SqliteConnection;

/// ContentRepository構造体は、ContentInterfaceの具体的な実装です。
/// データベース接続への可変参照を保持します。
pub struct TagRepository<'a> {
    conn: &'a mut SqliteConnection,
}

// `ContentRepository`を生成するためのヘルパー関数
impl<'a> TagRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> TagInterface for TagRepository<'a> {
    async fn create(&mut self, entity: &TagEntity) -> Result<TagEntity, BoxError> {
        let sql = "INSERT INTO tag (label) VALUES (?) RETURNING *";
        Ok(sqlx::query_as::<_, TagEntity>(sql)
            .bind(&entity.label)
            .fetch_one(&mut *self.conn)
            .await?)
    }

    async fn select(&mut self, id: i64) -> Result<Option<TagEntity>, BoxError> {
        let sql = "SELECT * FROM tag WHERE id = ?";
        Ok(sqlx::query_as::<_, TagEntity>(sql)
            .bind(id)
            .fetch_optional(&mut *self.conn)
            .await?)
    }

    async fn update(&mut self, entity: &TagEntity) -> Result<Option<TagEntity>, BoxError> {
        let sql = "UPDATE tag SET label = ? WHERE id = ? RETURNING *";
        Ok(sqlx::query_as::<_, TagEntity>(sql)
            .bind(&entity.label)
            .bind(&entity.id)
            .fetch_optional(&mut *self.conn)
            .await?)
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BoxError> {
        let sql = "DELETE FROM tag WHERE id = ?";
        Ok(sqlx::query(sql)
            .bind(id)
            .execute(&mut *self.conn)
            .await?
            .rows_affected())
    }

    async fn find_by_label(&mut self, label: &str) -> Result<Option<TagEntity>, BoxError> {
        let sql = "SELECT * FROM tag WHERE label = ?";
        Ok(sqlx::query_as::<_, TagEntity>(sql)
            .bind(label)
            .fetch_optional(&mut *self.conn)
            .await?)
    }
}
