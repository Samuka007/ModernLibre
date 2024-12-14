use crate::{models, schema};
use actix_web::{web, HttpResponse};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use libre_core::database::postgres::PostgresPool;
use libre_core::jsonwebtoken::validator_no_data;

mod download;
mod list;
mod upload;

#[inline]
pub fn service_config(cfg: &mut web::ServiceConfig) {
    let mw = actix_web_httpauth::middleware::HttpAuthentication::bearer(validator_no_data);

    cfg.service(
        web::scope("/books")
            .service(get_book_details)
            .service(list::list)
            .route("/download", web::get().to(download::oss_temp_credential).wrap(mw.clone()))
            .route("/upload", web::post().to(upload::upload).wrap(mw))
    );
}

#[actix_web::get("/details/{book_id}")]
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
