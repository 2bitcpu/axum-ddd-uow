use crate::model::content::ContentEntity;
use async_trait::async_trait;
use common::types::BoxError;

#[async_trait]
pub trait ContentInterface: Send {
    async fn create(&mut self, entity: &ContentEntity) -> Result<ContentEntity, BoxError>;
    async fn select(&mut self, id: i64) -> Result<Option<ContentEntity>, BoxError>;
    async fn update(&mut self, entity: &ContentEntity) -> Result<Option<ContentEntity>, BoxError>;
    async fn delete(&mut self, id: i64) -> Result<u64, BoxError>;
}
