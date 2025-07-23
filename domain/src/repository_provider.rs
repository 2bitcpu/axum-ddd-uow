use crate::unit_of_work::UnitOfWorkInterface;
use async_trait::async_trait;
use common::types::BoxError;

/// RepositoryProviderInterfaceは、UnitOfWorkのファクトリとして機能するインターフェースです。
#[async_trait]
pub trait RepositoryProviderInterface: Send + Sync {
    /// 新しいUnit of Work（トランザクション）を開始します。
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkInterface + '_>, BoxError>;
}
