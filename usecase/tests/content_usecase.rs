use common::setup::init_db;
use domain::repository_provider::RepositoryProviderInterface;
use infrastructure::repositories::RepositoryProvider;
use std::sync::Arc;
use usecase::{
    logic::content::ContentUseCases,
    model::content::{CreateContentRequestDto, EditContentRequestDto},
};

#[tokio::test]
async fn test_create_content_with_tags_in_memory_db() {
    // Arrange: インメモリDBと実際のRepositoryProviderを準備
    let pool = init_db("sqlite::memory:").await.unwrap();
    let provider = Arc::new(RepositoryProvider::new(pool));
    // providerのクローンを渡すことで、テストスコープでもproviderを使い続けられるようにする
    let use_cases = ContentUseCases::new(provider.clone());

    // `create`メソッドのロジックに合わせてlabelsフィールドを持つDTOを作成
    let dto = CreateContentRequestDto {
        title: "Test Title".to_string(),
        body: "Test Body".to_string(),
        labels: vec!["rust".to_string(), "ddd".to_string()],
    };

    // Act: ユースケースを実行
    let result = use_cases.create(dto).await.unwrap();

    // Assert: 戻り値が正しいことを検証
    assert_eq!(result.id, 1);
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.body, "Test Body");
    // レスポンスに含まれるタグも検証（順不同を考慮）
    assert_eq!(result.tags.len(), 2);
    assert!(result.tags.iter().any(|t| t.label == "rust"));
    assert!(result.tags.iter().any(|t| t.label == "ddd"));

    // Assert: データが実際にDBにコミットされたことを検証
    // 新しいプロバイダを作成する代わりに、テストの開始時に作成したものを再利用する
    let mut uow = provider.begin().await.unwrap();
    let content = uow.content().select(result.id).await.unwrap();

    assert!(
        content.is_some(),
        "保存されたコンテンツが見つかりませんでした"
    );
    let saved_content = content.unwrap();
    assert_eq!(saved_content.id, result.id);
    assert_eq!(saved_content.title, result.title);

    // DBに永続化されたタグと、コンテンツとの関連も検証
    let tag1 = uow.tag().find_by_label("rust").await.unwrap().unwrap();
    let tag2 = uow.tag().find_by_label("ddd").await.unwrap().unwrap();
    let content_tag1 = uow
        .content_tag()
        .select(saved_content.id, tag1.id)
        .await
        .unwrap();
    let content_tag2 = uow
        .content_tag()
        .select(saved_content.id, tag2.id)
        .await
        .unwrap();
    assert!(
        content_tag1.is_some(),
        "content-tag 'rust' relation not found"
    );
    assert!(
        content_tag2.is_some(),
        "content-tag 'ddd' relation not found"
    );
}

#[tokio::test]
async fn test_edit_content_with_tags() {
    // Arrange
    let pool = init_db("sqlite::memory:").await.unwrap();
    let provider = Arc::new(RepositoryProvider::new(pool));
    let use_cases = ContentUseCases::new(provider.clone());

    // 1. 最初にテスト用のコンテンツを作成
    let initial_dto = CreateContentRequestDto {
        title: "Initial Title".to_string(),
        body: "Initial Body".to_string(),
        labels: vec!["rust".to_string(), "ddd".to_string()],
    };
    let created_content = use_cases.create(initial_dto).await.unwrap();

    // 2. 編集用のDTOを作成（title, body, labelsを変更）
    let edit_dto = EditContentRequestDto {
        id: created_content.id,
        title: "Updated Title".to_string(),
        body: "Updated Body".to_string(),
        labels: vec!["rust".to_string(), "axum".to_string()], // "ddd"を削除し、"axum"を追加
    };

    // Act
    let result = use_cases.edit(edit_dto).await.unwrap();

    // Assert: 戻り値の検証
    assert_eq!(result.id, created_content.id);
    assert_eq!(result.title, "Updated Title");
    assert_eq!(result.body, "Updated Body");
    assert_eq!(result.tags.len(), 2);
    assert!(result.tags.iter().any(|t| t.label == "rust"));
    assert!(result.tags.iter().any(|t| t.label == "axum"));
    assert!(
        !result.tags.iter().any(|t| t.label == "ddd"),
        "Tag 'ddd' should have been removed"
    );

    // Assert: DBの状態の検証
    let mut uow = provider.begin().await.unwrap();
    // contentが更新されたか
    let content = uow.content().select(result.id).await.unwrap().unwrap();
    assert_eq!(content.title, "Updated Title");

    // tagとcontent_tagの関連が更新されたか
    let rust_tag = uow.tag().find_by_label("rust").await.unwrap().unwrap();
    let axum_tag = uow.tag().find_by_label("axum").await.unwrap().unwrap();
    let ddd_tag = uow.tag().find_by_label("ddd").await.unwrap().unwrap();

    // 存在するべき関連
    let rust_relation = uow
        .content_tag()
        .select(content.id, rust_tag.id)
        .await
        .unwrap();
    assert!(
        rust_relation.is_some(),
        "Relation with 'rust' tag should exist"
    );
    let axum_relation = uow
        .content_tag()
        .select(content.id, axum_tag.id)
        .await
        .unwrap();
    assert!(
        axum_relation.is_some(),
        "Relation with 'axum' tag should exist"
    );
    // 存在しないべき関連
    let ddd_relation = uow
        .content_tag()
        .select(content.id, ddd_tag.id)
        .await
        .unwrap();
    assert!(
        ddd_relation.is_none(),
        "Relation with 'ddd' tag should have been removed"
    );
}

#[tokio::test]
async fn test_remove_content_success() {
    // Arrange
    let pool = init_db("sqlite::memory:").await.unwrap();
    let provider = Arc::new(RepositoryProvider::new(pool));
    let use_cases = ContentUseCases::new(provider.clone());

    let dto = CreateContentRequestDto {
        title: "To Be Deleted".to_string(),
        body: "...".to_string(),
        labels: vec!["temp".to_string()],
    };
    let created_content = use_cases.create(dto).await.unwrap();

    // Act
    let result = use_cases.remove(created_content.id).await.unwrap();

    // Assert
    assert_eq!(result, 1, "Should return 1 for one deleted row");
    let mut uow = provider.begin().await.unwrap();
    let content = uow.content().select(created_content.id).await.unwrap();
    assert!(content.is_none(), "Content should be deleted from DB");
}

#[tokio::test]
async fn test_remove_non_existent_content_returns_zero() {
    // Arrange
    let pool = init_db("sqlite::memory:").await.unwrap();
    let provider = Arc::new(RepositoryProvider::new(pool));
    let use_cases = ContentUseCases::new(provider.clone());

    // Act: 存在しないコンテンツを削除しようとする
    let result = use_cases.remove(999).await.unwrap();

    // Assert: べき等な操作として成功し、0件の削除が返されるべき
    assert_eq!(result, 0, "Should return 0 for a non-existent content");
}
