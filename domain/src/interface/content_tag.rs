use crate::model::content_tag::ContentTagEntity;
use async_trait::async_trait;
use common::types::BoxError;

#[rustfmt::skip]
#[async_trait]
pub trait ContentTagInterface {
    async fn create(&mut self, entity: &ContentTagEntity) -> Result<ContentTagEntity, BoxError>;
    async fn delete(&mut self, entity: &ContentTagEntity) -> Result<u64, BoxError>;
}
