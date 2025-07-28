use common::types::BoxError;
use domain::repository_provider::RepositoryProviderInterface;
use std::sync::Arc;

#[derive(Clone)]
pub struct TagUseCases {
    provider: Arc<dyn RepositoryProviderInterface + Send + Sync>,
}

impl TagUseCases {
    pub fn new(provider: Arc<dyn RepositoryProviderInterface + Send + Sync>) -> Self {
        Self { provider }
    }

    pub async fn remove(&self, id: i64) -> Result<u64, BoxError> {
        let mut uow = self.provider.begin().await?;
        let count = uow.tag().delete(id).await?;
        // タグが削除された場合のみ、関連するcontent_tagも削除する
        if count > 0 {
            uow.content_tag().delete_by_tag_id(id).await?;
        }
        // トランザクションをコミットする
        uow.commit().await?;
        Ok(count)
    }

    pub async fn remove_label(&self, label: String) -> Result<u64, BoxError> {
        let mut uow = self.provider.begin().await?;
        let tag_entity = uow.tag().find_by_label(&label).await?;

        if let Some(tag) = tag_entity {
            let count = uow.tag().delete(tag.id).await?;
            // タグが削除された場合のみ、関連するcontent_tagも削除する
            if count > 0 {
                uow.content_tag().delete_by_tag_id(tag.id).await?;
            }
            // トランザクションをコミットする
            uow.commit().await?;
            Ok(count)
        } else {
            // タグが存在しない場合は何もせず、0を返す
            // uowはここでdropされ、トランザクションはロールバックされる（実質何もしていない）
            Ok(0)
        }
    }
}
