use actix_web::{web, App, HttpServer};
use libre_core::{
    database::{self},
    jsonwebtoken,
};
use libre_user::{controller, oauth};

/// 根据本地/集群环境加载不同的环境变量
pub fn load_env() {
    // 检查环境变量中的 KUBERNETES_SERVICE 标志位
    let is_kubernetes =
        std::env::var("KUBERNETES_SERVICE").unwrap_or_else(|_| "false".to_string()) == "true";

    // 如果不在 Kubernetes 集群中，则加载 .env 文件，否则默认使用 ConfigMap 和 Secret 注入的环境变量
    if !is_kubernetes {
        if dotenv::dotenv().is_err() {
            println!("Failed to read .env file");
        } else {
            println!(".env file loaded successfully");
        }
    }

    // 设置日志级别
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();

    let pool = database::postgres::init_postgres_pool().await;
    let postgres_app_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(postgres_app_data.clone())
            .configure(database::redis::init)
            .configure(jsonwebtoken::init_encoder)
            .configure(oauth::init)
            .configure(controller::init_routes)
    })
    .bind(("0.0.0.0", 8085))?
    .run()
    .await
}
