use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};

pub async fn remove(State(state): State<AppState>, Path(id): Path<i64>) -> StatusCode {
    state.modules.tag.remove(id).await.unwrap();
    StatusCode::NO_CONTENT
}

pub async fn remove_by_label(
    State(state): State<AppState>,
    Path(label): Path<String>,
) -> StatusCode {
    state.modules.tag.remove_label(label).await.unwrap();
    StatusCode::NO_CONTENT
}

// pub fn create_tag_handler() -> Router<AppState> {
//     Router::new()
//         .route("/id/{id}", delete(remove))
//         .route("/label/{label}", delete(remove_by_label));
// }
