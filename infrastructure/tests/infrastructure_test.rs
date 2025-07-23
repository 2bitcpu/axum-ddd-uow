use {
    common::{setup::init_db, types::BoxError},
    domain::{
        model::{content::ContentEntity, content_tag::ContentTagEntity, tag::TagEntity},
        repository_provider::RepositoryProviderInterface,
    },
    infrastructure::repositories::RepositoryProvider,
};

/// 各テストのために、新しいインメモリDBと`RepositoryProvider`をセットアップするヘルパー関数
async fn setup() -> Box<dyn RepositoryProviderInterface> {
    // "sqlite::memory:" を使うことで、テストごとに完全に独立したDBが作成される
    let pool = init_db("sqlite::memory:").await.unwrap();
    Box::new(RepositoryProvider::new(pool))
}

#[tokio::test]
async fn test_content_crud_operations() -> Result<(), BoxError> {
    // Arrange
    let provider = setup().await;
    let mut uow = provider.begin().await?;

    // Act: Create
    let new_content = ContentEntity {
        id: 0, // DB側で自動採番されるので、この値は無視される
        title: "Test Title".to_string(),
        body: "Test Body".to_string(),
    };
    let created = uow.content().create(&new_content).await?;
    assert_eq!(created.id, 1);
    assert_eq!(created.title, "Test Title");

    // Act: Select
    let found = uow.content().select(created.id).await?.unwrap();
    assert_eq!(found.id, created.id);

    // Act: Update
    let to_update = ContentEntity {
        id: found.id,
        title: "Updated Title".to_string(),
        body: "Updated Body".to_string(),
    };
    let updated = uow.content().update(&to_update).await?.unwrap();
    assert_eq!(updated.title, "Updated Title");

    // Act: Delete
    let rows_affected = uow.content().delete(updated.id).await?;
    assert_eq!(rows_affected, 1);
    let should_be_none = uow.content().select(updated.id).await?;
    assert!(should_be_none.is_none());

    // 全ての操作を確定
    uow.commit().await?;
    Ok(())
}

#[tokio::test]
async fn test_tag_crud_operations() -> Result<(), BoxError> {
    // Arrange
    let provider = setup().await;
    let mut uow = provider.begin().await?;

    // Act: Create
    let new_tag = TagEntity {
        id: 0,
        label: "rust".to_string(),
    };
    let created = uow.tag().create(&new_tag).await?;
    assert_eq!(created.id, 1);
    assert_eq!(created.label, "rust");

    // Act: Select
    let found = uow.tag().select(created.id).await?.unwrap();
    assert_eq!(found.id, created.id);

    uow.commit().await?;
    Ok(())
}

#[tokio::test]
async fn test_uow_commit_and_rollback() -> Result<(), BoxError> {
    // Arrange
    let provider = setup().await;

    // Act: トランザクション内で複数の操作を行い、コミットする
    {
        let mut uow = provider.begin().await?;
        let content = uow
            .content()
            .create(&ContentEntity {
                id: 0,
                title: "Commit Test".into(),
                body: "...".into(),
            })
            .await?;
        let tag = uow
            .tag()
            .create(&TagEntity {
                id: 0,
                label: "transaction".into(),
            })
            .await?;
        uow.content_tag()
            .create(&ContentTagEntity {
                content_id: content.id,
                tag_id: tag.id,
            })
            .await?;
        uow.commit().await?;
    }

    {
        // Assert: 新しいトランザクションでデータが永続化されていることを確認
        let mut uow2 = provider.begin().await?;
        let content = uow2.content().select(1).await?;
        assert!(content.is_some(), "コミットされたデータが見つかるべき");
    }

    // Act: 別の操作をロールバック（暗黙的ロールバック）
    {
        let mut uow3 = provider.begin().await?;
        uow3.content()
            .create(&ContentEntity {
                id: 0,
                title: "Rollback Test".into(),
                body: "...".into(),
            })
            .await?;
        // ここでuow3はスコープを抜けるが、commitされていないため自動的にロールバックされる
    }

    {
        // Assert: 新しいトランザクションでデータが永続化されていないことを確認
        let mut uow4 = provider.begin().await?;
        let content = uow4.content().select(2).await?;
        assert!(
            content.is_none(),
            "ロールバックされたデータは見つからないはず"
        );
    }
    Ok(())
}
