use crate::model::content::{
    CreateContentRequestDto, CreateContentResponseDto, EditContentRequestDto,
};
use common::types::BoxError;
use derive_new::new;
use domain::{
    model::{content_tag::ContentTagEntity, tag::TagEntity},
    repository_provider::RepositoryProviderInterface,
};
use std::sync::Arc;

#[derive(new, Clone)]
pub struct ContentUseCases<R: RepositoryProviderInterface> {
    provider: Arc<R>,
}

impl<R: RepositoryProviderInterface> ContentUseCases<R> {
    pub async fn create(
        &self,
        dto: CreateContentRequestDto,
    ) -> Result<CreateContentResponseDto, BoxError> {
        let mut uow = self.provider.begin().await?;

        let mut tags: Vec<TagEntity> = Vec::new();
        for tag in dto.to_tags() {
            let entity = uow.tag().find_by_label(&tag.label).await?;
            if let Some(entity) = entity {
                tags.push(entity);
            } else {
                let entity = uow.tag().create(&tag).await?;
                tags.push(entity);
            }
        }

        let contet = uow.content().create(&dto.to_content()).await?;

        for tag in &tags {
            let content_tag = uow.content_tag().select(contet.id, tag.id).await?;
            if content_tag.is_none() {
                uow.content_tag()
                    .create(&ContentTagEntity {
                        content_id: contet.id,
                        tag_id: tag.id,
                    })
                    .await?;
            }
        }
        uow.commit().await?;
        Ok(CreateContentResponseDto::from_entity(contet, tags))
    }

    pub async fn edit(
        &self,
        dto: EditContentRequestDto,
    ) -> Result<CreateContentResponseDto, BoxError> {
        let mut uow = self.provider.begin().await?;

        let _ = uow.content_tag().delete_by_content_id(dto.id).await?;

        let mut tags: Vec<TagEntity> = Vec::new();
        for tag in dto.to_tags() {
            let entity = uow.tag().find_by_label(&tag.label).await?;
            if let Some(entity) = entity {
                tags.push(entity);
            } else {
                let entity = uow.tag().create(&tag).await?;
                tags.push(entity);
            }
        }

        let contet = uow
            .content()
            .update(&dto.to_content())
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        for tag in &tags {
            let content_tag = uow.content_tag().select(contet.id, tag.id).await?;
            if content_tag.is_none() {
                uow.content_tag()
                    .create(&ContentTagEntity {
                        content_id: contet.id,
                        tag_id: tag.id,
                    })
                    .await?;
            }
        }
        uow.commit().await?;
        Ok(CreateContentResponseDto::from_entity(contet, tags))
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
