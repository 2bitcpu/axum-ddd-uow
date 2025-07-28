use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use usecase::model::content::{
    CreateContentRequestDto, CreateContentResponseDto, EditContentRequestDto,
};

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateContentRequestDto>,
) -> (StatusCode, Json<CreateContentResponseDto>) {
    let content = state.modules.content.create(payload).await.unwrap();
    (StatusCode::CREATED, Json(content))
}

pub async fn edit(
    State(state): State<AppState>,
    Json(payload): Json<EditContentRequestDto>,
) -> (StatusCode, Json<CreateContentResponseDto>) {
    let content = state.modules.content.edit(payload).await.unwrap();
    (StatusCode::OK, Json(content))
}

pub async fn remove(State(state): State<AppState>, Path(id): Path<i64>) -> StatusCode {
    state.modules.content.remove(id).await.unwrap();
    StatusCode::NO_CONTENT
}
