use crate::impliment::{
    content::ContentRepository, content_tag::ContentTagRepository, tag::TagRepository,
};
use async_trait::async_trait;
use common::types::{BoxError, DbPool};
use domain::{
    interface::{content::ContentInterface, content_tag::ContentTagInterface, tag::TagInterface},
    repository_provider::RepositoryProviderInterface,
    unit_of_work::UnitOfWorkInterface,
};
use sqlx::{Transaction, sqlite::Sqlite};
use std::ops::DerefMut;

// RepositoryProviderはコネクションプールを保持し、UnitOfWorkのファクトリとして機能します。
// アプリケーションの生存期間中、共有されるオブジェクトです。
#[derive(Clone)]
pub struct RepositoryProvider {
    pool: DbPool,
}

impl RepositoryProvider {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepositoryProviderInterface for RepositoryProvider {
    // 新しいUnit of Work（トランザクション）を開始します。
    // これがユースケースの起点となります。
    // 戻り値をトレイトオブジェクトにすることで、インフラ層の実装を隠蔽します。
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkInterface + '_>, BoxError> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(UnitOfWork { tx }))
    }
}

// UnitOfWorkは単一のトランザクションを表します。
// リクエストの処理中に生成され、処理が終わると破棄されます。
pub struct UnitOfWork<'a> {
    tx: Transaction<'a, Sqlite>,
}

#[async_trait]
impl<'a> UnitOfWorkInterface for UnitOfWork<'a> {
    // トランザクションをコミットします。
    // これを呼び出すとUnitOfWorkは消費され、中のトランザクションが確定します。
    async fn commit(self: Box<Self>) -> Result<(), BoxError> {
        self.tx.commit().await?;
        Ok(())
    }

    // トランザクションをロールバックします。
    // commitされずにUnitOfWorkが破棄（drop）された場合、自動的にロールバックされます。
    // 明示的に呼び出すことも可能です。
    async fn rollback(self: Box<Self>) -> Result<(), BoxError> {
        self.tx.rollback().await?;
        Ok(())
    }

    // ContentRepositoryへのアクセスを提供します。
    // `impliment`モジュールで定義された具体的なリポジトリ実装を返します。
    fn content<'s>(&'s mut self) -> Box<dyn ContentInterface + 's> {
        Box::new(ContentRepository::new(self.tx.deref_mut()))
    }

    // TagRepositoryへのアクセスを提供します。
    fn tag<'s>(&'s mut self) -> Box<dyn TagInterface + 's> {
        Box::new(TagRepository::new(self.tx.deref_mut()))
    }

    // TagContentRepositoryへのアクセスを提供します。
    fn content_tag<'s>(&'s mut self) -> Box<dyn ContentTagInterface + 's> {
        Box::new(ContentTagRepository::new(self.tx.deref_mut()))
    }
}
