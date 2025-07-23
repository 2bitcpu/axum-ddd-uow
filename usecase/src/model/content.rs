use domain::model::content::ContentEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateContentRequestDto {
    pub title: String,
    pub body: String,
}

impl CreateContentRequestDto {
    pub fn to_entity(&self) -> ContentEntity {
        ContentEntity {
            id: 0,
            title: self.title.clone(),
            body: self.body.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateContentResponseDto {
    pub id: i64,
    pub title: String,
    pub body: String,
}

impl CreateContentResponseDto {
    pub fn from_entity(entity: ContentEntity) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            body: entity.body,
        }
    }
}
