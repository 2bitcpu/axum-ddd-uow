use crate::model::content_tag::ContentTagEntity;
use async_trait::async_trait;
use common::types::BoxError;

#[rustfmt::skip]
#[async_trait]
pub trait ContentTagInterface {
    async fn create(&mut self, entity: &ContentTagEntity) -> Result<ContentTagEntity, BoxError>;
    async fn select(&mut self, content_id: i64, tag_id: i64) -> Result<Option<ContentTagEntity>, BoxError>;
    async fn delete(&mut self, entity: &ContentTagEntity) -> Result<u64, BoxError>;
    async fn delete_by_content_id(&mut self, content_id: i64) -> Result<u64, BoxError>;
    async fn delete_by_tag_id(&mut self, tag_id: i64) -> Result<u64, BoxError>;
}
