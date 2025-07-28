use common::setup::init_db;
use domain::repository_provider::RepositoryProviderInterface;
use infrastructure::repositories::RepositoryProvider;
use std::sync::Arc;
use usecase::{
    logic::{content::ContentUseCases, tag::TagUseCases},
    model::content::CreateContentRequestDto,
};

// Helper function to set up the test environment
async fn setup() -> (
    Arc<RepositoryProvider>,
    ContentUseCases<RepositoryProvider>,
    TagUseCases<RepositoryProvider>,
) {
    let pool = init_db("sqlite::memory:").await.unwrap();
    let provider = Arc::new(RepositoryProvider::new(pool));
    let content_use_cases = ContentUseCases::new(provider.clone());
    let tag_use_cases = TagUseCases::new(provider.clone());
    (provider, content_use_cases, tag_use_cases)
}

#[tokio::test]
async fn test_remove_tag_by_id_success() {
    // Arrange: コンテンツとタグを作成
    let (provider, content_use_cases, tag_use_cases) = setup().await;
    let dto = CreateContentRequestDto {
        title: "Test".to_string(),
        body: "...".to_string(),
        labels: vec!["tag_to_delete".to_string(), "another_tag".to_string()],
    };
    let created_content = content_use_cases.create(dto).await.unwrap();
    let tag_to_delete = created_content
        .tags
        .iter()
        .find(|t| t.label == "tag_to_delete")
        .unwrap();

    // Act: IDでタグを削除
    let result = tag_use_cases.remove(tag_to_delete.id).await.unwrap();

    // Assert: 戻り値の検証
    assert_eq!(result, 1, "Should return 1 for one deleted row");

    // Assert: DBの状態の検証
    let mut uow = provider.begin().await.unwrap();
    // タグが削除されたか
    let should_be_none = uow.tag().select(tag_to_delete.id).await.unwrap();
    assert!(should_be_none.is_none(), "Tag should be deleted");

    // 関連が削除されたか
    let relation_should_be_none = uow
        .content_tag()
        .select(created_content.id, tag_to_delete.id)
        .await
        .unwrap();
    assert!(
        relation_should_be_none.is_none(),
        "Tag relation should be deleted"
    );

    // 他のタグやコンテンツは影響を受けない
    let another_tag = created_content
        .tags
        .iter()
        .find(|t| t.label == "another_tag")
        .unwrap();
    let another_tag_should_exist = uow.tag().select(another_tag.id).await.unwrap();
    assert!(
        another_tag_should_exist.is_some(),
        "Other tags should not be affected"
    );
    let content_should_exist = uow.content().select(created_content.id).await.unwrap();
    assert!(
        content_should_exist.is_some(),
        "Content should not be affected"
    );
}

#[tokio::test]
async fn test_remove_tag_by_label_success() {
    // Arrange: コンテンツとタグを作成
    let (provider, content_use_cases, tag_use_cases) = setup().await;
    let dto = CreateContentRequestDto {
        title: "Test".to_string(),
        body: "...".to_string(),
        labels: vec!["label_to_delete".to_string()],
    };
    let created_content = content_use_cases.create(dto).await.unwrap();
    let tag_to_delete = created_content.tags.first().unwrap();

    // Act: ラベルでタグを削除
    let result = tag_use_cases
        .remove_label("label_to_delete".to_string())
        .await
        .unwrap();

    // Assert: 戻り値の検証
    assert_eq!(result, 1, "Should return 1 for one deleted row");

    // Assert: DBの状態の検証
    let mut uow = provider.begin().await.unwrap();
    let should_be_none = uow.tag().select(tag_to_delete.id).await.unwrap();
    assert!(should_be_none.is_none(), "Tag should be deleted");
    let relation_should_be_none = uow
        .content_tag()
        .select(created_content.id, tag_to_delete.id)
        .await
        .unwrap();
    assert!(
        relation_should_be_none.is_none(),
        "Tag relation should be deleted"
    );
}

#[tokio::test]
async fn test_remove_unassociated_tag_success() {
    // Arrange: どのコンテンツにも関連付けられていないタグを作成
    let (provider, _, tag_use_cases) = setup().await;
    let tag_id;
    {
        let mut uow = provider.begin().await.unwrap();
        let tag = uow
            .tag()
            .create(&domain::model::tag::TagEntity {
                id: 0,
                label: "unassociated".to_string(),
            })
            .await
            .unwrap();
        tag_id = tag.id;
        uow.commit().await.unwrap();
    }

    // Act: タグを削除
    let result = tag_use_cases.remove(tag_id).await.unwrap();

    // Assert: 戻り値の検証
    assert_eq!(result, 1, "Should return 1 for one deleted row");

    // Assert: DBの状態の検証
    let mut uow = provider.begin().await.unwrap();
    let should_be_none = uow.tag().select(tag_id).await.unwrap();
    assert!(should_be_none.is_none(), "Tag should be deleted");
}

#[tokio::test]
async fn test_remove_non_existent_tag_by_id_returns_zero() {
    // Arrange
    let (_, _, tag_use_cases) = setup().await;

    // Act
    let result = tag_use_cases.remove(999).await.unwrap();

    // Assert
    assert_eq!(result, 0, "Should return 0 for a non-existent tag");
}

#[tokio::test]
async fn test_remove_non_existent_tag_by_label_returns_zero() {
    // Arrange
    let (_, _, tag_use_cases) = setup().await;

    // Act
    let result = tag_use_cases
        .remove_label("non_existent_label".to_string())
        .await
        .unwrap();

    // Assert
    assert_eq!(result, 0, "Should return 0 for a non-existent tag");
}
