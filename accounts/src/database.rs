use async_std::task;
use lazy_static::lazy_static;
use sqlx::PgPool;

use crate::config::CONFIG;

lazy_static! {
    pub static ref DATABASE_POOL: PgPool = {
        async fn connect_to_database() -> PgPool {
            PgPool::connect(&CONFIG.database_url).await.unwrap()
        }

        task::block_on(connect_to_database())
    };
}
