use crate::{handlers, state::AppState};
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn create_router(state: AppState) -> Router {
    // コンテンツ関連のエンドポイントを定義するサブルーター
    let content_router = Router::new()
        .route("/", post(handlers::content::create))
        .route("/edit", post(handlers::content::edit));
    // .route("/remove/{id}", get(handlers::content::remove));

    let tag_router = Router::new()
        .route("/id/{id}", delete(handlers::tag::remove))
        .route("/label/{label}", delete(handlers::tag::remove_by_label));

    Router::new()
        // `/contents`というプレフィックスでサブルーターをネストする
        .nest("/contents", content_router)
        .nest("/tags", tag_router)
        .with_state(state)
}
