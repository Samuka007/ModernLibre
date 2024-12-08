use actix_web::web;
use redis::aio::MultiplexedConnection;

use crate::environment::REDIS_URL;

pub struct RedisMultiplexClient(pub redis::Client);

impl RedisMultiplexClient {
    pub async fn get(&self) -> Result<MultiplexedConnection, actix_web::Error> {
        self.0
            .get_multiplexed_async_connection()
            .await
            .map_err(actix_web::error::ErrorInternalServerError)
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.app_data(init_redis_pool());
}

pub fn init_redis_pool() -> RedisMultiplexClient {
    let client = redis::Client::open(
        std::env::var(REDIS_URL).expect(&format!("{REDIS_URL} must be set"))
    ).expect("Failed to create redis client"); // Enhancement: IO error handling
    RedisMultiplexClient(client)
}
