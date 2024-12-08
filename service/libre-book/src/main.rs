use actix_web::{web::Data, App, HttpServer};
use libre_book::{routes::init_routes, util};
use util::load_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive()) // TODO: 使用环境变量配置DisableCors
            .configure(libre_core::database::postgres::init) // 将连接池传递给App
            .configure(init_routes)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
