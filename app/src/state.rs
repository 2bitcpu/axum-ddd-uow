use infrastructure::repositories::RepositoryProvider;
use std::sync::Arc;
use usecase::logic::{content::ContentUseCases, tag::TagUseCases};

/// 全てのユースケースをまとめるコンテナ構造体
/// これにより、依存関係が整理され、AppStateがクリーンに保たれます。
#[derive(Clone)]
pub struct Modules {
    pub content: ContentUseCases,
    pub tag: TagUseCases,
}

impl Modules {
    pub fn new(provider: Arc<RepositoryProvider>) -> Self {
        // ここで具体的な型からトレイトオブジェクトへの変換が行われる
        let provider: Arc<
            dyn domain::repository_provider::RepositoryProviderInterface + Send + Sync,
        > = provider;
        Self {
            content: ContentUseCases::new(provider.clone()),
            tag: TagUseCases::new(provider),
        }
    }
}

/// アプリケーション全体の状態。全てのハンドラで共有されます。
#[derive(Clone)]
pub struct AppState {
    pub modules: Modules,
}
