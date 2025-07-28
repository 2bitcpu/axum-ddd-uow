use crate::model::tag::TagEntity;
use async_trait::async_trait;
use common::types::BoxError;

#[async_trait]
pub trait TagInterface: Send {
    async fn create(&mut self, entity: &TagEntity) -> Result<TagEntity, BoxError>;
    async fn select(&mut self, id: i64) -> Result<Option<TagEntity>, BoxError>;
    async fn update(&mut self, entity: &TagEntity) -> Result<Option<TagEntity>, BoxError>;
    async fn delete(&mut self, id: i64) -> Result<u64, BoxError>;
    async fn find_by_label(&mut self, label: &str) -> Result<Option<TagEntity>, BoxError>;
}
