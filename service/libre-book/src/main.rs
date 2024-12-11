use actix_web::{web::Data, App, HttpServer};
use libre_book::{routes::init_routes, s3, util};
use util::load_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    let postgres_pool = libre_core::database::postgres::init_postgres_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive()) // TODO: 使用环境变量配置DisableCors
            .app_data(Data::new(s3::StorageClient::new_from_env()))
            .app_data(Data::new(postgres_pool.clone()))
            .configure(init_routes)
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}
