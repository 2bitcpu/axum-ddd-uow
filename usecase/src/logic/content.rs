use crate::model::content::{
    CreateContentRequestDto, CreateContentResponseDto, EditContentRequestDto,
};
use common::types::BoxError;
use domain::{
    interface::tag::TagInterface,
    model::{content::ContentEntity, content_tag::ContentTagEntity, tag::TagEntity},
    repository_provider::RepositoryProviderInterface,
    unit_of_work::UnitOfWorkInterface,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ContentUseCases {
    provider: Arc<dyn RepositoryProviderInterface + Send + Sync>,
}

impl ContentUseCases {
    pub fn new(provider: Arc<dyn RepositoryProviderInterface + Send + Sync>) -> Self {
        Self { provider }
    }

    /// ラベル文字列のリストから、既存のタグを検索するか、新しいタグを作成する
    async fn find_or_create_tags(
        &self,
        tag_repo: &mut Box<dyn TagInterface + '_>,
        tag_entities: Vec<TagEntity>,
    ) -> Result<Vec<TagEntity>, BoxError> {
        let mut tags: Vec<TagEntity> = Vec::new();
        for tag in tag_entities {
            let entity = tag_repo.find_by_label(&tag.label).await?;
            if let Some(entity) = entity {
                tags.push(entity);
            } else {
                let entity = tag_repo.create(&tag).await?;
                tags.push(entity);
            }
        }
        Ok(tags)
    }

    /// 複数のタグを永続化し、コンテンツとの関連を記録する
    async fn link_tags_to_content(
        &self,
        uow: &mut Box<dyn UnitOfWorkInterface + '_>,
        content: &ContentEntity,
        tags: &[TagEntity],
    ) -> Result<(), BoxError> {
        for tag in tags {
            let content_tag = uow.content_tag().select(content.id, tag.id).await?;
            if content_tag.is_none() {
                uow.content_tag()
                    .create(&ContentTagEntity {
                        content_id: content.id,
                        tag_id: tag.id,
                    })
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn create(
        &self,
        dto: CreateContentRequestDto,
    ) -> Result<CreateContentResponseDto, BoxError> {
        let mut uow = self.provider.begin().await?;

        let tags = self
            .find_or_create_tags(&mut uow.tag(), dto.to_tags())
            .await?;

        let content = uow.content().create(&dto.to_content()).await?;

        self.link_tags_to_content(&mut uow, &content, &tags).await?;
        uow.commit().await?;
        Ok(CreateContentResponseDto::from_entity(content, tags))
    }

    pub async fn edit(
        &self,
        dto: EditContentRequestDto,
    ) -> Result<CreateContentResponseDto, BoxError> {
        let mut uow = self.provider.begin().await?;

        let _ = uow.content_tag().delete_by_content_id(dto.id).await?;

        let tags = self
            .find_or_create_tags(&mut uow.tag(), dto.to_tags())
            .await?;

        let content = uow
            .content()
            .update(&dto.to_content())
            .await?
            .ok_or_else(|| {
                let msg = "Content not found";
                let e: BoxError = Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, msg));
                e
            })?;

        self.link_tags_to_content(&mut uow, &content, &tags).await?;

        uow.commit().await?;
        Ok(CreateContentResponseDto::from_entity(content, tags))
    }

    pub async fn remove(&self, id: i64) -> Result<u64, BoxError> {
        let mut uow = self.provider.begin().await?;
        let count = uow.content().delete(id).await?;
        // 削除された行があった場合のみ、関連するタグも削除する
        if count > 0 {
            uow.content_tag().delete_by_content_id(id).await?;
        }
        uow.commit().await?;
        Ok(count)
    }
}
