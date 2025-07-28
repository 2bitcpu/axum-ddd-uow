use domain::model::{content::ContentEntity, tag::TagEntity};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateContentRequestDto {
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
}

impl CreateContentRequestDto {
    pub fn to_content(&self) -> ContentEntity {
        ContentEntity {
            id: 0,
            title: self.title.clone(),
            body: self.body.clone(),
        }
    }

    pub fn to_tags(&self) -> Vec<TagEntity> {
        self.labels
            .iter()
            .map(|label| TagEntity {
                id: 0,
                label: label.clone(),
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTagResponseDto {
    pub id: i64,
    pub label: String,
}

impl CreateTagResponseDto {
    pub fn from_entity(tag: TagEntity) -> Self {
        Self {
            id: tag.id,
            label: tag.label,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateContentResponseDto {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub tags: Vec<CreateTagResponseDto>,
}

impl CreateContentResponseDto {
    pub fn from_entity(content: ContentEntity, tags: Vec<TagEntity>) -> Self {
        Self {
            id: content.id,
            title: content.title,
            body: content.body,
            tags: tags
                .into_iter()
                .map(CreateTagResponseDto::from_entity)
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditContentRequestDto {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
}

impl EditContentRequestDto {
    pub fn to_content(&self) -> ContentEntity {
        ContentEntity {
            id: self.id,
            title: self.title.clone(),
            body: self.body.clone(),
        }
    }

    pub fn to_tags(&self) -> Vec<TagEntity> {
        self.labels
            .iter()
            .map(|label| TagEntity {
                id: 0,
                label: label.clone(),
            })
            .collect()
    }
}
