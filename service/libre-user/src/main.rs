use actix_web::{App, HttpServer};
use libre_user::{controller, oauth};
use libre_core::{database, jsonwebtoken};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .configure(database::postgres::init)
        .configure(database::redis::init)
        .configure(jsonwebtoken::init_encoder)
        .configure(oauth::init)
        .configure(controller::init_routes)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
