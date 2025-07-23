use crate::model::content::{CreateContentRequestDto, CreateContentResponseDto};
use common::types::BoxError;
use derive_new::new;
use domain::repository_provider::RepositoryProviderInterface;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct ContentUseCases<R: RepositoryProviderInterface> {
    provider: Arc<R>,
}

impl<R: RepositoryProviderInterface> ContentUseCases<R> {
    pub async fn post(
        &self,
        dto: CreateContentRequestDto,
    ) -> Result<CreateContentResponseDto, BoxError> {
        let mut uow = self.provider.begin().await?;
        let entity = uow.content().create(&dto.to_entity()).await?;
        uow.commit().await?;
        Ok(CreateContentResponseDto::from_entity(entity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::content::CreateContentRequestDto;
    use common::setup::init_db;
    use infrastructure::repositories::RepositoryProvider;

    #[tokio::test]
    async fn test_post_content_with_in_memory_db() {
        // Arrange: インメモリDBと実際のRepositoryProviderを準備
        // `common::setup::init_db` を使ってテスト用のDBプールを作成
        let pool = init_db("sqlite::memory:").await.unwrap();
        // 実際のRepositoryProviderをインスタンス化
        let provider = Arc::new(RepositoryProvider::new(pool.clone()));
        let use_cases = ContentUseCases::new(provider);
        let dto = CreateContentRequestDto {
            title: "Test Title".to_string(),
            body: "Test Body".to_string(),
        };

        // Act: ユースケースを実行
        let result = use_cases.post(dto).await.unwrap();

        // Assert: 戻り値が正しいことを検証
        assert_eq!(result.id, 1);
        assert_eq!(result.title, "Test Title");
        assert_eq!(result.body, "Test Body");

        // Assert: データが実際にDBにコミットされたことを検証
        // 別のUoWを開始して、書き込まれたデータを読み出す
        let verification_provider = RepositoryProvider::new(pool);
        let mut uow = verification_provider.begin().await.unwrap();
        let content = uow.content().select(result.id).await.unwrap();

        assert!(
            content.is_some(),
            "保存されたコンテンツが見つかりませんでした"
        );
        let saved_content = content.unwrap();
        assert_eq!(saved_content.id, result.id);
        assert_eq!(saved_content.title, result.title);
    }
}
