use crate::interface::content::ContentInterface;
use crate::interface::content_tag::ContentTagInterface;
use crate::interface::tag::TagInterface;
use async_trait::async_trait;
use common::types::BoxError;

#[async_trait]
pub trait UnitOfWorkInterface: Send {
    // トランザクションをコミット
    async fn commit(self: Box<Self>) -> Result<(), BoxError>;
    // トランザクションをロールバック
    async fn rollback(self: Box<Self>) -> Result<(), BoxError>;
    // Contentリポジトリを取得
    fn content<'s>(&'s mut self) -> Box<dyn ContentInterface + 's>;
    // Tagリポジトリを取得
    fn tag<'s>(&'s mut self) -> Box<dyn TagInterface + 's>;
    // ContentTagリポジトリを取得
    fn content_tag<'s>(&'s mut self) -> Box<dyn ContentTagInterface + 's>;
}
