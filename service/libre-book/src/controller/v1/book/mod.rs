use crate::{schema, models, s3};
use actix_web::{web, HttpResponse};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use libre_core::database::postgres::PostgresPool;
use libre_core::jsonwebtoken::{init, validator_no_data};

mod list;
mod file;
mod upload;
mod download;

#[inline]
pub fn service_config(cfg: &mut web::ServiceConfig) {
    let middleware =
        actix_web_httpauth::middleware::HttpAuthentication::bearer(validator_no_data);

    cfg.service(
        web::scope("/books")
            .route("/details/{book_id}", web::get().to(get_book_details))
            .route("/list", web::get().to(list::list))
            // .service(
            //     web::resource("/upload")
            //         .app_data(init::decoder())
            //         .app_data(s3::s3_client())
            //         .wrap(middleware)
            //         .route(web::post().to(upload)),
            // )
            // 
    );
}

pub async fn get_book_details(
    db_pool: web::Data<PostgresPool>,
    book_id: web::Path<u32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().await?;

    let book = schema::books::dsl::books
        .find(*book_id as i32)
        .select(models::Book::as_select())
        .first(&mut conn)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound(""))?;

    Ok(HttpResponse::Ok().json(book))
}
