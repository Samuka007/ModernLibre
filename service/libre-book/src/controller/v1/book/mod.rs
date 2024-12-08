use crate::schema;
use actix_web::{web, HttpResponse};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use libre_core::database::postgres::PostgresPool;

mod list;

#[inline]
pub fn service_config(cfg: &mut web::ServiceConfig) {
    let middleware =
        actix_web_httpauth::middleware::HttpAuthentication::bearer(
            libre_core::jsonwebtoken::validator
        );

    cfg.service(
        web::scope("/books")
            .route("/details/{book_id}", web::get().to(get_book_details))
            .route("/list", web::get().to(list::list))
            .service(
                web::resource("/upload")
                    .wrap(middleware)
                    .route(web::post().to(upload_book_info)),
            ),
    );
}

pub async fn get_book_details(
    db_pool: web::Data<PostgresPool>,
    book_id: web::Path<u32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = db_pool.get().await?;

    let book = schema::book::dsl::book
        .find(*book_id as i32)
        .select(schema::Book::as_select())
        .first(&mut conn)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound(""))?;

    Ok(HttpResponse::Ok().json(book))
}

// 接收File Service 传回的书本信息
pub async fn upload_book_info(
    db_pool: web::Data<PostgresPool>,
    book_: web::Json<schema::Book>,
) -> Result<HttpResponse, actix_web::Error> {
    //log::debug!("get book info: {:?}", book);
    let book = book_.into_inner();
    // 将book信息插入数据库
    let mut conn = db_pool.get().await?;

    diesel::insert_into(schema::book::table)
        .values(&book)
        .execute(&mut conn)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError(""))?;

    Ok(HttpResponse::Ok().finish())
}
