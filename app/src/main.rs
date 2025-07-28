use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use common::{setup::init_db, types::BoxError};
use infrastructure::repositories::RepositoryProvider;
use std::sync::Arc;
use usecase::{
    logic::{content::ContentUseCases, tag::TagUseCases},
    model::content::{CreateContentRequestDto, CreateContentResponseDto, EditContentRequestDto},
};

// --- state.rsの内容 ---
#[derive(Clone)]
pub struct Modules {
    pub content: ContentUseCases,
    pub tag: TagUseCases,
}

impl Modules {
    pub fn new(
        provider: Arc<dyn domain::repository_provider::RepositoryProviderInterface + Send + Sync>,
    ) -> Self {
        Self {
            content: ContentUseCases::new(provider.clone()),
            tag: TagUseCases::new(provider),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub modules: Modules,
}

// --- handlers/content.rsの内容 ---
pub async fn create_content(
    State(state): State<AppState>,
    Json(payload): Json<CreateContentRequestDto>,
) -> (StatusCode, Json<CreateContentResponseDto>) {
    let content = state.modules.content.create(payload).await.unwrap();
    (StatusCode::CREATED, Json(content))
}

pub async fn edit_content(
    State(state): State<AppState>,
    Json(payload): Json<EditContentRequestDto>,
) -> (StatusCode, Json<CreateContentResponseDto>) {
    let content = state.modules.content.edit(payload).await.unwrap();
    (StatusCode::OK, Json(content))
}

pub async fn remove_content(State(state): State<AppState>, Path(id): Path<i64>) -> StatusCode {
    state.modules.content.remove(id).await.unwrap();
    StatusCode::NO_CONTENT
}

// --- handlers/tag.rsの内容 ---
pub async fn remove_tag(State(state): State<AppState>, Path(id): Path<i64>) -> StatusCode {
    state.modules.tag.remove(id).await.unwrap();
    StatusCode::NO_CONTENT
}

pub async fn remove_tag_by_label(
    State(state): State<AppState>,
    Path(label): Path<String>,
) -> StatusCode {
    state.modules.tag.remove_label(label).await.unwrap();
    StatusCode::NO_CONTENT
}

// --- Health Check Handler ---
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// --- router.rsの内容 ---
pub fn create_router(state: AppState) -> Router {
    // ご要望のRPC形式のエンドポイントを定義します
    let service_router = Router::new()
        // Content endpoints
        .route("/content/create", post(create_content))
        .route("/content/edit", post(edit_content))
        .route("/content/remove/{id}", get(remove_content))
        // Tag endpoints
        .route("/tag/remove/{id}", delete(remove_tag))
        .route("/tag/remove_label/{label}", delete(remove_tag_by_label))
        // Health check endpoint
        .route("/health-check", get(health_check));

    Router::new()
        .nest("/service", service_router)
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let pool = init_db("sqlite::memory:").await?;
    let provider = Arc::new(RepositoryProvider::new(pool));
    let modules = Modules::new(provider);
    let state = AppState { modules };
    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
