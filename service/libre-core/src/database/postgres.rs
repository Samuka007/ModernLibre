use actix_web::web;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use crate::environment::POSTGRES_URL;

pub struct PostgresPool(pub Pool<AsyncPgConnection>);

impl PostgresPool {
    pub async fn get(&self) -> Result<PooledConnection<'_, AsyncPgConnection>, actix_web::Error> {
        self.0
            .get()
            .await
            .map_err(actix_web::error::ErrorInternalServerError)
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.app_data(init_postgres_pool());
}

pub fn init_postgres_pool() -> PostgresPool {
    // create a new connection pool with the default config
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var(POSTGRES_URL).expect(&format!("{POSTGRES_URL} must be set")),
    );
    let pool = actix_web::rt::System::new()
        .block_on(Pool::builder().build(config))
        .expect("Failed to create pool"); // Enhancement: IO error handling
    PostgresPool(pool)
}
