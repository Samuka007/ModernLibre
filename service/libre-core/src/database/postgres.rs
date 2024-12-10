use actix_web::web;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use crate::environment::{POSTGRES_MAX_CONN, POSTGRES_URL};

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

pub async fn init_postgres_pool() -> PostgresPool {
    // create a new connection pool with the default config
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var(POSTGRES_URL).expect(&format!("{POSTGRES_URL} must be set")),
    );
    let max_conn = std::env::var(POSTGRES_MAX_CONN)
        .map(|s| s.parse().expect("Failed to parse max connection count"))
        .unwrap_or(10);
    let pool = Pool::builder()
        .max_size(max_conn)
        .build(config)
        .await
        .expect("Failed to create pool"); // Enhancement: IO error handling
    PostgresPool(pool)
}
